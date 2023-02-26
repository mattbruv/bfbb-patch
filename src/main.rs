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

    let out_elf = object::write::Object::new(
        object::BinaryFormat::Elf,
        object::Architecture::PowerPc,
        object::Endianness::Big,
    );

    let target_obj = read_obj(&target_elf);

    let mut syms = target_obj.symbols;
    syms.sort_by_key(|x| x.index.0);

    let mut count = 1;
    for sym in &syms {
        if sym.data.relocs.len() > 0 {
            println!("{} starts at offset {}", sym.name, sym.section_offset);
            for rel in &sym.data.relocs {
                println!(
                    "reloc {} = addr: {}, rel: {}, {}, {:?}, {:?}",
                    count, rel.address, rel.relative_address, rel.symbol_name, rel.kind, rel.addend
                );
                count += 1;
            }
            println!("");
        }
    }

    println!("{} symbols processed", syms.len());

    println!("Sections:");
    for section in target_obj.sections {
        println!("{:?}", section);
    }

    //let s = target_elf.symbols().nth(0).unwrap();
    //out_elf.add_symbol(s);

    fs::write(args.out, out_elf.write().unwrap())?;

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
