use std::fs;

use object::{write::Symbol, SectionKind};

use super::bfbbobj::{BFBBObj, BFBBSymbol};

pub fn write_obj(obj: &BFBBObj, path: String) {
    let mut out_elf = object::write::Object::new(
        object::BinaryFormat::Elf,
        object::Architecture::PowerPc,
        object::Endianness::Big,
    );

    // add each section to the new object file
    for section in &obj.sections {
        let id = out_elf.add_section(vec![], section.name.as_bytes().to_vec(), section.kind);
        let out_section = out_elf.section_mut(id);
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

    //out_elf.add_symbol();

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
