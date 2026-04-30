// ─── Concept 2: Variables and Data Types ────────────────────────────────────
//
// Run this file with:   cargo run --bin variables
//
// ────────────────────────────────────────────────────────────────────────────

fn main() {
    // ── 1. Creating a variable ───────────────────────────────────────────────
    //    Use the `let` keyword
    //    By default, variables CANNOT be changed (immutable)
    let age = 21;
    println!("Age: {}", age);

    // ── 2. Mutable variable ──────────────────────────────────────────────────
    //    Add `mut` if you want to change the value later
    let mut score = 0;
    println!("Score before: {}", score);
    score = 10;
    println!("Score after:  {}", score);

    // ── 3. Number types ──────────────────────────────────────────────────────
    let whole_number: i32 = 100;       // i32  = whole number (positive or negative)
    let big_number: i64 = 1_000_000;   // i64  = bigger whole number
    let decimal: f64 = 3.14;           // f64  = number with decimal point
    let positive_only: u32 = 50;       // u32  = whole number, positive only

    println!("i32:  {}", whole_number);
    println!("i64:  {}", big_number);
    println!("f64:  {}", decimal);
    println!("u32:  {}", positive_only);

    // ── 4. Text types ────────────────────────────────────────────────────────
    let greeting: &str = "Hello";          // &str  = simple text, fixed, borrowed
    let full_name: String = String::from("Devesh Chaudhary"); // String = text you own and can change

    println!("{}, {}!", greeting, full_name);

    // ── 5. Boolean ───────────────────────────────────────────────────────────
    let is_learning: bool = true;
    let is_bored: bool = false;

    println!("Learning Rust: {}", is_learning);
    println!("Bored:         {}", is_bored);

    // ── 6. Constants ─────────────────────────────────────────────────────────
    //    Like `let` but NEVER changes, ever
    //    Written in ALL_CAPS by convention
    //    Must always have a type
    const MAX_SCORE: i32 = 100;
    println!("Max score: {}", MAX_SCORE);

    // ── 7. Type can be guessed by Rust (type inference) ──────────────────────
    //    You don't always need to write the type — Rust figures it out
    let city = "Mumbai";      // Rust knows this is &str
    let population = 20_000_000; // Rust knows this is i32
    println!("{} has {} people", city, population);
}
