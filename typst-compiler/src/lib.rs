use std::cell::{RefCell, RefMut};
use std::collections::HashMap;
use std::fs::{self, File};
use std::hash::Hash;
use std::path::{Path, PathBuf};

use codespan_reporting::diagnostic::{Diagnostic, Label};
use codespan_reporting::term::{self, termcolor};
use comemo::Prehashed;
use elsa::FrozenVec;
use memmap2::Mmap;
use once_cell::unsync::OnceCell;
use same_file::{Handle};
use siphasher::sip128::{Hasher128, SipHasher13};
use typst::diag::{FileError, FileResult, SourceError, StrResult};
use typst::eval::{Library, Value};
use typst::font::{Font, FontBook, FontInfo};
use typst::syntax::{Source, SourceId};
use typst::util::{Buffer, PathExt};
use typst_library::prelude::*;
use typst::World;
use walkdir::WalkDir;

use typst_library::prelude::EcoString;


/// A world that provides access to the operating system.
struct SystemWorld {
    root: PathBuf,
    library: Option<Prehashed<Library>>,
    book: Prehashed<FontBook>,
    fonts: Vec<FontSlot>,
    hashes: RefCell<HashMap<PathBuf, FileResult<PathHash>>>,
    paths: RefCell<HashMap<PathHash, PathSlot>>,
    sources: FrozenVec<Box<Source>>,
    main: SourceId,
}

impl SystemWorld {
    fn new(root: PathBuf, font_paths: &[PathBuf]) -> Self {
        let mut searcher = FontSearcher::new();
        searcher.search(font_paths);

        let library = typst_library::build();

        Self {
            root,
            library: Some(Prehashed::new(library)),
            book: Prehashed::new(searcher.book),
            fonts: searcher.fonts,
            hashes: RefCell::default(),
            paths: RefCell::default(),
            sources: FrozenVec::new(),
            main: SourceId::detached(),
        }
    }

    fn define(&mut self, label: &str, var: &Value) {
        self.library.as_mut().unwrap().update(|l|
            l.global
                .scope_mut()
                .define_captured(label, var.to_owned())
        );
    }

    fn reset_lib(&mut self, vars: &Vec<(&str, Value)>) {
        self.library.as_mut().unwrap().update(|l|
            for var in vars {
                l.global
                    .scope_mut()
                    .define_captured(var.0, Value::None)
            }
        );
        
    }
}

impl World for SystemWorld {
    fn today(&self, _: Option<i64>) -> Option<typst::eval::Datetime>
    {
        todo!()
    }

    fn root(&self) -> &Path {
        &self.root
    }

    fn library(&self) -> &Prehashed<Library> {
        self.library.as_ref().unwrap()
    }

    fn main(&self) -> &Source {
        self.source(self.main)
    }

    #[tracing::instrument(skip_all)]
    fn resolve(&self, path: &Path) -> FileResult<SourceId> {
        self.slot(path)?
            .source
            .get_or_init(|| {
                let buf = read(path)?;
                let text = String::from_utf8(buf)?;
                Ok(self.insert(path, text))
            })
            .clone()
    }

    fn source(&self, id: SourceId) -> &Source {
        &self.sources[id.as_u16() as usize]
    }

    fn book(&self) -> &Prehashed<FontBook> {
        &self.book
    }

    fn font(&self, id: usize) -> Option<Font> {
        let slot = &self.fonts[id];
        slot.font
            .get_or_init(|| {
                let data = self.file(&slot.path).ok()?;
                Font::new(data, slot.index)
            })
            .clone()
    }

    fn file(&self, path: &Path) -> FileResult<Buffer> {
        self.slot(path)?
            .buffer
            .get_or_init(|| read(path).map(Buffer::from))
            .clone()
    }
}

