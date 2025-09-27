// src/main.rs
unsafe extern "C" {
    fn twice(x: i32) -> i32;
}

fn main() {
    let value = 5;
    // Calling C function is unsafe
    let result = unsafe { twice(value) };
    println!("Twice {} is {}", value, result);
}

