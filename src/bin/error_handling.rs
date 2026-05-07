// ─── Concept 14: Error Handling ──────────────────────────────────────────────
//
// Run this file with:   cargo run --bin error_handling
//
// Rust has no exceptions (no try/catch like other languages).
// Instead, errors are just values — you return them and handle them explicitly.
//
// Two main types:
//   Result<T, E>  — for things that can fail (file read, parsing, etc.)
//   Option<T>     — for things that might not exist (covered more in concept 15)
//
// ────────────────────────────────────────────────────────────────────────────


// ── Result<T, E> ──────────────────────────────────────────────────────────────
//    Ok(value)  — it worked, here's the value
//    Err(e)     — it failed, here's the error

fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("Cannot divide by zero"))
    } else {
        Ok(a / b)
    }
}

fn parse_age(input: &str) -> Result<u32, String> {
    match input.trim().parse::<u32>() {
        Ok(age)  => Ok(age),
        Err(_)   => Err(format!("'{}' is not a valid age", input)),
    }
}


fn main() {
    // ── 1. Handling Result with match ─────────────────────────────────────────
    let result = divide(10.0, 2.0);

    match result {
        Ok(answer) => println!("10 / 2 = {}", answer),
        Err(e)     => println!("Error: {}", e),
    }

    let result = divide(10.0, 0.0);

    match result {
        Ok(answer) => println!("Answer: {}", answer),
        Err(e)     => println!("Error: {}", e),
    }


    // ── 2. unwrap() — get the value, crash if it's an error ──────────────────
    //    Only use this when you are 100% sure it will succeed
    let answer = divide(20.0, 4.0).unwrap();
    println!("\n20 / 4 = {}", answer);

    // divide(5.0, 0.0).unwrap();  // this would crash the program


    // ── 3. expect() — like unwrap but with a custom crash message ────────────
    //    Better than unwrap because the message tells you what went wrong
    let answer = divide(9.0, 3.0).expect("Division failed");
    println!("9 / 3 = {}", answer);


    // ── 4. is_ok() and is_err() — check without extracting ───────────────────
    let good = divide(10.0, 2.0);
    let bad  = divide(10.0, 0.0);

    println!("\nGood result is ok:  {}", good.is_ok());
    println!("Bad result is err:  {}", bad.is_err());


    // ── 5. Real example — parsing user input ─────────────────────────────────
    let inputs = vec!["21", "abc", "17"];

    println!();
    for input in inputs {
        match parse_age(input) {
            Ok(age)  => println!("Valid age: {}", age),
            Err(e)   => println!("Invalid: {}", e),
        }
    }


    // ── 6. unwrap_or() — use a default value if it's an error ────────────────
    let age = parse_age("abc").unwrap_or(0);
    println!("\nFallback age: {}", age);

    // ── 7. unwrap_or_else() — run a function if it's an error ────────────────
    let age = parse_age("xyz").unwrap_or_else(|e| {
        println!("Caught error: {}", e);
        0
    });
    println!("Age after fallback: {}", age);
}
