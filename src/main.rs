use argh::FromArgs;
use object::{
    self, read::elf::Sym, File, Object, ObjectSection, ObjectSymbol, Relocation, SectionKind,
    Symbol, SymbolKind,
};
use std::{collections::binary_heap::Iter, error::Error, fs, io::Read};

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

    let replace_funcs: Vec<Symbol> = source_funcs
        .into_iter()
        .filter(|func| {
            target_funcs
                .iter()
                .any(|f| f.name().unwrap() == func.name().unwrap())
        })
        .collect();

    for sym in replace_funcs {
        let name = sym.name().unwrap();
        let bytes_target = get_function_code(name, &target_elf);
        let bytes_source = get_function_code(name, &source_elf);
    }

    fs::write(args.out, out_elf.write().unwrap())?;

    Ok(())
}

fn bytes_equal(vec1: &Vec<u8>, vec2: &Vec<u8>) -> bool {
    if !vec1.len() != vec2.len() {
        return false;
    }

    for i in 0..vec1.len() {
        if vec1[i] != vec2[i] {
            return false;
        }
    }

    true
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
