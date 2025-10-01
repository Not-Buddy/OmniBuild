fn main() {
    // Compile C code
    cc::Build::new()
        .file("src/C_Code/basic_math.c")
        .compile("basic_math");

    // Compile C++ code
    cc::Build::new()
        .file("src/CPP_Code/advanced_math.cpp")
        .cpp(true)
        .compile("advanced_math");

    // Detect OS and compile assembly accordingly
    if cfg!(target_os = "windows") {
        // Compile Windows-compatible assembly (.asm with MASM)
        cc::Build::new()
            .file("src/ASM_Code/bitwise_ops.asm")
            // depending on your assembler support, you may need to specify specific flags
            .compile("bitwise_ops");
    } else {
        // Compile Linux-compatible assembly (.s with NASM or GAS)
        cc::Build::new()
            .file("src/ASM_Code/bitwise_ops.s")
            .compile("bitwise_ops");
    }

    println!("cargo:rerun-if-changed=src/C_Code/");
    println!("cargo:rerun-if-changed=src/CPP_Code/");
    println!("cargo:rerun-if-changed=src/ASM_Code/");
}
