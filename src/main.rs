// src/main.rs
use std::io::{self, Write};
mod calculator;

fn main() {
    println!("🧮 Multi-Language Calculator");
    println!("Languages: Rust + C + C++ + Assembly + Java (JNI)");
    println!("=============================================");

    loop {
        print_menu();
        let choice = get_user_input("Enter your choice: ");
        match choice.trim() {
            "1" => basic_operations_menu(),
            "2" => advanced_operations_menu(),
            "3" => bitwise_operations_menu(),
            "4" => java_operations_menu(),
            "5" => {
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
    println!("4. Java Operations (JNI)");
    println!("5. Exit");
}

fn basic_operations_menu() {
    println!("\n➕ Basic Operations (Powered by C)");
    println!("1. Addition 2. Subtraction 3. Multiplication 4. Division");
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
    println!("3. Sine 4. Cosine 5. Tangent 6. Natural Log");
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
        // These could be JNI calls if appropriate Java methods are implemented!
        _ => {
            println!("Operation not implemented in this demo!");
            return;
        }
    };
    println!("Result: {}", result);
}

fn bitwise_operations_menu() {
    println!("\n⚡ Bitwise Operations (Assembly)");
    println!("1. AND 2. OR 3. XOR");
    let op = get_user_input("Choose operation (1-3): ");
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
        _ => {
            println!("Invalid operation!");
            return;
        }
    };
    println!("Result: {} (0x{:x})", result, result);
}

fn java_operations_menu() {
    println!("\n☕ Java Operations (Powered by JNI)");
    println!("1. Fibonacci (Nth Number)");
    println!("2. Prime Checker");
    println!("3. Array Median");
    println!("4. String Uppercase");
    let op = get_user_input("Choose operation (1-4): ");
    match op.trim() {
        "1" => {
            let n = get_integer("Enter Fibonacci sequence index: ") as i32;
            let result = calculator::java_fibonacci(n);
            println!("Fibonacci({}) = {}", n, result);
        }
        "2" => {
            let num = get_integer("Enter number to check: ") as i64;
            let is_prime = calculator::java_is_prime(num);
            println!("{} is{} prime", num, if is_prime { "" } else { " not" });
        }
        "3" => {
            println!("Enter numbers separated by spaces:");
            let input = get_user_input("Numbers: ");
            let numbers: Vec<f64> = input
                .split_whitespace()
                .filter_map(|s| s.parse().ok())
                .collect();
            if numbers.is_empty() {
                println!("No valid numbers entered!");
            } else {
                let median = calculator::java_array_median(&numbers);
                println!("Array Median: {}", median);
            }
        }
        "4" => {
            let text = get_user_input("Enter text: ").trim().to_string();
            if text.is_empty() {
                println!("No text entered!");
            } else {
                let result = calculator::java_string_uppercase(&text);
                println!("Uppercase: {}", result);
            }
        }
        _ => println!("Invalid operation!"),
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
