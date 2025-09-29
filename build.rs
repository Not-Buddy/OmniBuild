// build.rs
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
        
    // Compile Assembly code
    cc::Build::new()
        .file("src/ASM_Code/bitwise_ops.s")
        .compile("bitwise_ops");
        
    println!("cargo:rerun-if-changed=src/C_Code/");
    println!("cargo:rerun-if-changed=src/CPP_Code/");
    println!("cargo:rerun-if-changed=src/ASM_Code/");
}
