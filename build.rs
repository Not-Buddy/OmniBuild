use std::env;
use std::path::Path;
use std::process::Command;
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

fn setup_java_environment() {
    let java_home = env::var("JAVA_HOME")
        .unwrap_or_else(|_| {
            println!("cargo:warning=⚠️  JAVA_HOME not set, attempting auto-detection");
            String::new()
        });

    if java_home.is_empty() {
        println!("cargo:warning=⚠️  JAVA_HOME not found, Java integration disabled");
        return;
    }

    println!("cargo:warning=☕ JAVA_HOME: {}", java_home);
    println!("cargo:rustc-env=JAVA_HOME={}", &java_home);

    if cfg!(target_os = "windows") {
        let possible_dirs = [
            format!("{}\\lib", java_home),
            format!("{}\\lib\\server", java_home),
            format!("{}\\bin\\server", java_home),
        ];

        for dir in &possible_dirs {
            let jvm_path = format!("{}\\jvm.lib", dir);
            if Path::new(&jvm_path).exists() {
                println!("cargo:rustc-link-search=native={}", dir);
                println!("cargo:rustc-link-lib=dylib=jvm");
                println!("cargo:warning=Found jvm.lib at: {}", dir);
                return;
            }
        }
        println!("cargo:warning=⚠️  jvm.lib not found in JAVA_HOME");
    } else if cfg!(target_os = "macos") {
        let lib_server = format!("{}/lib/server", java_home);
        let lib_root = format!("{}/lib", java_home);

        if Path::new(&format!("{}/libjvm.dylib", lib_server)).exists() {
            println!("cargo:rustc-link-search=native={}", lib_server);
            println!("cargo:rustc-link-lib=dylib=jvm");
        } else if Path::new(&format!("{}/libjvm.dylib", lib_root)).exists() {
            println!("cargo:rustc-link-search=native={}", lib_root);
            println!("cargo:rustc-link-lib=dylib=jvm");
        } else {
            println!("cargo:warning=⚠️ JVM lib not found on macOS");
        }
    } else {
        // Linux and Unix-like
        let lib_server = format!("{}/lib/server", java_home);
        let lib_root = format!("{}/lib", java_home);

        if Path::new(&format!("{}/libjvm.so", lib_server)).exists() {
            println!("cargo:rustc-link-search=native={}", lib_server);
            println!("cargo:rustc-link-lib=dylib=jvm");
        } else if Path::new(&format!("{}/libjvm.so", lib_root)).exists() {
            println!("cargo:rustc-link-search=native={}", lib_root);
            println!("cargo:rustc-link-lib=dylib=jvm");
        } else {
            println!("cargo:warning=⚠️ JVM lib not found on Linux");
        }
    }
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
    if cfg!(target_os = "windows") {
        if Path::new("src/ASM_Code/bitwise_ops.asm").exists() {
            cc::Build::new()
                .file("src/ASM_Code/bitwise_ops.asm")
                .compile("bitwise_ops");
            println!("cargo:warning=✅ Windows ASM compiled");
        } else {
            println!("cargo:warning=⚠️  Windows ASM file not found");
        }
    } else if cfg!(target_arch = "x86_64") {
        if Path::new("src/ASM_Code/bitwise_ops.s").exists() {
            cc::Build::new()
                .file("src/ASM_Code/bitwise_ops.s")
                .compile("bitwise_ops");
            println!("cargo:warning=✅ Linux/Unix x86_64 ASM compiled");
        } else {
            println!("cargo:warning=⚠️  x86_64 ASM file not found");
        }
    } else if cfg!(target_arch = "aarch64") {
        if Path::new("src/ASM_Code/bitwise_ops_arm64.s").exists() {
            cc::Build::new()
                .file("src/ASM_Code/bitwise_ops_arm64.s")
                .compile("bitwise_ops_arm64");
            println!("cargo:warning=✅ ARM64 ASM compiled");
        } else {
            println!("cargo:warning=⚠️  ARM64 ASM file not found");
        }
    } else {
        let arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_else(|_| "unknown".to_string());
        println!("cargo:warning=⚠️  ASM not supported for: {}", arch);
    }
}

fn compile_java_files() {
    let java_files = [
        "src/JAVA_Code/Fibonacci.java",
        "src/JAVA_Code/PrimeChecker.java",
        "src/JAVA_Code/ArrayStats.java",
        "src/JAVA_Code/StringOps.java",
    ];

    let existing_files: Vec<&str> = java_files.iter()
        .filter(|&&file| Path::new(file).exists())
        .copied()
        .collect();

    if existing_files.is_empty() {
        println!("cargo:warning=⚠️  No Java source files found");
        return;
    }

    let javac_cmd = env::var("JAVA_HOME")
        .map(|java_home| {
            if cfg!(target_os = "windows") {
                format!("{}\\bin\\javac.exe", java_home)
            } else {
                format!("{}/bin/javac", java_home)
            }
        })
        .unwrap_or_else(|_| "javac".to_string());

    match Command::new(&javac_cmd).args(&existing_files).output() {
        Ok(result) => {
            if result.status.success() {
                println!("cargo:warning=✅ Java compiled ({} files)", existing_files.len());
            } else {
                println!("cargo:warning=❌ Java compilation failed: {}", 
                    String::from_utf8_lossy(&result.stderr));
            }
        }
        Err(_) => println!("cargo:warning=⚠️  javac not found, Java disabled"),
    }
}