impl SystemWorld {
    #[tracing::instrument(skip_all)]
    fn slot(&self, path: &Path) -> FileResult<RefMut<PathSlot>> {
        let mut hashes = self.hashes.borrow_mut();
        let hash = match hashes.get(path).cloned() {
            Some(hash) => hash,
            None => {
                let hash = PathHash::new(path);
                if let Ok(canon) = path.canonicalize() {
                    hashes.insert(canon.normalize(), hash.clone());
                }
                hashes.insert(path.into(), hash.clone());
                hash
            }
        }?;

        Ok(std::cell::RefMut::map(self.paths.borrow_mut(), |paths| {
            paths.entry(hash).or_default()
        }))
    }

    #[tracing::instrument(skip_all)]
    fn insert(&self, path: &Path, text: String) -> SourceId {
        let id = SourceId::from_u16(self.sources.len() as u16);
        let source = Source::new(id, path, text);
        self.sources.push(Box::new(source));
        id
    }

    #[tracing::instrument(skip_all)]
    fn reset(&mut self) {
        self.sources.as_mut().clear();
        self.hashes.borrow_mut().clear();
        self.paths.borrow_mut().clear();
    }
}

/// Print diagnostic messages to the terminal.
fn get_diagnostics(
    world: &SystemWorld,
    errors: Vec<SourceError>,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut buffer = termcolor::Buffer::no_color();

    let config = term::Config { tab_width: 2, ..Default::default() };

    for error in errors {
        let range = error.range(world);
        let diag = Diagnostic::error()
            .with_message(error.message)
            .with_labels(vec![Label::primary(error.span.source(), range)]);

        // Write to buffer
        term::emit(&mut buffer, &config, world, &diag)?;

        for point in error.trace {
            let message = point.v.to_string();
            let help = Diagnostic::help().with_message(message).with_labels(vec![
                Label::primary(
                    point.span.source(),
                    world.source(point.span.source()).range(point.span),
                ),
            ]);

            // Write to buffer
            term::emit(&mut buffer, &config, world, &help)?;
        }
    }

    // Convert the buffer into a String
    let output = String::from_utf8(buffer.into_inner())?;

    Ok(output)
}


fn convert_json(value: serde_json::Value) -> Value {
    match value {
        serde_json::Value::Null => Value::None,
        serde_json::Value::Bool(v) => v.into_value(),
        serde_json::Value::Number(v) => match v.as_i64() {
            Some(int) => int.into_value(),
            None => v.as_f64().unwrap_or(f64::NAN).into_value(),
        },
        serde_json::Value::String(v) => v.into_value(),
        serde_json::Value::Array(v) => {
            v.into_iter().map(convert_json).collect::<Array>().into_value()
        }
        serde_json::Value::Object(v) => v
            .into_iter()
            .map(|(key, value)| (key.into(), convert_json(value)))
            .collect::<Dict>()
            .into_value(),
    }
}

pub struct Compiler<'a> {
    world: SystemWorld,
    globals: Vec<(&'a str, Value)>
}

pub fn to_json(json_str: &str) -> Value {
    convert_json(serde_json::from_str(json_str).unwrap())
}

pub fn to_text(text: &str) -> Value {
   Value::Str(Str::from(text)) 
}

impl<'a> Compiler<'a> {

    pub fn new(root: PathBuf) -> Compiler<'a> {
        
        let world = SystemWorld::new(root, &[PathBuf::new()]);
        Compiler{world, globals: vec!()}

    }

    //pub fn define(&mut self, label: &'a str, value: Value) {
        //self.globals.push((label, value))
    //}

    pub fn compile(
        &mut self,
        input: PathBuf,
        var: &Option<Value>
        ) -> StrResult<Vec<u8>> 
    {

        if let Some(var) = var {
            self.world.define("_DATA", var);
        }
        self.world.reset();
        self.world.main = self.world.resolve(&self.world.root.join(&input))?;
        let result = match typst::compile(&self.world) {
            // Export the PDF.
            Ok(document) => {
                let buffer = typst::export::pdf(&document);
                tracing::info!("Compilation succeeded");
                Ok(buffer)
            }

            // Print diagnostics.
            Err(errors) => {
                tracing::info!("Compilation failed");
                let diagnostic = get_diagnostics(&self.world, *errors).unwrap();
                tracing::info!("Compilation failed");
                Err(EcoString::from(format!("Error compiling!\n{}", diagnostic)))
            }
        };
        self.world.reset_lib(&self.globals);
        self.globals = vec!();
        result
    }
}

