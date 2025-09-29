// src/calculator.rs

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
