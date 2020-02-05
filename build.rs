use std::env;
use std::fs::File;
use std::io;
use std::io::Write;
use std::path::Path;

const MAX_SIZE: usize = 128;

fn main() -> io::Result<()> {
    println!("cargo:rerun-if-changed=build.rs");

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("macrocalls.rs");
    let mut f = File::create(dest_path)?;
    for i in 1..MAX_SIZE + 1 {
        write_macro_defs(&mut f, i)?;
        for k in 1..i {
            write_relation_defs(&mut f, i, k)?;
        }
        write!(f, "\n")?;
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
    write!(f, "define_prim_wrappers!(u{0}, {0});\n", targetsize)
}

fn write_newtype_defs(f: &mut File, targetsize: usize, containersize: usize) -> io::Result<()> {
    write!(
        f,
        "define_unsigned!(u{0}, u{1}, {0}, #[doc=\"Unsigned {0}-bit integer.\"]);\n",
        targetsize, containersize,
    )
}

fn write_relation_defs(f: &mut File, bigsize: usize, smallsize: usize) -> io::Result<()> {
    write!(
        f,
        "{}!(bigger: u{}, smaller: u{});\n",
        if is_prim_size(bigsize) && is_prim_size(smallsize) {
            "define_bitcontainerof_relation"
        } else {
            "define_newtype_relations"
        },
        bigsize,
        smallsize,
    )
}

fn is_prim_size(targetsize: usize) -> bool {
    container_size_for(targetsize) == targetsize
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
