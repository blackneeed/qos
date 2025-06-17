use std::{process::Command, env};
use glob::glob;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    for entry in glob("src/**/*.asm").expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                let asm_path = path.to_str().unwrap();
                let file_stem = path.file_stem().unwrap().to_str().unwrap();

                let obj_path = format!("{}/{}.o", out_dir, file_stem);

                let status = Command::new("nasm")
                    .args(&["-felf32", asm_path, "-o", &obj_path])
                    .status()
                    .expect("nasm failed");

                if !status.success() {
                    panic!("nasm failed on {}", asm_path);
                }

                println!("cargo:rustc-link-search=native={}", out_dir);
                println!("cargo:rustc-link-arg={}", obj_path);
                println!("cargo:rerun-if-changed={}", asm_path);
            }
            Err(e) => eprintln!("{:?}", e),
        }
    }
}
