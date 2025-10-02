use std::process::Command;

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

    // Compile Assembly based on target architecture
    if cfg!(target_arch = "x86_64") {
        cc::Build::new()
            .file("src/ASM_Code/bitwise_ops.s")
            .compile("bitwise_ops");
    } else {
        println!("cargo:warning=Assembly not supported on this architecture");
    }

    // Compile Java files
    compile_java_files();
    
    println!("cargo:rerun-if-changed=src/C_Code/");
    println!("cargo:rerun-if-changed=src/CPP_Code/");
    println!("cargo:rerun-if-changed=src/ASM_Code/");
    println!("cargo:rerun-if-changed=src/JAVA_Code/");
}

fn compile_java_files() {
    let java_files = [
        "src/JAVA_Code/Fibonacci.java",
        "src/JAVA_Code/PrimeChecker.java", 
        "src/JAVA_Code/ArrayStats.java",
        "src/JAVA_Code/StringOps.java",
    ];
    
    // Check if javac is available
    match Command::new("javac").arg("-version").output() {
        Ok(_) => {
            println!("cargo:warning=Compiling Java files...");
            
            // Compile all Java files
            let mut cmd = Command::new("javac");
            cmd.args(&java_files);
            
            match cmd.output() {
                Ok(result) => {
                    if result.status.success() {
                        println!("cargo:warning=Java compilation successful");
                    } else {
                        println!("cargo:warning=Java compilation failed: {}", 
                                String::from_utf8_lossy(&result.stderr));
                    }
                }
                Err(_) => println!("cargo:warning=Failed to run javac"),
            }
        }
        Err(_) => println!("cargo:warning=Java compiler (javac) not found"),
    }
}
