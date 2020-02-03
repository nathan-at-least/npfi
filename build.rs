use std::env;
use std::fs::File;
use std::io;
use std::io::Write;
use std::path::Path;

fn main() -> io::Result<()> {
    println!("cargo:rerun-if-changed=build.rs");

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("macrocalls.rs");
    let mut f = File::create(dest_path)?;
    for i in 1..128 {
        write_macro_defs(&mut f, i)?;
    }

    Ok(())
}

fn write_macro_defs(f: &mut File, targetsize: usize) -> io::Result<()> {
    let containersize = container_size_for(targetsize);
    if targetsize == containersize {
        write_prim_defs(f, containersize)
    } else {
        write_newtype_defs(f, targetsize, containersize)
    }
}

fn write_prim_defs(f: &mut File, targetsize: usize) -> io::Result<()> {
    f.write_all(&format!("define_prim_wrappers!(u{0}, {0});\n", targetsize).as_bytes())
}

fn write_newtype_defs(f: &mut File, targetsize: usize, containersize: usize) -> io::Result<()> {
    f.write_all(
        &format!(
            "define_unsigned!(u{0}, u{1}, {0}, #[doc=\"Unsigned {0}-bit integer.\"]);\n",
            targetsize, containersize,
        )
        .as_bytes(),
    )
}

fn container_size_for(targetsize: usize) -> usize {
    assert!(targetsize <= 128);

    for &cs in &[8usize, 16, 32, 64, 128] {
        if targetsize <= cs {
            return cs;
        }
    }

    unreachable!();
}
