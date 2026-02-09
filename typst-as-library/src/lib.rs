// Modified by rousbound (Geraldo Luiz) in 2026
// Changes: Added support for receiving external values from Lua through "_LUADATA" variable
// Original work Copyright 2026 tfachmann (Timo Bachmann)
// Licensed under the Apache License, Version 2.0
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use typst::diag::Tracepoint;

use codespan_reporting::diagnostic::{Diagnostic, Label};
use codespan_reporting::files::{Error as CodespanError, Files};
use codespan_reporting::term::termcolor::Buffer;
use codespan_reporting::term::{self};
use typst::diag::{
    eco_format, FileError, FileResult, PackageError, PackageResult, Severity, SourceDiagnostic,
    Warned,
};
use typst::foundations::{Bytes, Datetime, Dict, Value};
use typst::syntax::package::PackageSpec;
use typst::syntax::{FileId, Lines, Source, Span};
use typst::text::{Font, FontBook};
use typst::utils::LazyHash;
use typst::Library;
use typst::LibraryExt;
use typst::World;
use typst::WorldExt;
use typst_kit::fonts::{FontSearcher, FontSlot};
use typst_pdf::PdfOptions;

type CodespanResult<T> = Result<T, CodespanError>;

/// Main interface that determines the environment for Typst.
pub struct TypstWrapperWorld {
    /// Root path to which files will be resolved.
    root: PathBuf,

    /// The content of a source.
    source: Source,

    /// The standard library.
    library: LazyHash<Library>,

    /// Metadata about all known fonts.
    book: LazyHash<FontBook>,

    /// Metadata about all known fonts.
    fonts: Vec<FontSlot>,

    /// Map of all known files.
    files: Arc<Mutex<HashMap<FileId, FileEntry>>>,

    /// Cache directory (e.g. where packages are downloaded to).
    cache_directory: PathBuf,

    /// http agent to download packages.
    http: ureq::Agent,

    /// Datetime.
    time: time::OffsetDateTime,
}

