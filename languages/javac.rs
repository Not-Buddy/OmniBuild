use std::env;
use std::process::Command;
use std::path::Path;

pub fn compile_java_files() {
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
        println!("cargo:warning=⚠️ No Java source files found");
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
        Err(_) => println!("cargo:warning=⚠️ javac not found, Java disabled"),
    }
}

pub fn setup_java_environment() {
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