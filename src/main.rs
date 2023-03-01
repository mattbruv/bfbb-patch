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

    let bin_target = fs::read(args.target)?;
    let bin_source = fs::read(args.source)?;

    let target_elf = object::File::parse(&*bin_target)?;
    let source_elf = object::File::parse(&*bin_source)?;

    let source_obj = read_obj(&source_elf);

    /*
       for sec in source_elf.sections() {
           println!("{:?}", sec);
           for rel in sec.relocations() {
               println!("{:?}", rel);
           }
       }
    */

    write_obj(&source_obj, args.out);

    Ok(())
}

// fn parse_object(elf: &object::read::File) -> ParsedObject {}

fn bytes_equal(vec1: &Vec<u8>, vec2: &Vec<u8>) -> bool {
    if vec1.len() != vec2.len() {
        return false;
    }

    for i in 0..vec1.len() {
        if vec1[i] != vec2[i] {
            return false;
        }
    }

    true
}
