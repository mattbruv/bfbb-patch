use object::{self, Object, ObjectSection};
use std::{collections::HashSet, fs, hash::Hash};

fn main() -> std::io::Result<()> {
    let mut all_sections: HashSet<String> = HashSet::new();
    let mut files_with_section: Vec<String> = vec![];

    let search_section = "init";

    let paths = fs::read_dir("objs")?;
    for entry in paths {
        let path = entry?.path();
        let bin_data = fs::read(path.clone())?;
        let obj_file = object::File::parse(&*bin_data).unwrap();

        let filename = path.file_name().unwrap().to_str().unwrap().to_string();

        for section in obj_file.sections() {
            let name = section.name().unwrap().to_string();
            let count = section_count_by_name(&name, &obj_file);

            if count > 1 {
                println!("{} has {} {} sections", filename, count, name);
                all_sections.insert(name.clone());
            }

            if name.contains(search_section)
                && files_with_section.contains(&search_section.to_string()) == false
            {
                files_with_section.push(filename.clone());
            }

            //all_sections.insert(name);
        }
    }

    let mut secs: Vec<String> = all_sections.into_iter().collect();
    secs.sort();

    println!(
        "{} is found in {} files:",
        search_section,
        files_with_section.len()
    );

    for file in files_with_section {
        println!("{}", file);
    }

    for section in secs {
        println!("{}", section);
    }

    //

    Ok(())
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
