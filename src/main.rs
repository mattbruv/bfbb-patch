use argh::FromArgs;
use object;
use std::{error::Error, fs};

#[derive(FromArgs)]
/// Creates a patched ELF object file. Patches code from source object into target object.
struct PatchArgs {
    /// source object file
    #[argh(positional)]
    source: String,

    /// target object file
    #[argh(positional)]
    target: String,

    /// out object file
    #[argh(positional, short = 'o')]
    out: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: PatchArgs = argh::from_env();
    let target_bin = fs::read(args.target)?;
    let target_elf = object::File::parse(&*target_bin)?;
    let source_bin = fs::read(args.source)?;
    let source_elf = object::File::parse(&*source_bin)?;

    //let patch_elf = object::write::
    let out_elf = object::write::Object::new(
        object::BinaryFormat::Elf,
        object::Architecture::PowerPc,
        object::Endianness::Little,
    );

    fs::write(args.out, out_elf.write().unwrap())?;

    Ok(())
}
