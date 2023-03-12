pub mod util;

use argh::FromArgs;
use object::{
    self, Object, ObjectSection, ObjectSymbol, Relocation, RelocationTarget, SectionKind, Symbol,
    SymbolKind,
};
use std::{
    error::Error,
    fs::{self, read},
    vec,
};
use util::write::write_obj;

use crate::util::read::read_obj;

#[derive(FromArgs)]
/// Creates a patched ELF object file. Patches code from source object into target object.
struct PatchArgs {
    /// source object file
    #[argh(positional)]
    source: String,

    /// target object file
    #[argh(positional)]
    target: String,

    /// out object file
    #[argh(positional, short = 'o')]
    out: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: PatchArgs = argh::from_env();
    patch_object(args.target, args.source, args.out)
}

pub fn patch_object(target: String, source: String, out: String) -> Result<(), Box<dyn Error>> {
    let bin_target = fs::read(target)?;
    let bin_source = fs::read(source)?;

    let target_elf = object::File::parse(&*bin_target)?;
    let source_elf = object::File::parse(&*bin_source)?;

    let source_obj = read_obj(&source_elf);
    write_obj(&source_obj, out);

    Ok(())
}
