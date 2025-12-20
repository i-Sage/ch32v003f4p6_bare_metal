use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    
    // Copy the linker script to the output directory
    // The linker needs to be able to find this file
    let mut file = File::create(out_dir.join("memory.x")).unwrap();
    file.write_all(include_bytes!("memory.x")).unwrap();
    
    println!("cargo:rustc-link-search={}", out_dir.display());
    println!("cargo:rerun-if-changed=memory.x");
    
    // Assemble the startup code
    // The cc crate knows how to invoke the assembler for our target
    cc::Build::new()
        .file("src/startup.s")
        .flag("-march=rv32imac")     // Match our target architecture
        .flag("-mabi=ilp32")          // Match our ABI (integer, long, pointer = 32-bit)
        .compile("startup");
    
    println!("cargo:rerun-if-changed=src/startup.s");
}