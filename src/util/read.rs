use std::vec;

use super::bfbbobj::{BFBBData, BFBBObj, BFBBRelocation, BFBBSection, BFBBSymbol};
use object::{Object, ObjectSection, ObjectSymbol, RelocationTarget, SymbolKind};

pub fn read_obj(elf: &object::read::File) -> BFBBObj {
    BFBBObj {
        sections: read_sections(elf),
        symbols: read_symbols(elf),
    }
}

fn read_symbols(elf: &object::read::File) -> Vec<BFBBSymbol> {
    let symbols: Vec<BFBBSymbol> = elf.symbols().map(|s| read_symbol(&s, elf)).collect();
    symbols
}

fn read_sections(elf: &object::read::File) -> Vec<BFBBSection> {
    elf.sections()
        .map(|section| BFBBSection {
            name: section.name().unwrap().to_string(),
            kind: section.kind(),
            address: section.address(),
            size: section.size(),
            align: section.align(),
            index: section.index().0,
            flags: section.flags(),
        })
        .collect()
}

fn get_relocs_at_data(
    section_id: Option<usize>,
    start: u64,
    end: u64,
    elf: &object::read::File,
) -> Vec<BFBBRelocation> {
    match section_id {
        Some(id) => {
            let section = elf.section_by_index(object::SectionIndex(id)).unwrap();

            section
                .relocations()
                .filter(|(addr, _)| *addr >= start && *addr <= end)
                .map(|(addr, rel)| BFBBRelocation {
                    address: addr,
                    relative_address: addr - start,
                    offset: 0,
                    size: rel.size(),
                    kind: rel.kind(),
                    encoding: rel.encoding(),
                    symbol_name: match rel.target() {
                        RelocationTarget::Symbol(idx) => {
                            symbol_name(&elf.symbol_by_index(idx).unwrap())
                        }
                        _ => panic!("Unhandled relocation target"),
                    },
                    addend: rel.addend(),
                })
                .collect()
        }
        _ => vec![],
    }
}

fn get_symbol_data(symbol: &object::Symbol, elf: &object::read::File) -> Vec<u8> {
    match symbol.kind() {
        SymbolKind::Data | SymbolKind::Text => {
            let data = elf
                .section_by_index(symbol.section_index().unwrap())
                .unwrap()
                .data()
                .unwrap();

            match data {
                // If empty data section, these symbols are uninitialized and have no data
                [] => vec![],
                _ => {
                    let start = symbol.address() as usize;
                    let size = symbol.size() as usize;
                    let end = start + size;
                    data[start..end].to_vec()
                }
            }
        }
        _ => vec![],
    }
}

fn symbol_name(symbol: &object::Symbol) -> String {
    String::from(symbol.name().unwrap())
}

fn read_symbol(symbol: &object::Symbol, elf: &object::read::File) -> BFBBSymbol {
    let section = match symbol.section().index() {
        Some(x) => Some(x.0),
        _ => None,
    };

    let section_name = match section {
        Some(index) => String::from(
            elf.section_by_index(object::SectionIndex(index))
                .unwrap()
                .name()
                .unwrap(),
        ),
        _ => String::new(),
    };

    let flags = symbol.flags();
    let offset = symbol.address();
    let size = symbol.size();
    let end = offset + size;
    BFBBSymbol {
        index: symbol.index(),
        name: symbol_name(symbol),
        size: symbol.size(),
        kind: symbol.kind(),
        scope: symbol.scope(),
        weak: symbol.is_weak(),
        section: section,
        section_name: section_name,
        section_offset: offset,
        flags: flags,
        data: BFBBData {
            bytes: get_symbol_data(symbol, elf),
            relocs: get_relocs_at_data(section, offset, end, elf),
        },
    }
}