#[derive(Debug, Clone)]
pub struct FontsCommand {
    /// Also list style variants of each font family
    pub variants: bool,
}

type CodespanResult<T> = Result<T, CodespanError>;
type CodespanError = codespan_reporting::files::Error;


/// Holds details about the location of a font and lazily the font itself.
struct FontSlot {
    path: PathBuf,
    index: u32,
    font: OnceCell<Option<Font>>,
}

/// Holds canonical data for all paths pointing to the same entity.
#[derive(Default)]
struct PathSlot {
    source: OnceCell<FileResult<SourceId>>,
    buffer: OnceCell<FileResult<Buffer>>,
}



/// A hash that is the same for all paths pointing to the same entity.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct PathHash(u128);

impl PathHash {
    fn new(path: &Path) -> FileResult<Self> {
        let f = |e| FileError::from_io(e, path);
        let handle = Handle::from_path(path).map_err(f)?;
        let mut state = SipHasher13::new();
        handle.hash(&mut state);
        Ok(Self(state.finish128().as_u128()))
    }
}

/// Read a file.
#[tracing::instrument(skip_all)]
fn read(path: &Path) -> FileResult<Vec<u8>> {
    let f = |e| FileError::from_io(e, path);
    if fs::metadata(path).map_err(f)?.is_dir() {
        Err(FileError::IsDirectory)
    } else {
        fs::read(path).map_err(f)
    }
}

impl<'a> codespan_reporting::files::Files<'a> for SystemWorld {
    type FileId = SourceId;
    type Name = std::path::Display<'a>;
    type Source = &'a str;

    fn name(&'a self, id: SourceId) -> CodespanResult<Self::Name> {
        Ok(World::source(self, id).path().display())
    }

    fn source(&'a self, id: SourceId) -> CodespanResult<Self::Source> {
        Ok(World::source(self, id).text())
    }

    fn line_index(&'a self, id: SourceId, given: usize) -> CodespanResult<usize> {
        let source = World::source(self, id);
        source
            .byte_to_line(given)
            .ok_or_else(|| CodespanError::IndexTooLarge {
                given,
                max: source.len_bytes(),
            })
    }

    fn line_range(
        &'a self,
        id: SourceId,
        given: usize,
    ) -> CodespanResult<std::ops::Range<usize>> {
        let source = World::source(self, id);
        source
            .line_to_range(given)
            .ok_or_else(|| CodespanError::LineTooLarge { given, max: source.len_lines() })
    }

