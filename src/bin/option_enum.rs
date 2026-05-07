// ─── Concept 15: Option Enum ─────────────────────────────────────────────────
//
// Run this file with:   cargo run --bin option_enum
//
// In other languages, a variable can be `null` — and accessing a null value
// crashes the program. Rust removes null entirely.
//
// Instead, Rust uses Option<T>:
//   Some(value)  — there is a value
//   None         — there is no value
//
// The compiler forces you to handle both cases — no surprise crashes.
//
// ────────────────────────────────────────────────────────────────────────────


fn find_first_even(numbers: &[i32]) -> Option<i32> {
    for &n in numbers {
        if n % 2 == 0 {
            return Some(n);
        }
    }
    None  // nothing found
}


fn main() {
    // ── 1. Basic Option ───────────────────────────────────────────────────────
    let some_number: Option<i32> = Some(42);
    let no_number: Option<i32>   = None;

    println!("some_number: {:?}", some_number);
    println!("no_number:   {:?}", no_number);


    // ── 2. Handling Option with match ─────────────────────────────────────────
    let name: Option<&str> = Some("Vivek");

    match name {
        Some(n) => println!("Name is: {}", n),
        None    => println!("No name provided"),
    }


    // ── 3. if let — shorter way to handle Some ────────────────────────────────
    let age: Option<u32> = Some(21);

    if let Some(a) = age {
        println!("Age is: {}", a);
    } else {
        println!("Age not provided");
    }


    // ── 4. unwrap() — get the value, crash if None ───────────────────────────
    //    Only use when you are sure it's Some
    let city: Option<&str> = Some("Mumbai");
    println!("City: {}", city.unwrap());


    // ── 5. unwrap_or() — use a default if None ───────────────────────────────
    let nickname: Option<&str> = None;
    println!("Nickname: {}", nickname.unwrap_or("no nickname"));


    // ── 6. is_some() and is_none() ────────────────────────────────────────────
    let score: Option<i32> = Some(95);
    println!("\nHas score:  {}", score.is_some());
    println!("Has no score: {}", score.is_none());


    // ── 7. Real example — searching a list ───────────────────────────────────
    let numbers = vec![1, 3, 5, 8, 11];

    match find_first_even(&numbers) {
        Some(n) => println!("\nFirst even number: {}", n),
        None    => println!("\nNo even numbers found"),
    }

    let odd_only = vec![1, 3, 5, 7];

    match find_first_even(&odd_only) {
        Some(n) => println!("First even number: {}", n),
        None    => println!("No even numbers found"),
    }


    // ── Key idea ──────────────────────────────────────────────────────────────
    //    Option<T> forces you to think about the "nothing" case
    //    No null pointer crashes — the compiler won't let you forget
}
