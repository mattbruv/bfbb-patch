use super::bfbbobj::BFBBSymbol;
use object::{write::SymbolSection, Object, ObjectSymbol, Section};

pub fn read_symbols(elf: &object::read::File) -> Vec<BFBBSymbol> {
    let symbols: Vec<BFBBSymbol> = elf.symbols().map(read_symbol).collect();
    symbols
}

fn read_symbol(symbol: object::Symbol) -> BFBBSymbol {
    let section = match symbol.section().index() {
        Some(x) => Some(x.0),
        _ => None,
    };
    let flags = symbol.flags();
    BFBBSymbol {
        //name: symbol.name().unwrap().as_bytes().to_vec(),
        name: String::from(symbol.name().unwrap()),
        value: 0,
        size: symbol.size(),
        kind: symbol.kind(),
        scope: symbol.scope(),
        weak: symbol.is_weak(),
        section: section,
        flags: flags,
    }
}
