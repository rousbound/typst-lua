use std::cell::{RefCell, RefMut};
use std::collections::HashMap;
use std::fs::{self, File};
use std::hash::Hash;
use std::path::{Path, PathBuf};


mod world;
mod fonts;
mod package;

use codespan_reporting::diagnostic::{Diagnostic, Label};
use codespan_reporting::term::{self, termcolor};
use comemo::Prehashed;
use elsa::FrozenVec;
use memmap2::Mmap;
use once_cell::unsync::OnceCell;
use same_file::{Handle};
use siphasher::sip128::{Hasher128, SipHasher13};
use typst::diag::{FileError, FileResult, StrResult};
use typst::eval::{Library, Value};
use typst::font::{Font, FontBook, FontInfo};
use typst::syntax::{Source};
use typst_library::prelude::*;
use typst::World;
use walkdir::WalkDir;

use typst_library::prelude::EcoString;
use crate::world::SystemWorld;
use typst::eval::Tracer;

/// Which format to use for diagnostics.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum DiagnosticFormat {
    Human,
    Short,
}

#[derive(Debug, Clone)]
pub struct SharedArgs {
    pub input: PathBuf,

    pub root: Option<PathBuf>,

    pub font_paths: Vec<PathBuf>,

    pub diagnostic_format: DiagnosticFormat,
}


impl SystemWorld {

    fn define(&mut self, label: &str, var: &Value) {
        self.library.update(|l|
            l.global
                .scope_mut()
                .define_captured(label, var.to_owned())
        );
    }

}


pub fn compile(
    input: &str,
    var: &Option<Value>
    ) -> StrResult<Vec<u8>> 
{
    let args = SharedArgs{
        input: PathBuf::from(input),
        root: None,
        font_paths: Vec::new(),
        diagnostic_format: DiagnosticFormat::Human
    };
    let mut world = world::SystemWorld::new(&args).unwrap();

    if let Some(var) = var {
        world.define("_LUADATA", var);
    }
    let mut tracer = Tracer::new();
    world.reset();
    world.source(world.main()).map_err(|err| err.to_string())?;
    let result = match typst::compile(&world, &mut tracer) {
        // Export the PDF.
        Ok(document) => {
            let buffer = typst::export::pdf(&document);
            tracing::info!("Compilation succeeded");
            Ok(buffer)
        }

        // Print diagnostics.
        Err(errors) => {
            tracing::info!("Compilation failed");
            Err(eco_format!("Error compiling!\n{:?}", errors))
        }
    };
    result
}