impl TypstWrapperWorld {
    pub fn new(root: String, source: String, data: Option<Value>) -> Self {
        let root = PathBuf::from(root);
        let fonts = FontSearcher::new().include_system_fonts(true).search();
        let lib = {
            let builder = Library::builder();
            let builder = if let Some(d) = data {
                let dict: Dict = d.clone().cast::<Dict>().unwrap();
                builder.with_inputs(dict)
            } else {
                builder
            };
            builder.build()
        };

        Self {
            library: LazyHash::new(lib),
            book: LazyHash::new(fonts.book),
            root,
            fonts: fonts.fonts,
            source: Source::detached(source),
            time: time::OffsetDateTime::now_utc(),
            cache_directory: std::env::var_os("CACHE_DIRECTORY")
                .map(|os_path| os_path.into())
                .unwrap_or(std::env::temp_dir()),
            http: ureq::Agent::new(),
            files: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

/// A File that will be stored in the HashMap.
#[derive(Clone, Debug)]
struct FileEntry {
    bytes: Bytes,
    source: Option<Source>,
}

impl FileEntry {
    fn new(bytes: Vec<u8>, source: Option<Source>) -> Self {
        Self {
            bytes: Bytes::new(bytes),
            source,
        }
    }

    fn source(&mut self, id: FileId) -> FileResult<Source> {
        let source = if let Some(source) = &self.source {
            source
        } else {
            let contents = std::str::from_utf8(&self.bytes).map_err(|_| FileError::InvalidUtf8)?;
            let contents = contents.trim_start_matches('\u{feff}');
            let source = Source::new(id, contents.into());
            self.source.insert(source)
        };
        Ok(source.clone())
    }
}

impl TypstWrapperWorld {
    /// Helper to handle file requests.
    ///
    /// Requests will be either in packages or a local file.
    fn file(&self, id: FileId) -> FileResult<FileEntry> {
        let mut files = self.files.lock().map_err(|_| FileError::AccessDenied)?;
        if let Some(entry) = files.get(&id) {
            return Ok(entry.clone());
        }
        let path = if let Some(package) = id.package() {
            // Fetching file from package
            let package_dir = self.download_package(package)?;
            id.vpath().resolve(&package_dir)
        } else {
            // Fetching file from disk
            id.vpath().resolve(&self.root)
        }
        .ok_or(FileError::AccessDenied)?;

        let content = std::fs::read(&path).map_err(|error| FileError::from_io(error, &path))?;
        Ok(files
            .entry(id)
            .or_insert(FileEntry::new(content, None))
            .clone())
    }

    /// Downloads the package and returns the system path of the unpacked package.
    fn download_package(&self, package: &PackageSpec) -> PackageResult<PathBuf> {
        let package_subdir = format!("{}/{}/{}", package.namespace, package.name, package.version);
        let path = self.cache_directory.join(package_subdir);

        if path.exists() {
            return Ok(path);
        }

        eprintln!("downloading {package}");
        let url = format!(
            "https://packages.typst.org/{}/{}-{}.tar.gz",
            package.namespace, package.name, package.version,
        );

        let response = retry(|| {
            let response = self
                .http
                .get(&url)
                .call()
                .map_err(|error| eco_format!("{error}"))?;

            let status = response.status();
            // if !http_successful(status) {
            //     return Err(eco_format!(
            //         "response returned unsuccessful status code {status}",
            //     ));
            // }

            Ok(response)
        })
        .map_err(|error| PackageError::NetworkFailed(Some(error)))?;

        let mut compressed_archive = Vec::new();
        response
            .into_reader()
            .read_to_end(&mut compressed_archive)
            .map_err(|error| PackageError::NetworkFailed(Some(eco_format!("{error}"))))?;
        let raw_archive = zune_inflate::DeflateDecoder::new(&compressed_archive)
            .decode_gzip()
            .map_err(|error| PackageError::MalformedArchive(Some(eco_format!("{error}"))))?;
        let mut archive = tar::Archive::new(raw_archive.as_slice());
        archive.unpack(&path).map_err(|error| {
            _ = std::fs::remove_dir_all(&path);
            PackageError::MalformedArchive(Some(eco_format!("{error}")))
        })?;

        Ok(path)
    }

    fn lookup(&self, id: FileId) -> Lines<String> {
        if id == self.source.id() {
            self.source.lines().clone()
        } else {
            <Self as World>::source(self, id)
                .map(|source| source.lines().clone())
                .expect("file id does not point to a valid source")
        }
    }
}

/// This is the interface we have to implement such that `typst` can compile it.
///
/// I have tried to keep it as minimal as possible
impl typst::World for TypstWrapperWorld {
    /// Standard library.
    fn library(&self) -> &LazyHash<Library> {
        &self.library
    }

    /// Metadata about all known Books.
    fn book(&self) -> &LazyHash<FontBook> {
        &self.book
    }

    /// Accessing the main source file.
    fn main(&self) -> FileId {
        self.source.id()
    }

    /// Accessing a specified source file (based on `FileId`).
    fn source(&self, id: FileId) -> FileResult<Source> {
        if id == self.source.id() {
            Ok(self.source.clone())
        } else {
            self.file(id)?.source(id)
        }
    }

    /// Accessing a specified file (non-file).
    fn file(&self, id: FileId) -> FileResult<Bytes> {
        self.file(id).map(|file| file.bytes.clone())
    }

    /// Accessing a specified font per index of font book.
    fn font(&self, id: usize) -> Option<Font> {
        self.fonts[id].get()
    }

    /// Get the current date.
    ///
    /// Optionally, an offset in hours is given.
    fn today(&self, offset: Option<i64>) -> Option<Datetime> {
        let offset = offset.unwrap_or(0);
        let offset = time::UtcOffset::from_hms(offset.try_into().ok()?, 0, 0).ok()?;
        let time = self.time.checked_to_offset(offset)?;
        Some(Datetime::Date(time.date()))
    }
}

fn retry<T, E>(mut f: impl FnMut() -> Result<T, E>) -> Result<T, E> {
    if let Ok(ok) = f() {
        Ok(ok)
    } else {
        f()
    }
}

pub fn compile(input: &str, data: &Option<Value>) -> Result<Vec<u8>, String> {
    let input_path = Path::new(input);
    let root = input_path.parent().unwrap_or(Path::new("."));
    let content = fs::read_to_string(input)
        .map_err(|err| format!("failed to read source file `{input}`: {err}"))?;
    let spath = root.to_string_lossy().into_owned();
    let world = TypstWrapperWorld::new(spath, content, data.clone());
    let Warned { output, warnings } = typst::compile(&world);

    match output {
        Ok(document) => typst_pdf::pdf(&document, &PdfOptions::default())
            .map_err(|errors| render_diagnostics(&world, &errors, &warnings)),
        Err(errors) => Err(render_diagnostics(&world, &errors, &warnings)),
    }
}

fn render_diagnostics(
    world: &TypstWrapperWorld,
    errors: &[SourceDiagnostic],
    warnings: &[SourceDiagnostic],
) -> String {
    format_diagnostics(world, errors, warnings)
        .unwrap_or_else(|err| format!("failed to format diagnostics: {err}"))
}

fn format_diagnostics(
    world: &TypstWrapperWorld,
    errors: &[SourceDiagnostic],
    warnings: &[SourceDiagnostic],
) -> Result<String, String> {
    let config = term::Config {
        tab_width: 2,
        ..Default::default()
    };
    let mut buffer = Buffer::no_color();

    for diagnostic in warnings.iter().chain(errors) {
        let diag = match diagnostic.severity {
            Severity::Error => Diagnostic::error(),
            Severity::Warning => Diagnostic::warning(),
        }
        .with_message(diagnostic.message.clone())
        .with_notes(
            diagnostic
                .hints
                .iter()
                .map(|hint| (eco_format!("hint: {hint}")).into())
                .collect(),
        )
        .with_labels(label(world, diagnostic.span).into_iter().collect());

        term::emit(&mut buffer, &config, world, &diag).map_err(|err| err.to_string())?;

        for point in &diagnostic.trace {
            let message = match &point.v {
                Tracepoint::Call(Some(name)) => format!("in call to {}", name.as_str()),

                Tracepoint::Call(None) => "in call".to_string(),

                Tracepoint::Show(name) => format!("while showing {}", name.as_str()),

                Tracepoint::Import => "during import".to_string(),
            };

            let help = Diagnostic::help()
                .with_message(message)
                .with_labels(label(world, diagnostic.span).into_iter().collect());

            term::emit(&mut buffer, &config, world, &help).map_err(|err| err.to_string())?;
        }
    }

    String::from_utf8(buffer.into_inner()).map_err(|err| err.to_string())
}

fn label(world: &TypstWrapperWorld, span: Span) -> Option<Label<FileId>> {
    Some(Label::primary(span.id()?, world.range(span)?))
}

impl<'a> Files<'a> for TypstWrapperWorld {
    type FileId = FileId;
    type Name = String;
    type Source = Lines<String>;

    fn name(&'a self, id: FileId) -> CodespanResult<Self::Name> {
        let vpath = id.vpath();
        Ok(if let Some(package) = id.package() {
            format!("{package}{}", vpath.as_rooted_path().display())
        } else if let Some(path) = vpath.resolve(&self.root) {
            path.to_string_lossy().into()
        } else {
            vpath.as_rootless_path().to_string_lossy().into_owned()
        })
    }

    fn source(&'a self, id: FileId) -> CodespanResult<Self::Source> {
        Ok(self.lookup(id))
    }

    fn line_index(&'a self, id: FileId, given: usize) -> CodespanResult<usize> {
        let source = self.lookup(id);
        source
            .byte_to_line(given)
            .ok_or_else(|| CodespanError::IndexTooLarge {
                given,
                max: source.len_bytes(),
            })
    }

    fn line_range(&'a self, id: FileId, given: usize) -> CodespanResult<std::ops::Range<usize>> {
        let source = self.lookup(id);
        source
            .line_to_range(given)
            .ok_or_else(|| CodespanError::LineTooLarge {
                given,
                max: source.len_lines(),
            })
    }

    fn column_number(&'a self, id: FileId, _: usize, given: usize) -> CodespanResult<usize> {
        let source = self.lookup(id);
        source.byte_to_column(given).ok_or_else(|| {
            let max = source.len_bytes();
            if given <= max {
                CodespanError::InvalidCharBoundary { given }
            } else {
                CodespanError::IndexTooLarge { given, max }
            }
        })
    }
}
