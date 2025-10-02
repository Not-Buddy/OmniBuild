// src/calculator.rs
use std::process::Command;

// C function declarations
unsafe extern "C" {
    fn c_add(a: f64, b: f64) -> f64;
    fn c_subtract(a: f64, b: f64) -> f64;
    fn c_multiply(a: f64, b: f64) -> f64;
    fn c_divide(a: f64, b: f64) -> f64;
}

// C++ function declarations
unsafe extern "C" {
    fn cpp_power(base: f64, exponent: f64) -> f64;
    fn cpp_sqrt(x: f64) -> f64;
    fn cpp_sin(x: f64) -> f64;
    fn cpp_cos(x: f64) -> f64;
    fn cpp_tan(x: f64) -> f64;
    fn cpp_log(x: f64) -> f64;
}

// Assembly function declarations
unsafe extern "C" {
    fn asm_fast_and(a: u64, b: u64) -> u64;
    fn asm_fast_or(a: u64, b: u64) -> u64;
    fn asm_fast_xor(a: u64, b: u64) -> u64;
    fn asm_fast_shift_left(value: u64, shift: u64) -> u64;
    fn asm_fast_shift_right(value: u64, shift: u64) -> u64;
    fn asm_count_bits(value: u64) -> u64;
    fn asm_reverse_bits(value: u64) -> u64;
}

// Safe Rust wrappers for C functions
pub fn add(a: f64, b: f64) -> f64 {
    unsafe { c_add(a, b) }
}

pub fn subtract(a: f64, b: f64) -> f64 {
    unsafe { c_subtract(a, b) }
}

pub fn multiply(a: f64, b: f64) -> f64 {
    unsafe { c_multiply(a, b) }
}

pub fn divide(a: f64, b: f64) -> f64 {
    unsafe { c_divide(a, b) }
}

// Safe Rust wrappers for C++ functions
pub fn power(base: f64, exponent: f64) -> f64 {
    unsafe { cpp_power(base, exponent) }
}

pub fn sqrt(x: f64) -> f64 {
    let result = unsafe { cpp_sqrt(x) };
    if result < 0.0 {
        println!("Error: Cannot calculate square root of negative number");
        0.0
    } else {
        result
    }
}

pub fn sin(x: f64) -> f64 {
    unsafe { cpp_sin(x) }
}

pub fn cos(x: f64) -> f64 {
    unsafe { cpp_cos(x) }
}

pub fn tan(x: f64) -> f64 {
    unsafe { cpp_tan(x) }
}

pub fn log(x: f64) -> f64 {
    let result = unsafe { cpp_log(x) };
    if result < 0.0 {
        println!("Error: Cannot calculate log of non-positive number");
        0.0
    } else {
        result
    }
}

// Safe Rust wrappers for Assembly functions
pub fn bitwise_and(a: u64, b: u64) -> u64 {
    unsafe { asm_fast_and(a, b) }
}

pub fn bitwise_or(a: u64, b: u64) -> u64 {
    unsafe { asm_fast_or(a, b) }
}

pub fn bitwise_xor(a: u64, b: u64) -> u64 {
    unsafe { asm_fast_xor(a, b) }
}

pub fn shift_left(value: u64, shift: u64) -> u64 {
    unsafe { asm_fast_shift_left(value, shift) }
}

pub fn shift_right(value: u64, shift: u64) -> u64 {
    unsafe { asm_fast_shift_right(value, shift) }
}

pub fn count_set_bits(value: u64) -> u64 {
    unsafe { asm_count_bits(value) }
}

pub fn reverse_bits(value: u64) -> u64 {
    unsafe { asm_reverse_bits(value) }
}

// ============================================================================
//                              JAVA FUNCTIONS
// ============================================================================

pub fn java_fibonacci(n: i32) {
    execute_java_program("Fibonacci", &[&n.to_string()]);
}

pub fn java_is_prime(num: u64) -> bool {
    let output = execute_java_program("PrimeChecker", &[&num.to_string()]);
    output.contains("Is Prime: true")
}

pub fn java_array_stats(numbers: Vec<f64>) {
    let args: Vec<String> = numbers.iter().map(|n| n.to_string()).collect();
    let str_args: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
    execute_java_program("ArrayStats", &str_args);
}

pub fn java_string_ops(text: String) {
    execute_java_program("StringOps", &[&text]);
}

// Helper function to execute pre-compiled Java programs
fn execute_java_program(class_name: &str, args: &[&str]) -> String {
    let java_dir = "src/JAVA_Code";
    
    // Compile Java file if needed
    let java_file = format!("{}/{}.java", java_dir, class_name);
    let class_file = format!("{}/{}.class", java_dir, class_name);
    
    // Check if class file exists and is newer than source
    let compile_needed = if std::path::Path::new(&class_file).exists() {
        match (std::fs::metadata(&java_file), std::fs::metadata(&class_file)) {
            (Ok(java_meta), Ok(class_meta)) => {
                java_meta.modified().unwrap() > class_meta.modified().unwrap()
            }
            _ => true,
        }
    } else {
        true
    };
    
    if compile_needed {
        let compile_output = Command::new("javac")
            .arg(&java_file)
            .output();
        
        match compile_output {
            Ok(result) => {
                if !result.status.success() {
                    println!("❌ Java compilation failed: {}", String::from_utf8_lossy(&result.stderr));
                    return String::new();
                }
            }
            Err(_) => {
                println!("❌ javac not found! Please install Java Development Kit (JDK)");
                println!("   On Ubuntu: sudo apt install default-jdk");
                println!("   On Windows: Download from https://adoptium.net/");
                return String::new();
            }
        }
    }
    
    // Run Java program
    let run_output = Command::new("java")
        .arg("-cp")
        .arg(java_dir)
        .arg(class_name)
        .args(args)
        .output();
    
    match run_output {
        Ok(result) => {
            if result.status.success() {
                let output = String::from_utf8_lossy(&result.stdout);
                print!("{}", output);
                output.to_string()
            } else {
                println!("❌ Java execution failed: {}", String::from_utf8_lossy(&result.stderr));
                String::new()
            }
        }
        Err(_) => {
            println!("❌ java not found! Please install Java Runtime Environment (JRE)");
            println!("   On Ubuntu: sudo apt install default-jre");
            println!("   On Windows: Download from https://adoptium.net/");
            String::new()
        }
    }
}
