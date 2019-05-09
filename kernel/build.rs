extern crate cc;

use std::fs::File;
use std::io::{Result, Write};
use std::path::Path;

fn main() {
    println!("cargo:rerun-if-env-changed=LOG");
    println!("cargo:rerun-if-env-changed=BOARD");
    println!("cargo:rerun-if-env-changed=SFSIMG");

    let arch: String = std::env::var("ARCH").unwrap();
    let board: String = std::env::var("BOARD").unwrap();
    match arch.as_str() {
        "x86_64" => {
            gen_vector_asm().unwrap();
        }
        "riscv32" => {}
        "riscv64" => {
            if board == "rocket_chip" {
                gen_dtb_asm(&String::from("riscv32"), &board).unwrap();
            }
        }
        "mipsel" => {
            gen_dtb_asm(&arch, &board).unwrap();
        }
        "aarch64" => {}
        _ => panic!("Unknown arch {}", arch),
    }
}

fn gen_vector_asm() -> Result<()> {
    let mut f = File::create("src/arch/x86_64/interrupt/vector.asm").unwrap();

    writeln!(f, "# generated by build.rs - do not edit")?;
    writeln!(f, ".section .text")?;
    writeln!(f, ".intel_syntax noprefix")?;
    for i in 0..256 {
        writeln!(f, "vector{}:", i)?;
        if !(i == 8 || (i >= 10 && i <= 14) || i == 17) {
            writeln!(f, "\tpush 0")?;
        }
        writeln!(f, "\tpush {}", i)?;
        writeln!(f, "\tjmp __alltraps")?;
    }

    writeln!(f, "\n.section .rodata")?;
    writeln!(f, ".global __vectors")?;
    writeln!(f, "__vectors:")?;
    for i in 0..256 {
        writeln!(f, "\t.quad vector{}", i)?;
    }
    Ok(())
}

fn gen_dtb_asm(arch: &String, _board: &String) -> Result<()> {
    let dtb = std::env::var("DTB").unwrap();

    if !Path::new(&dtb).is_file() {
        panic!("DTB `{}` not found", dtb)
    }

    let mut f = File::create(format!("src/arch/{}/boot/dtb.gen.s", arch)).unwrap();

    println!("cargo:rerun-if-changed={}", dtb);
    println!("cargo:rerun-if-env-changed=DTB");

    writeln!(f, "# generated by build.rs - do not edit")?;
    write!(
        f,
        r#"
    .section .dtb,"a"
    .align 12
    .global _dtb_start, _dtb_end
_dtb_start:
    .incbin "{}"
_dtb_end:
    "#,
        dtb
    )?;

    Ok(())
}
