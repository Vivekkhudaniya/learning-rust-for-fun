// ─── Concept 3: Functions ────────────────────────────────────────────────────
//
// Run this file with:   cargo run --bin functions
//
// ────────────────────────────────────────────────────────────────────────────

fn main() {
    // ── 1. Calling a simple function ─────────────────────────────────────────
    greet();

    // ── 2. Calling a function with inputs ────────────────────────────────────
    greet_person("Vivek");
    greet_person("Raj");

    // ── 3. Calling a function that returns a value ───────────────────────────
    let result = add(10, 5);
    println!("10 + 5 = {}", result);

    // ── 4. Using the returned value directly ─────────────────────────────────
    println!("20 + 30 = {}", add(20, 30));

    // ── 5. Function that does a calculation and returns ───────────────────────
    let area = rectangle_area(6, 4);
    println!("Area of rectangle: {}", area);

    // ── 6. Function with multiple return — using a bool ───────────────────────
    let age = 20;
    let can_vote = is_adult(age);
    println!("Age: {} — Can vote: {}", age, can_vote);
}

// ── How to write a function ──────────────────────────────────────────────────
//
//   fn function_name(parameter: type) -> return_type {
//       // your code
//   }
//
//   fn   = keyword to define a function
//   ()   = inputs go here (empty means no inputs)
//   ->   = "this function gives back a value of this type"
// ─────────────────────────────────────────────────────────────────────────────

// Simple function — no input, no output
fn greet() {
    println!("Hello from a function!");
}

// Function with one input (parameter)
//   name: &str   means we're passing in some text
fn greet_person(name: &str) {
    println!("Hello, {}!", name);
}

// Function with two inputs and a return value
//   a: i32, b: i32   = two whole numbers coming in
//   -> i32           = a whole number coming back out
fn add(a: i32, b: i32) -> i32 {
    a + b   // no semicolon on the last line = this value is returned
            // same as writing:  return a + b;
}

// Another example with return
fn rectangle_area(width: i32, height: i32) -> i32 {
    width * height
}

// Function that returns a bool (true or false)
fn is_adult(age: i32) -> bool {
    age >= 18
}
