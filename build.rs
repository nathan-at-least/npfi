use std::env;
use std::fs::File;
use std::io;
use std::path::Path;

fn main() -> io::Result<()> {
    println!("cargo:rerun-if-changed=build.rs");

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("macrocalls.rs");
    let mut f = File::create(dest_path)?;
    for i in 1..128 {
        write_macro_calls(&mut f, i)?;
    }

    Ok(())
}

fn write_macro_calls(f: &mut File, usersize: usize) -> io::Result<()> {
    use std::io::Write;

    let containersize = container_size_for(usersize);

    f.write_all(
        &format!(
            "define_unsigned!(u{0}, u{1}, {0}, #[doc=\"Unsigned {0}-bit integer.\"]);\n",
            usersize, containersize,
        )
        .as_bytes(),
    )
}

fn container_size_for(_usersize: usize) -> usize {
    128 // FIXME: choose the smallest primitive type that contains usersize.
}
