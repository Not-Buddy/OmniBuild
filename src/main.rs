// src/main.rs
use std::io::{self, Write};

mod calculator;

fn main() {
    println!("🧮 Multi-Language Calculator");
    println!("Languages: Rust + C + C++ + Assembly + Java");  // Updated
    println!("=============================================");

    loop {
        print_menu();
        let choice = get_user_input("Enter your choice: ");

        match choice.trim() {
            "1" => basic_operations_menu(),
            "2" => advanced_operations_menu(),
            "3" => bitwise_operations_menu(),
            "4" => java_operations_menu(),  // New Java menu
            "5" => {  // Updated exit option
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
    println!("4. Java Operations (Java)");  // New option
    println!("5. Exit");  // Updated
}

fn basic_operations_menu() {
    println!("\n➕ Basic Operations (Powered by C)");
    println!("1. Addition 2. Subtraction");
    println!("3. Multiplication 4. Division");
    
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
    println!("1. Power 2. Square Root");
    println!("3. Sine 4. Cosine");
    println!("5. Tangent 6. Natural Log");
    
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
    println!("1. AND 2. OR 3. XOR");
    println!("4. Shift Left 5. Shift Right");
    println!("6. Count Bits 7. Reverse Bits");
    
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

// New Java operations menu
fn java_operations_menu() {
    println!("\n☕ Java Operations (Powered by Java)");
    println!("1. Calculate Fibonacci Sequence");
    println!("2. Check Prime Number");
    println!("3. Array Statistics Analysis");
    println!("4. String Operations");
    
    let op = get_user_input("Choose operation (1-4): ");
    
    match op.trim() {
        "1" => {
            let n = get_integer("Enter Fibonacci sequence length: ") as i32;
            if n <= 0 {
                println!("Please enter a positive number!");
                return;
            }
            calculator::java_fibonacci(n);
        }
        "2" => {
            let num = get_integer("Enter number to check: ");
            let is_prime = calculator::java_is_prime(num);
            if !is_prime {
                println!("\n🔍 Additional Info: {} is not a prime number", num);
            }
        }
        "3" => {
            println!("Enter numbers separated by spaces (e.g., 1.5 2.3 4.1 3.7):");
            let input = get_user_input("Numbers: ");
            let numbers: Vec<f64> = input
                .split_whitespace()
                .filter_map(|s| s.parse().ok())
                .collect();
            
            if numbers.is_empty() {
                println!("No valid numbers entered!");
            } else {
                calculator::java_array_stats(numbers);
            }
        }
        "4" => {
            let text = get_user_input("Enter text to analyze: ");
            let clean_text = text.trim().to_string();
            if clean_text.is_empty() {
                println!("No text entered!");
            } else {
                calculator::java_string_ops(clean_text);
            }
        }
        _ => {
            println!("Invalid operation!");
        }
    }
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
