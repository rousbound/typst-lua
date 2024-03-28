mod args;
mod compile;
mod download;
mod fonts;
mod init;
mod package;
mod query;
mod terminal;
mod timings;
#[cfg(feature = "self-update")]
mod update;
mod watch;
mod world;

use std::cell::Cell;
use std::io::{self, Write};
use std::process::ExitCode;

use clap::Parser;
use codespan_reporting::term::termcolor::WriteColor;
use codespan_reporting::term::{self, termcolor};
use once_cell::sync::Lazy;

use ecow::eco_format;

use crate::args::{CliArguments};

use typst::eval::Tracer;
use crate::args::SharedArgs;
use crate::world::SystemWorld;
use typst::foundations::Value;
use typst::foundations::Smart;
use typst::foundations::Capturer;

use typst::diag::{Severity, SourceDiagnostic, StrResult};
use typst::syntax::{FileId, Span};
use typst::{World, WorldExt};
use codespan_reporting::diagnostic::{Diagnostic, Label};

thread_local! {
    /// The CLI's exit code.
    static EXIT: Cell<ExitCode> = Cell::new(ExitCode::SUCCESS);
}

static ARGS: Lazy<CliArguments> = Lazy::new(CliArguments::parse);
/// Used by `args.rs`.
fn typst_version() -> &'static str {
    env!("TYPST_VERSION")
}
fn set_failed() {
    EXIT.with(|cell| cell.set(ExitCode::FAILURE));
}

/// Print an application-level error (independent from a source file).
fn print_error(msg: &str) -> io::Result<()> {
    let styles = term::Styles::default();

    let mut output = terminal::out();
    output.set_color(&styles.header_error)?;
    write!(output, "error")?;

    output.reset()?;
    writeln!(output, ": {msg}")
}

impl SystemWorld {

    fn define(&mut self, label: &str, var: &Value) {
        self.library.update(|l|
            l.global
                .scope_mut()
                .define_captured(label, var.to_owned(), Capturer::Function)
        );
    }

}

fn label(world: &SystemWorld, span: Span) -> Option<Label<FileId>> {
    Some(Label::primary(span.id()?, world.range(span)?))
}

pub fn format_diagnostics(
    world: &SystemWorld,
    errors: &[SourceDiagnostic],
    warnings: &[SourceDiagnostic],
) -> Result<String, codespan_reporting::files::Error> {
    let mut w = termcolor::Buffer::no_color();

    let config = term::Config {
        tab_width: 2,
        ..Default::default()
    };

    for diagnostic in warnings.iter().chain(errors.iter()) {
        let diag = match diagnostic.severity {
            Severity::Error => Diagnostic::error(),
            Severity::Warning => Diagnostic::warning(),
        }
        .with_message(diagnostic.message.clone())
        .with_notes(
            diagnostic
                .hints
                .iter()
                .map(|e| (eco_format!("hint: {e}")).into())
                .collect(),
        )
        .with_labels(label(world, diagnostic.span).into_iter().collect());

        term::emit(&mut w, &config, world, &diag)?;

        // Stacktrace-like helper diagnostics.
        for point in &diagnostic.trace {
            let message = point.v.to_string();
            let help = Diagnostic::help()
                .with_message(message)
                .with_labels(label(world, point.span).into_iter().collect());

            term::emit(&mut w, &config, world, &help)?;
        }
    }

    let s = String::from_utf8(w.into_inner()).unwrap();
    Ok(s)
}


pub fn compile(
    input: &str,
    var: &Option<Value>
    ) -> StrResult<Vec<u8>> 
{
    let args = SharedArgs{
        input: args::Input::Path(input.into()),
        inputs: Vec::new(),
        root: None,
        font_paths: Vec::new(),
        diagnostic_format: args::DiagnosticFormat::Human
    };
    let mut world = world::SystemWorld::new(&args).unwrap();

    if let Some(var) = var {
        world.define("_LUADATA", var);
    }
    let mut tracer = Tracer::new();
    world.reset();
    world.source(world.main()).map_err(|err| err.to_string())?;
    
    match typst::compile(&world, &mut tracer) {
        Ok(document) => {
            let buffer = typst_pdf::pdf(&document, Smart::Auto, None);
            Ok(buffer)
        }

        Err(errors) => {
            let warnings = tracer.warnings();
            Err(format_diagnostics(&world, &errors, &warnings).unwrap().into())
        }
    }
}

