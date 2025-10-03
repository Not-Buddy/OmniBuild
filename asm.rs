use std::path::Path;
use std::env;

pub fn compile_assembly_code() {
    if cfg!(target_os = "windows") {
        if Path::new("src/ASM_Code/bitwise_ops.asm").exists() {
            cc::Build::new()
                .file("src/ASM_Code/bitwise_ops.asm")
                .compile("bitwise_ops");
            println!("cargo:warning=✅ Windows ASM compiled");
        } else {
            println!("cargo:warning=⚠️ Windows ASM file not found");
        }
    } else if cfg!(target_arch = "x86_64") {
        if Path::new("src/ASM_Code/bitwise_ops.s").exists() {
            cc::Build::new()
                .file("src/ASM_Code/bitwise_ops.s")
                .compile("bitwise_ops");
            println!("cargo:warning=✅ Linux/Unix x86_64 ASM compiled");
        } else {
            println!("cargo:warning=⚠️ x86_64 ASM file not found");
        }
    } else if cfg!(target_arch = "aarch64") {
        if Path::new("src/ASM_Code/bitwise_ops_arm64.s").exists() {
            cc::Build::new()
                .file("src/ASM_Code/bitwise_ops_arm64.s")
                .compile("bitwise_ops_arm64");
            println!("cargo:warning=✅ ARM64 ASM compiled");
        } else {
            println!("cargo:warning=⚠️ ARM64 ASM file not found");
        }
    } else {
        let arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_else(|_| "unknown".to_string());
        println!("cargo:warning=⚠️ ASM not supported for: {}", arch);
    }
}
