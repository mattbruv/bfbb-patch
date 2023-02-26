use std::fs;

use super::bfbbobj::BFBBObj;

pub fn write_obj(obj: &BFBBObj, path: String) {
    let out_elf = object::write::Object::new(
        object::BinaryFormat::Elf,
        object::Architecture::PowerPc,
        object::Endianness::Big,
    );

    fs::write(path, out_elf.write().unwrap()).unwrap();
}
