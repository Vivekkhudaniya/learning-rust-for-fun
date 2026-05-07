// ─── Project 1: CLI Calculator ───────────────────────────────────────────────
//
// Run this file with:   cargo run --bin calculator
//
// Concepts used: variables, functions, user input, match, loop, error handling
//
// ────────────────────────────────────────────────────────────────────────────

use std::io;
use std::io::Write;

fn add(a: f64, b: f64) -> f64      { a + b }
fn subtract(a: f64, b: f64) -> f64 { a - b }
fn multiply(a: f64, b: f64) -> f64 { a * b }
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("Cannot divide by zero"))
    } else {
        Ok(a / b)
    }
}

fn read_number(prompt: &str) -> f64 {
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim().parse::<f64>() {
            Ok(n)  => return n,
            Err(_) => println!("That's not a valid number. Try again."),
        }
    }
}

fn read_operator() -> char {
    loop {
        print!("Operator (+, -, *, /): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "+" => return '+',
            "-" => return '-',
            "*" => return '*',
            "/" => return '/',
            _   => println!("Invalid operator. Use +, -, * or /"),
        }
    }
}

fn main() {
    println!("================================");
    println!("       CLI Calculator");
    println!("================================");
    println!("Type 'q' as first number to quit\n");

    loop {
        // check if user wants to quit
        print!("First number (or 'q' to quit): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        if input.trim() == "q" {
            println!("Bye!");
            break;
        }

        let a: f64 = match input.trim().parse() {
            Ok(n)  => n,
            Err(_) => {
                println!("Not a valid number. Try again.\n");
                continue;
            }
        };

        let op = read_operator();
        let b  = read_number("Second number: ");

        let result = match op {
            '+' => Ok(add(a, b)),
            '-' => Ok(subtract(a, b)),
            '*' => Ok(multiply(a, b)),
            '/' => divide(a, b),
            _   => Err(String::from("Unknown operator")),
        };

        match result {
            Ok(answer) => println!("\n  {} {} {} = {}\n", a, op, b, answer),
            Err(e)     => println!("\n  Error: {}\n", e),
        }

        println!("--------------------------------");
    }
}
