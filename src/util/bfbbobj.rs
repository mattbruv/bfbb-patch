use object::{
    RelocationEncoding, RelocationKind, SectionFlags, SectionIndex, SectionKind, SymbolFlags,
    SymbolIndex, SymbolKind, SymbolScope,
};

#[derive(Debug)]
pub struct BFBBObj {
    pub sections: Vec<BFBBSection>,
    pub symbols: Vec<BFBBSymbol>,
}

#[derive(Debug)]
pub struct BFBBData {
    pub bytes: Vec<u8>,
    pub relocs: Vec<BFBBRelocation>,
}

#[derive(Debug)]
pub struct BFBBSymbol {
    pub index: SymbolIndex,
    pub name: String,
    pub size: u64,
    pub kind: SymbolKind,
    pub scope: SymbolScope,
    pub weak: bool,
    pub section: Option<usize>,
    pub section_name: String,
    pub section_offset: u64,
    pub flags: SymbolFlags<SectionIndex>,
    pub data: BFBBData,
}

#[derive(Debug)]
pub struct BFBBRelocation {
    // address relative to the start of the section in the object file
    pub address: u64,

    // address relative to the start of the function text
    pub relative_address: u64,

    pub offset: u64,
    pub size: u8,
    pub kind: RelocationKind,
    pub encoding: RelocationEncoding,
    pub symbol_name: String,
    pub addend: i64,
}

#[derive(Debug)]
pub struct BFBBSection {
    pub name: String,
    pub kind: SectionKind,
    pub address: u64,
    pub size: u64,
    // pub data: Vec<u8>,
    pub align: u64,
    pub index: usize,
    pub flags: SectionFlags,
}
