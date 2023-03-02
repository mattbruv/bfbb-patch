use std::{collections::HashMap, fs, hash::Hash};

use object::{
    write::{SectionId, Symbol, SymbolSection},
    SectionKind, SymbolKind,
};

use super::bfbbobj::{BFBBObj, BFBBSymbol};

pub fn write_obj(obj: &BFBBObj, path: String) {
    let mut out_elf = object::write::Object::new(
        object::BinaryFormat::Elf,
        object::Architecture::PowerPc,
        object::Endianness::Big,
    );

    let mut section_map: HashMap<String, SectionId> = HashMap::new();

    // add each section to the new object file
    for section in &obj.sections {
        let id = out_elf.add_section(vec![], section.name.as_bytes().to_vec(), section.kind);
        let out_section = out_elf.section_mut(id);

        section_map.insert(section.name.clone(), id);

        out_section.flags = section.flags;

        // add the data for this particular section
        match section.kind {
            SectionKind::Text => {
                let mut section_data: Vec<u8> = vec![];

                // get all symbols which belong to this section
                let bytes = obj
                    .symbols
                    .iter()
                    .filter(|s| s.section_name == section.name)
                    .map(|s| &s.data.bytes);

                // unify all symbol data belonging to this section
                for data_ref in bytes {
                    let mut data = data_ref.clone();
                    section_data.append(&mut data);
                }

                out_section.append_data(&section_data, 4);
            }
            _ => {
                println!("Passing section type: {:?}", section);
            }
        }
    }
    // Done writing sections

    // Write all symbols to symbol table
    for symbol in &obj.symbols {
        let symbol_section = match symbol.kind {
            SymbolKind::File => SymbolSection::None,
            SymbolKind::Section => {
                println!("FUCK {:?}", symbol);

                match section_map.get(&symbol.name) {
                    Some(sid) => SymbolSection::Section(*sid),
                    None => SymbolSection::None,
                }
            }
            _ => SymbolSection::Undefined,
        };

        println!("Adding symbol: {:?}", symbol);

        out_elf.add_symbol(object::write::Symbol {
            name: symbol.name.clone().into_bytes(),
            size: symbol.size,
            kind: symbol.kind,
            scope: symbol.scope,
            weak: symbol.weak,

            // not sure about these three
            section: symbol_section,
            value: 0,
            flags: object::SymbolFlags::None,
        });
    }

    // TODO: Add relocations

    fs::write(path, out_elf.write().unwrap()).unwrap();
}

/*
fn to_write_sym(symbol: &BFBBSymbol) -> object::write::Symbol {
    object::write::Symbol {
        name: symbol.name.into(),
        value: 0,
        size: symbol.size,
        kind: symbol.kind,
        scope: symbol.scope,
        weak: symbol.weak,
        section: symbol.section,
        flags: symbol.flags,
    }
}
*/
