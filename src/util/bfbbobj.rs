/*
pub struct Symbol {
    pub name: Vec<u8>,
    pub value: u64,
    pub size: u64,
    pub kind: SymbolKind,
    pub scope: Symbolcope,
    pub weak: bool,
    pub section: SymbolSection,
    pub flags: SymbolFlags<SectionId>,
}
 */

use object::{write::SectionId, SymbolFlags, SymbolKind, SymbolScope};

#[derive(Debug)]
pub struct BFBBSymbol {
    pub name: String,
    pub value: u64,
    pub size: u64,
    pub kind: SymbolKind,
    pub scope: SymbolScope,
    pub weak: bool,
    pub section: Option<usize>,
    pub flags: Option<SymbolFlags<SectionId>>,
}
