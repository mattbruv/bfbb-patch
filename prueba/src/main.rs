use object::{self, read::elf::ElfFile, Object, ObjectSection, ObjectSymbol};
use std::{
    collections::HashSet,
    fs,
    fs::{metadata, DirEntry},
    hash::Hash,
};

fn main() -> std::io::Result<()> {
    let files = fs::read_dir("../asm/").unwrap();

    for f in files {
        if let Ok(entry) = f {
            // println!("{}", entry.path().to_str().unwrap());

            if !metadata(entry.path())?.is_file() {
                continue;
            }

            println!("{}", metadata(entry.path()).unwrap().file_type());

            let bin = fs::read(entry.path()).unwrap();
            let elf = object::File::parse(&*bin).unwrap();
            print_symbols(&elf);
        }
    }

    Ok(())
}

fn get_all_asm_objs() {
    //
    let paths = fs::read_dir("../asm").unwrap();
}

fn print_symbols(elf: &object::read::File) {
    for symbol in elf.symbols() {
        println!("{}", symbol.address());
    }
}

fn section_count_by_name(name: &str, elf: &object::read::File) -> i32 {
    let mut count = 0;

    for section in elf.sections() {
        if section.name().unwrap() == name {
            count += 1
        }
    }

    count
}
