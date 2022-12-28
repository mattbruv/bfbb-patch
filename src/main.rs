use argh::FromArgs;
use object::{
    self, read::elf::Sym, File, Object, ObjectSection, ObjectSymbol, Relocation, RelocationTarget,
    SectionKind, Symbol, SymbolKind,
};
use std::{collections::binary_heap::Iter, error::Error, fs, io::Read, vec};

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

enum ParsedSymbolFlag {
    Global,
    Local,
    Weak,
    Common,
    Hidden,
}

enum ParsedSymbolKind {
    Unknown,
    Function,
    Object,
    Section,
}

struct ParsedSymbol {
    pub name: String,
    pub address: u64,
    pub section: Option<usize>,
    pub size: u64,
    pub flags: ParsedSymbolFlag,
    pub kind: ParsedSymbolKind,
}

enum ParsedSectionKind {
    Code,
    Data,
    ReadOnlyData,
    Bss,
}

enum ParsedRelocKind {
    Absolute,
    PpcAddr16Hi,
    PpcAddr16Ha,
    PpcAddr16Lo,
    PpcRel24,
    PpcRel14,
    PpcEmbSda21,
}

struct ParsedReloc {
    pub kind: ParsedRelocKind,
    pub address: u64,
    pub target_symbol: usize,
    pub addend: i64,
}

struct ParsedSection {
    pub name: String,
    pub kind: ParsedSectionKind,
    pub address: u64,
    pub size: u64,
    pub data: Vec<u8>,
    pub align: u64,
    pub index: usize,
    pub relocations: Vec<ParsedReloc>,
    pub original_address: u64,
    pub file_offset: u64,
}

struct ParsedFunction {
    pub name: String,
    pub size: u64,
    pub bytes: Vec<u8>,
}

struct ParsedObject {
    pub symbols: Vec<ParsedSymbol>,
    pub sections: Vec<ParsedSection>,
    pub functions: Vec<ParsedFunction>,
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

    Ok(())
}

fn parse_object(elf: &object::read::File) -> ParsedObject {
    ParsedObject {
        symbols: vec![],
        sections: vec![],
        functions: vec![],
    }
}

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
