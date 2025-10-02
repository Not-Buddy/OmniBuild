// build.rs
use std::process::Command;
use std::path::Path;

fn main() {
    println!("🔨 OmniBuild Multi-Architecture Build System");
    
    // Compile C code (universal)
    compile_c_code();
    
    // Compile C++ code (universal)  
    compile_cpp_code();
    
    // Compile Assembly code (architecture-specific)
    compile_assembly_code();
    
    // Compile Java files (universal)
    compile_java_files();
    
    // Set up rerun triggers
    println!("cargo:rerun-if-changed=src/C_Code/");
    println!("cargo:rerun-if-changed=src/CPP_Code/");
    println!("cargo:rerun-if-changed=src/ASM_Code/");
    println!("cargo:rerun-if-changed=src/JAVA_Code/");
}

fn compile_c_code() {
    if Path::new("src/C_Code/basic_math.c").exists() {
        cc::Build::new()
            .file("src/C_Code/basic_math.c")
            .compile("basic_math");
        println!("cargo:warning=✅ C compilation successful");
    } else {
        println!("cargo:warning=⚠️  C source file not found");
    }
}

fn compile_cpp_code() {
    if Path::new("src/CPP_Code/advanced_math.cpp").exists() {
        cc::Build::new()
            .file("src/CPP_Code/advanced_math.cpp")
            .cpp(true)
            .flag_if_supported("-std=c++17")
            .compile("advanced_math");
        println!("cargo:warning=✅ C++ compilation successful");
    } else {
        println!("cargo:warning=⚠️  C++ source file not found");
    }
}

fn compile_assembly_code() {
    // Windows x64 Assembly
    if cfg!(target_os = "windows") && cfg!(target_arch = "x86_64") {
        if Path::new("src/ASM_Code/bitwise_ops.asm").exists() {
            cc::Build::new()
                .file("src/ASM_Code/bitwise_ops.asm")
                .compile("bitwise_ops");
            println!("cargo:warning=✅ Windows x64 Assembly compiled");
        } else {
            println!("cargo:warning=⚠️  Windows Assembly file not found");
        }
    }
    // Linux/Unix x86_64 Assembly
    else if cfg!(target_arch = "x86_64") {
        if Path::new("src/ASM_Code/bitwise_ops.s").exists() {
            cc::Build::new()
                .file("src/ASM_Code/bitwise_ops.s")
                .compile("bitwise_ops");
            println!("cargo:warning=✅ x86_64 Assembly compiled");
        } else {
            println!("cargo:warning=⚠️  x86_64 Assembly file not found");
        }
    }
    // ARM64 (AArch64) Assembly
    else if cfg!(target_arch = "aarch64") {
        if Path::new("src/ASM_Code/bitwise_ops_arm64.s").exists() {
            cc::Build::new()
                .file("src/ASM_Code/bitwise_ops_arm64.s")
                .compile("bitwise_ops_arm64");
            println!("cargo:warning=✅ ARM64 Assembly compiled");
        } else {
            println!("cargo:warning=⚠️  ARM64 Assembly file not found");
        }
    }
    // Other architectures
    else {
        let arch = std::env::var("CARGO_CFG_TARGET_ARCH")
            .unwrap_or_else(|_| "unknown".to_string());
        println!("cargo:warning=⚠️  Assembly not supported for: {}", arch);
    }
}

fn compile_java_files() {
    let java_files = [
        "src/JAVA_Code/Fibonacci.java",
        "src/JAVA_Code/PrimeChecker.java", 
        "src/JAVA_Code/ArrayStats.java",
        "src/JAVA_Code/StringOps.java",
    ];
    
    // Check which Java files exist
    let existing_files: Vec<&str> = java_files.iter()
        .filter(|&file| Path::new(file).exists())
        .copied()
        .collect();
    
    if existing_files.is_empty() {
        println!("cargo:warning=⚠️  No Java source files found");
        return;
    }
    
    // Check if javac is available and compile
    match Command::new("javac").arg("-version").output() {
        Ok(_) => {
            match Command::new("javac").args(&existing_files).output() {
                Ok(result) => {
                    if result.status.success() {
                        println!("cargo:warning=✅ Java compiled ({} files)", existing_files.len());
                    } else {
                        println!("cargo:warning=❌ Java compilation failed");
                    }
                }
                Err(_) => println!("cargo:warning=❌ Failed to run javac"),
            }
        }
        Err(_) => println!("cargo:warning=⚠️  javac not found - Java disabled"),
    }
}
