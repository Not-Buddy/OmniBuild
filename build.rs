mod javac;
mod gcc;
mod gpp;
mod asm;

use javac::{setup_java_environment,compile_java_files};
use gcc::compile_c_code;
use gpp::compile_cpp_code;
use asm::compile_assembly_code;

use std::env;
use std::fs;

fn main() {
    println!("🔨 OmniBuild Multi-Architecture Build System");

    // Load .env if it exists (manually without extra dependency)
    load_env_file();

    // Setup JAVA environment
    setup_java_environment();

    // Compile C/C++/Assembly/Java code
    compile_c_code();
    compile_cpp_code();
    compile_assembly_code();
    compile_java_files();

    // Rerun triggers
    println!("cargo:rerun-if-changed=src/C_Code/");
    println!("cargo:rerun-if-changed=src/CPP_Code/");
    println!("cargo:rerun-if-changed=src/ASM_Code/");
    println!("cargo:rerun-if-changed=src/JAVA_Code/");
    println!("cargo:rerun-if-changed=.env");
}

fn load_env_file() {
    if let Ok(contents) = fs::read_to_string(".env") {
        println!("cargo:warning=📄 .env file loaded");
        for line in contents.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            if let Some((key, value)) = line.split_once('=') {
                unsafe {
                    env::set_var(key.trim(), value.trim());
                }
                println!("cargo:warning=🔧 Set {} from .env", key.trim());
            }
        }
    } else {
        println!("cargo:warning=⚠️  No .env file found (optional)");
    }
}