use argh::FromArgs;
use object::{self, read::elf::Sym, File, Object, ObjectSection, ObjectSymbol, Symbol, SymbolKind};
use std::{collections::binary_heap::Iter, error::Error, fs};

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

    let target_funcs: Vec<Symbol> = target_elf
        .symbols()
        .filter(|sym| sym.kind() == SymbolKind::Text)
        .collect();

    let source_funcs: Vec<Symbol> = source_elf
        .symbols()
        .filter(|sym| sym.kind() == SymbolKind::Text)
        .collect();

    dbg!(source_funcs.iter().count());

    for func in source_funcs {
        println!("{} {}", func.name().unwrap(), func.size());
    }

    fs::write(args.out, out_elf.write().unwrap())?;

    Ok(())
}
