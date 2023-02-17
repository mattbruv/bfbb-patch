pub mod util;

use argh::FromArgs;
use object::{
    self, Object, ObjectSection, ObjectSymbol, Relocation, RelocationTarget, SectionKind, Symbol,
    SymbolKind,
};
use std::{error::Error, fs, vec};

use crate::util::read::read_symbols;

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

    let mut syms = read_symbols(&target_elf);
    syms.sort_by_key(|x| x.name.clone());

    println!("Parsed symbols: ");
    for sym in syms {
        println!("{:?}", sym);
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

fn dump_sections(name: &str, elf: &object::read::File) {
    println!("Dumping...");
    for section in elf.sections() {
        let name = section.name().unwrap();
        let data = section.data().unwrap();
        println!("{}", name);
        println!("{:02X?}", data);
        println!("");
    }
    //
}

// Takes a function symbol + elf and returns the bytes of that function
fn get_function_code(func_name: &str, elf: &object::read::File) -> Vec<u8> {
    let sym = elf
        .symbols()
        .find(|s| s.name().unwrap() == func_name)
        .unwrap();

    let index = sym.section_index().unwrap();
    let begin = sym.address() as usize;
    let end = begin + sym.size() as usize;
    let section = elf.section_by_index(index).unwrap();
    let data = section.data().unwrap();
    let bytes: Vec<u8> = data[begin..end].into();

    bytes
}
