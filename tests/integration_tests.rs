use std::fs;
use std::process::Command;

#[test]
fn can_mirror_object() {
    fs::remove_file("mirror.o").unwrap_or(());

    Command::new("cargo")
        .args([
            "run",
            "objs/asm/Game/zActionLine.o",
            "objs/asm/Game/zActionLine.o",
            "mirror.o",
        ])
        .output()
        .expect("Failed to run process");

    //    let path =
    //    let bin_target = fs::read(path).expect(path);
    //    let obj = object::File::parse(&*bin_target).expect(path)
}
