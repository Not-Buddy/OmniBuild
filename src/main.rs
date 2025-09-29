// src/main.rs
use std::io::{self, Write};

mod calculator;

fn main() {
    println!("🧮 Multi-Language Calculator");
    println!("Languages: Rust + C + C++ + Assembly");
    println!("========================================");
    
    loop {
        print_menu();
        
        let choice = get_user_input("Enter your choice: ");
        
        match choice.trim() {
            "1" => basic_operations_menu(),
            "2" => advanced_operations_menu(),
            "3" => bitwise_operations_menu(),
            "4" => {
                println!("Thanks for using Multi-Language Calculator!");
                break;
            }
            _ => println!("Invalid choice! Please try again."),
        }
    }
}

fn print_menu() {
    println!("\n📋 Main Menu:");
    println!("1. Basic Operations (C)");
    println!("2. Advanced Operations (C++)");
    println!("3. Bitwise Operations (Assembly)");
    println!("4. Exit");
}

fn basic_operations_menu() {
    println!("\n➕ Basic Operations (Powered by C)");
    println!("1. Addition    2. Subtraction");
    println!("3. Multiplication    4. Division");
    
    let op = get_user_input("Choose operation (1-4): ");
    let a = get_number("Enter first number: ");
    let b = get_number("Enter second number: ");
    
    let result = match op.trim() {
        "1" => calculator::add(a, b),
        "2" => calculator::subtract(a, b),
        "3" => calculator::multiply(a, b),
        "4" => calculator::divide(a, b),
        _ => {
            println!("Invalid operation!");
            return;
        }
    };
    
    println!("Result: {}", result);
}

fn advanced_operations_menu() {
    println!("\n🚀 Advanced Operations (Powered by C++)");
    println!("1. Power      2. Square Root");
    println!("3. Sine       4. Cosine");
    println!("5. Tangent    6. Natural Log");
    
    let op = get_user_input("Choose operation (1-6): ");
    
    let result = match op.trim() {
        "1" => {
            let base = get_number("Enter base: ");
            let exp = get_number("Enter exponent: ");
            calculator::power(base, exp)
        }
        "2" => {
            let x = get_number("Enter number: ");
            calculator::sqrt(x)
        }
        "3" => {
            let x = get_number("Enter angle (radians): ");
            calculator::sin(x)
        }
        "4" => {
            let x = get_number("Enter angle (radians): ");
            calculator::cos(x)
        }
        "5" => {
            let x = get_number("Enter angle (radians): ");
            calculator::tan(x)
        }
        "6" => {
            let x = get_number("Enter number: ");
            calculator::log(x)
        }
        _ => {
            println!("Invalid operation!");
            return;
        }
    };
    
    println!("Result: {}", result);
}

fn bitwise_operations_menu() {
    println!("\n⚡ Bitwise Operations (Powered by Assembly)");
    println!("1. AND      2. OR       3. XOR");
    println!("4. Shift Left    5. Shift Right");
    println!("6. Count Bits    7. Reverse Bits");
    
    let op = get_user_input("Choose operation (1-7): ");
    
    let result = match op.trim() {
        "1" => {
            let a = get_integer("Enter first number: ");
            let b = get_integer("Enter second number: ");
            calculator::bitwise_and(a, b)
        }
        "2" => {
            let a = get_integer("Enter first number: ");
            let b = get_integer("Enter second number: ");
            calculator::bitwise_or(a, b)
        }
        "3" => {
            let a = get_integer("Enter first number: ");
            let b = get_integer("Enter second number: ");
            calculator::bitwise_xor(a, b)
        }
        "4" => {
            let value = get_integer("Enter value: ");
            let shift = get_integer("Enter shift amount: ");
            calculator::shift_left(value, shift)
        }
        "5" => {
            let value = get_integer("Enter value: ");
            let shift = get_integer("Enter shift amount: ");
            calculator::shift_right(value, shift)
        }
        "6" => {
            let value = get_integer("Enter number: ");
            calculator::count_set_bits(value)
        }
        "7" => {
            let value = get_integer("Enter number: ");
            calculator::reverse_bits(value)
        }
        _ => {
            println!("Invalid operation!");
            return;
        }
    };
    
    println!("Result: {} (0x{:x})", result, result);
}

fn get_user_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input
}

fn get_number(prompt: &str) -> f64 {
    loop {
        let input = get_user_input(prompt);
        match input.trim().parse::<f64>() {
            Ok(num) => return num,
            Err(_) => println!("Please enter a valid number!"),
        }
    }
}

fn get_integer(prompt: &str) -> u64 {
    loop {
        let input = get_user_input(prompt);
        match input.trim().parse::<u64>() {
            Ok(num) => return num,
            Err(_) => println!("Please enter a valid integer!"),
        }
    }
}