    fn column_number(
        &'a self,
        id: SourceId,
        _: usize,
        given: usize,
    ) -> CodespanResult<usize> {
        let source = World::source(self, id);
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

/// Searches for fonts.
struct FontSearcher {
    book: FontBook,
    fonts: Vec<FontSlot>,
}

impl FontSearcher {
    /// Create a new, empty system searcher.
    fn new() -> Self {
        Self { book: FontBook::new(), fonts: vec![] }
    }

    /// Search everything that is available.
    fn search(&mut self, font_paths: &[PathBuf]) {
        self.search_system();

        #[cfg(feature = "embed-fonts")]
        self.search_embedded();

        for path in font_paths {
            self.search_dir(path)
        }
    }

    /// Add fonts that are embedded in the binary.
    #[cfg(feature = "embed-fonts")]
    fn search_embedded(&mut self) {
        let mut search = |bytes: &'static [u8]| {
            let buffer = Buffer::from_static(bytes);
            for (i, font) in Font::iter(buffer).enumerate() {
                self.book.push(font.info().clone());
                self.fonts.push(FontSlot {
                    path: PathBuf::new(),
                    index: i as u32,
                    font: OnceCell::from(Some(font)),
                });
            }
        };

        // Embed default fonts.
        //search(include_bytes!("../../assets/fonts/LinLibertine_R.ttf"));
        //search(include_bytes!("../../assets/fonts/LinLibertine_RB.ttf"));
        //search(include_bytes!("../../assets/fonts/LinLibertine_RBI.ttf"));
        //search(include_bytes!("../../assets/fonts/LinLibertine_RI.ttf"));
        search(include_bytes!("../assets/fonts/NewCMMath-Book.otf"));
        search(include_bytes!("../assets/fonts/NewCMMath-Regular.otf"));
        search(include_bytes!("../assets/fonts/NewCM10-Regular.otf"));
        search(include_bytes!("../assets/fonts/NewCM10-Bold.otf"));
        search(include_bytes!("../assets/fonts/NewCM10-Italic.otf"));
        search(include_bytes!("../assets/fonts/NewCM10-BoldItalic.otf"));
        //search(include_bytes!("../../assets/fonts/DejaVuSansMono.ttf"));
        //search(include_bytes!("../../assets/fonts/DejaVuSansMono-Bold.ttf"));
        //search(include_bytes!("../../assets/fonts/DejaVuSansMono-Oblique.ttf"));
        //search(include_bytes!("../../assets/fonts/DejaVuSansMono-BoldOblique.ttf"));
    }

    /// Search for fonts in the linux system font directories.
    #[cfg(all(unix, not(target_os = "macos")))]
    fn search_system(&mut self) {
        self.search_dir("/usr/share/fonts");
        self.search_dir("/usr/local/share/fonts");

        if let Some(dir) = dirs::font_dir() {
            self.search_dir(dir);
        }
    }

    /// Search for fonts in the macOS system font directories.
    #[cfg(target_os = "macos")]
    fn search_system(&mut self) {
        self.search_dir("/Library/Fonts");
        self.search_dir("/Network/Library/Fonts");
        self.search_dir("/System/Library/Fonts");

        if let Some(dir) = dirs::font_dir() {
            self.search_dir(dir);
        }
    }

    /// Search for fonts in the Windows system font directories.
    #[cfg(windows)]
    fn search_system(&mut self) {
        let windir =
            std::env::var("WINDIR").unwrap_or_else(|_| "C:\\Windows".to_string());

        self.search_dir(Path::new(&windir).join("Fonts"));

        if let Some(roaming) = dirs::config_dir() {
            self.search_dir(roaming.join("Microsoft\\Windows\\Fonts"));
        }

        if let Some(local) = dirs::cache_dir() {
            self.search_dir(local.join("Microsoft\\Windows\\Fonts"));
        }
    }

    /// Search for all fonts in a directory recursively.
    fn search_dir(&mut self, path: impl AsRef<Path>) {
        for entry in WalkDir::new(path)
            .follow_links(true)
            .sort_by(|a, b| a.file_name().cmp(b.file_name()))
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            if matches!(
                path.extension().and_then(|s| s.to_str()),
                Some("ttf" | "otf" | "TTF" | "OTF" | "ttc" | "otc" | "TTC" | "OTC"),
            ) {
                self.search_file(path);
            }
        }
    }

    /// Index the fonts in the file at the given path.
    fn search_file(&mut self, path: impl AsRef<Path>) {
        let path = path.as_ref();
        if let Ok(file) = File::open(path) {
            if let Ok(mmap) = unsafe { Mmap::map(&file) } {
                for (i, info) in FontInfo::iter(&mmap).enumerate() {
                    self.book.push(info);
                    self.fonts.push(FontSlot {
                        path: path.into(),
                        index: i as u32,
                        font: OnceCell::new(),
                    });
                }
            }
        }
    }
}
