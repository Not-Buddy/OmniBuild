use std::path::Path;

pub fn compile_cpp_code() {
    if Path::new("src/CPP_Code/advanced_math.cpp").exists() {
        cc::Build::new()
            .file("src/CPP_Code/advanced_math.cpp")
            .cpp(true)
            .flag_if_supported("-std=c++17")
            .compile("advanced_math");
        println!("cargo:warning=✅ C++ compilation successful");
    } else {
        println!("cargo:warning=⚠️ C++ source file not found");
    }
}
