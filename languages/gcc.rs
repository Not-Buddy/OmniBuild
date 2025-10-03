use std::path::Path;

pub fn compile_c_code() {
    if Path::new("src/C_Code/basic_math.c").exists() {
        cc::Build::new()
            .file("src/C_Code/basic_math.c")
            .compile("basic_math");
        println!("cargo:warning=✅ C compilation successful");
    } else {
        println!("cargo:warning=⚠️ C source file not found");
    }
}
