// ─── Concept 13: Pattern Matching ────────────────────────────────────────────
//
// Run this file with:   cargo run --bin pattern_matching
//
// We used `match` with enums in concept 12.
// But match works on ANY value — numbers, strings, tuples, structs.
// This concept covers all the ways you can use it.
//
// ────────────────────────────────────────────────────────────────────────────

#[allow(dead_code)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn main() {
    // ── 1. match on a number ──────────────────────────────────────────────────
    //    Every possible case must be handled — Rust forces you to
    let score = 85;

    let grade = match score {
        90..=100 => "A",
        80..=89  => "B",
        70..=79  => "C",
        60..=69  => "D",
        _        => "F",   // _ = everything else (like default)
    };
    println!("Score {} = Grade {}", score, grade);


    // ── 2. match on an enum ───────────────────────────────────────────────────
    let coin = Coin::Quarter;

    let value = match coin {
        Coin::Penny   => 1,
        Coin::Nickel  => 5,
        Coin::Dime    => 10,
        Coin::Quarter => 25,
    };
    println!("Coin value: {} cents", value);


    // ── 3. match with multiple patterns ──────────────────────────────────────
    //    Use | to match more than one value at once
    let day = "Saturday";

    let kind = match day {
        "Saturday" | "Sunday" => "Weekend",
        _                     => "Weekday",
    };
    println!("{} is a {}", day, kind);


    // ── 4. match with a guard ─────────────────────────────────────────────────
    //    Add an `if` condition inside a match arm for extra filtering
    let age = 20;

    let category = match age {
        n if n < 13  => "child",
        n if n < 18  => "teenager",
        n if n < 60  => "adult",
        _            => "senior",
    };
    println!("Age {} is a {}", age, category);


    // ── 5. match on a tuple ───────────────────────────────────────────────────
    //    Match multiple values at once by grouping them in a tuple
    let location = ("India", "Mumbai");

    match location {
        ("India", "Mumbai")  => println!("You are in Mumbai, India"),
        ("India", city)      => println!("You are in {}, India", city),
        (country, _)         => println!("You are in {}", country),
    }


    // ── 6. if let — shorter match when you only care about one case ───────────
    //    Instead of writing a full match with _ => () for the rest,
    //    use `if let` when you only want to handle one specific case
    let favorite_color: Option<&str> = Some("blue");

    // long way with match:
    // match favorite_color {
    //     Some(color) => println!("Favorite color: {}", color),
    //     None        => (),
    // }

    // short way with if let:
    if let Some(color) = favorite_color {
        println!("Favorite color: {}", color);
    }


    // ── 7. if let with else ───────────────────────────────────────────────────
    let points: Option<i32> = None;

    if let Some(p) = points {
        println!("You have {} points", p);
    } else {
        println!("No points yet");
    }
}
