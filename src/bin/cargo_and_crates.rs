// ─── Concept 16: Cargo, Packages and External Dependencies ───────────────────
//
// Run this file with:   cargo run --bin cargo_and_crates
//
// Cargo is Rust's built-in package manager and build tool.
// External packages are called "crates" — like npm packages in JavaScript.
//
// To add a crate:
//   1. Add it to Cargo.toml under [dependencies]
//   2. Run `cargo build` — Cargo downloads it automatically
//   3. Use it in your code with `use`
//
// This file uses the `rand` crate as an example.
// Make sure Cargo.toml has:   rand = "0.8"
//
// ────────────────────────────────────────────────────────────────────────────

use rand::Rng;  // bring Rng trait into scope so we can call gen_range()

fn main() {
    // ── 1. Generate a random number ───────────────────────────────────────────
    let mut rng = rand::thread_rng();

    let number = rng.gen_range(1..=100);
    println!("Random number (1-100): {}", number);


    // ── 2. Random bool ────────────────────────────────────────────────────────
    let coin_flip: bool = rng.gen();
    println!("Coin flip: {}", if coin_flip { "Heads" } else { "Tails" });


    // ── 3. Random float ───────────────────────────────────────────────────────
    let float: f64 = rng.gen_range(0.0..1.0);
    println!("Random float: {:.4}", float);


    // ── 4. Shuffle a list ────────────────────────────────────────────────────
    use rand::seq::SliceRandom;
    let mut cards = vec!["Ace", "King", "Queen", "Jack", "10"];
    cards.shuffle(&mut rng);
    println!("\nShuffled cards: {:?}", cards);


    // ── Key Cargo commands ────────────────────────────────────────────────────
    //
    //   cargo new <name>          — create a new project
    //   cargo build               — compile the project
    //   cargo run                 — compile and run
    //   cargo run --bin <name>    — run a specific file
    //   cargo check               — check for errors without compiling
    //   cargo test                — run tests
    //   cargo add <crate>         — add a dependency to Cargo.toml
    //
    // ── Cargo.toml ───────────────────────────────────────────────────────────
    //
    //   [package]
    //   name    = "my-project"
    //   version = "0.1.0"
    //   edition = "2021"
    //
    //   [dependencies]
    //   rand    = "0.8"      ← external crate
    //   serde   = "1.0"      ← another crate
    //
    // ── crates.io ────────────────────────────────────────────────────────────
    //    All public Rust crates live at https://crates.io
    //    Search for any crate, see docs, copy the version to Cargo.toml
}
