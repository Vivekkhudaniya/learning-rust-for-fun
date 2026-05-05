// ─── Concept 4: Control Flow ─────────────────────────────────────────────────
//
// Run this file with:   cargo run --bin control_flow
//
// ────────────────────────────────────────────────────────────────────────────

fn main() {
    // ── 1. if / else if / else ───────────────────────────────────────────────
    let age = 20;

    if age < 13 {
        println!("You are a child.");
    } else if age < 18 {
        println!("You are a teenager.");
    } else {
        println!("You are an adult.");
    }

    // ── 2. if as a value ─────────────────────────────────────────────────────
    //    In Rust, if can return a value — no need for a ternary operator
    let score = 85;
    let grade = if score >= 90 {
        "A"
    } else if score >= 75 {
        "B"
    } else {
        "C"
    };
    println!("Grade: {}", grade);

    // ── 3. loop — runs forever until you break out ────────────────────────────
    let mut count = 0;

    loop {
        count += 1;
        println!("count: {}", count);

        if count == 3 {
            break;   // stop the loop
        }
    }

    // ── 4. loop can return a value ────────────────────────────────────────────
    let mut number = 0;
    let result = loop {
        number += 1;
        if number == 5 {
            break number * 2;   // returns 10
        }
    };
    println!("Loop result: {}", result);

    // ── 5. while — runs as long as a condition is true ────────────────────────
    let mut x = 1;

    while x <= 5 {
        println!("x = {}", x);
        x += 1;
    }

    // ── 6. for — loop over a range ────────────────────────────────────────────
    //    1..5   = 1, 2, 3, 4       (5 not included)
    //    1..=5  = 1, 2, 3, 4, 5    (5 included)
    for i in 1..=5 {
        println!("i = {}", i);
    }

    // ── 7. for — loop over a list ─────────────────────────────────────────────
    let fruits = ["apple", "banana", "mango"];

    for fruit in fruits {
        println!("Fruit: {}", fruit);
    }

    // ── 8. continue — skip the current step and go to next ───────────────────
    for i in 1..=5 {
        if i == 3 {
            continue;   // skip 3
        }
        println!("{}", i);   // prints 1, 2, 4, 5
    }
}
