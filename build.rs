// build.rs
use std::process::Command;
use std::path::Path;
use std::env;
use std::fs;

fn main() {
    println!("🔨 OmniBuild Multi-Architecture Build System");
    
    // Load .env file manually (no external dependency needed)
    load_env_file();
    
    // Set up Java environment from .env
    setup_java_environment();
    
    // Compile all components
    compile_c_code();
    compile_cpp_code();
    compile_assembly_code();
    compile_java_files();
    
    // Set up rerun triggers
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
                // Use unsafe block for set_var
                unsafe {
                    env::set_var(key.trim(), value.trim());
                }
                println!("cargo:warning=🔧 Set {} from .env", key.trim());
            }
        }
    } else {
        println!("cargo:warning=⚠️  No .env file found (this is optional)");
    }
}

fn setup_java_environment() {
    let java_home = env::var("JAVA_HOME")
        .or_else(|_| find_java_home())
        .unwrap_or_else(|_| {
            println!("cargo:warning=⚠️  JAVA_HOME not found in .env or environment");
            String::new()
        });

    if !java_home.is_empty() {
        println!("cargo:warning=☕ JAVA_HOME: {}", java_home);
        
        // Set environment variable for runtime
        println!("cargo:rustc-env=JAVA_HOME={}", java_home);
        
        // Set library search paths for JNI
        if cfg!(target_os = "windows") {
            let jvm_lib = format!("{}\\bin\\server", java_home);
            println!("cargo:rustc-link-search=native={}", jvm_lib);
            println!("cargo:rustc-link-lib=dylib=jvm");
        } else if cfg!(target_os = "macos") {
            let jvm_lib = format!("{}/lib/server", java_home);
            println!("cargo:rustc-link-search=native={}", jvm_lib);
            println!("cargo:rustc-link-lib=dylib=jvm");
        } else {
            // Linux
            let jvm_lib = format!("{}/lib/server", java_home);
            println!("cargo:rustc-link-search=native={}", jvm_lib);
            println!("cargo:rustc-link-lib=dylib=jvm");
        }
    }
}

fn find_java_home() -> Result<String, env::VarError> {
    let common_paths = if cfg!(target_os = "windows") {
        vec![
            "C:\\Program Files\\Java\\jdk-11",
            "C:\\Program Files\\Java\\jdk-17",
            "C:\\Program Files\\OpenJDK\\jdk-11",
        ]
    } else if cfg!(target_os = "macos") {
        vec![
            "/Library/Java/JavaVirtualMachines/openjdk-11.jdk/Contents/Home",
            "/usr/local/opt/openjdk@11",
            "/opt/homebrew/opt/openjdk@11",
        ]
    } else {
        vec![
            "/usr/lib/jvm/default-java",
            "/usr/lib/jvm/java-11-openjdk-amd64",
            "/usr/lib/jvm/java-17-openjdk-amd64",
            "/opt/java/openjdk",
        ]
    };

    for path in common_paths {
        if Path::new(path).exists() {
            println!("cargo:warning=🔍 Auto-detected Java at: {}", path);
            return Ok(path.to_string());
        }
    }
    
    Err(env::VarError::NotPresent)
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
    if cfg!(target_arch = "x86_64") {
        if Path::new("src/ASM_Code/bitwise_ops.s").exists() {
            cc::Build::new()
                .file("src/ASM_Code/bitwise_ops.s")
                .compile("bitwise_ops");
            println!("cargo:warning=✅ x86_64 Assembly compiled");
        } else {
            println!("cargo:warning=⚠️  x86_64 Assembly file not found");
        }
    } else if cfg!(target_arch = "aarch64") {
        if Path::new("src/ASM_Code/bitwise_ops_arm64.s").exists() {
            cc::Build::new()
                .file("src/ASM_Code/bitwise_ops_arm64.s")
                .compile("bitwise_ops_arm64");
            println!("cargo:warning=✅ ARM64 Assembly compiled");
        } else {
            println!("cargo:warning=⚠️  ARM64 Assembly file not found");
        }
    } else {
        let arch = env::var("CARGO_CFG_TARGET_ARCH")
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
    
    let existing_files: Vec<&str> = java_files.iter()
        .filter(|&file| Path::new(file).exists())
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
        Err(_) => println!("cargo:warning=⚠️  javac not found - Java disabled"),
    }
}
