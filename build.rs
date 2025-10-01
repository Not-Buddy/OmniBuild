fn main() {
    cc::Build::new()
        .file("src/C_Code/basic_math.c")
        .compile("basic_math");

    cc::Build::new()
        .file("src/CPP_Code/advanced_math.cpp")
        .cpp(true)
        .compile("advanced_math");

    if cfg!(target_os = "windows") {
        cc::Build::new()
            .file("src/ASM_Code/bitwise_ops.asm")
            .compile("bitwise_ops");
    } else if cfg!(target_arch = "x86_64") {
        cc::Build::new()
            .file("src/ASM_Code/bitwise_ops.s")
            .compile("bitwise_ops");
    } else if cfg!(target_arch = "aarch64") {
        cc::Build::new()
            .file("src/ASM_Code/bitwise_ops_arm64.s")  // Your ARM64 asm file
            .compile("bitwise_ops_arm64");
    } else {
        // optionally fallback for other architectures
    }

    println!("cargo:rerun-if-changed=src/C_Code/");
    println!("cargo:rerun-if-changed=src/CPP_Code/");
    println!("cargo:rerun-if-changed=src/ASM_Code/");
}

