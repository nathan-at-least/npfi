use std::env;
use std::fs::File;
use std::io;
use std::path::Path;

fn main() -> io::Result<()> {
    println!("cargo:rerun-if-changed=build.rs");

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("macrocalls.rs");
    let f = File::open(dest_path);
    for i in 1..128 {
        write_macro_calls(&f, i)?;
    }
}

fn write_macro_calls(f: &File, usersize: usize) -> io::Result<()> {
    let containersize = container_size_for(usersize);

    f.write(&format!(
        "define_unsigned!(u{0}, u{1}, {0}, \"Unsigned {0}-bit integer.\");\n",
        usersize, containersize,
    ));
}
