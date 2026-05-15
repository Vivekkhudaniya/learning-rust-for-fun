// ─── Interview Question 1: Ownership & Borrowing ─────────────────────────────
//
// Run this file with:   cargo run --bin q1_ownership
//
// ────────────────────────────────────────────────────────────────────────────

fn main() {

    // ══════════════════════════════════════════════════════════════════════════
    // PART 1 — OWNERSHIP
    // ══════════════════════════════════════════════════════════════════════════

    // ── Rule 1: Every value has exactly one owner ─────────────────────────────
    let name = String::from("Vivek");   // `name` is the owner of this String

    // ── Rule 2: When the owner goes out of scope, value is dropped ────────────
    {
        let temp = String::from("I am temporary");
        println!("Inside block: {}", temp);
    }
    // `temp` is gone here — memory is already freed
    // println!("{}", temp);   // ERROR: temp no longer exists

    // ── Rule 3: Ownership can be MOVED ───────────────────────────────────────
    let s1 = String::from("hello");
    let s2 = s1;                        // ownership MOVES from s1 to s2
    // println!("{}", s1);              // ERROR: s1 no longer owns the value
    println!("s2 owns the value: {}", s2);

    // ── Copy types don't move — they duplicate ────────────────────────────────
    //    Simple types like i32, bool, f64 are copied automatically
    let x = 5;
    let y = x;                          // x is COPIED, not moved
    println!("x = {}, y = {}", x, y);  // both work fine


    // ══════════════════════════════════════════════════════════════════════════
    // PART 2 — BORROWING
    // ══════════════════════════════════════════════════════════════════════════

    // Problem: if we pass a value to a function, ownership moves into it
    // and we can't use the value afterwards.
    //
    // Solution: BORROW it instead of giving it away.

    // ── Immutable borrow (&) — read only ──────────────────────────────────────
    //    Many borrows allowed at the same time
    let city = String::from("Mumbai");

    let len = get_length(&city);        // lend city to the function, don't give it
    println!("City: {}, length: {}", city, len);  // city still works!

    // ── Mutable borrow (&mut) — read and write ────────────────────────────────
    //    Only ONE mutable borrow allowed at a time
    let mut greeting = String::from("Hello");
    add_name(&mut greeting);            // lend greeting, allow changes
    println!("{}", greeting);           // greeting has the updated value


    // ── The golden rule of borrowing ─────────────────────────────────────────
    //
    //   At any given time, you can have EITHER:
    //     - Any number of immutable borrows (&)   — multiple readers
    //     - Exactly ONE mutable borrow (&mut)     — one writer
    //   But NEVER both at the same time.
    //
    //   This is how Rust prevents data races at compile time.

    let mut score = String::from("100");

    let r1 = &score;                    // immutable borrow — ok
    let r2 = &score;                    // second immutable borrow — also ok
    println!("r1: {}, r2: {}", r1, r2);
    // r1 and r2 are done being used here

    let r3 = &mut score;               // mutable borrow — ok now (r1, r2 are done)
    r3.push_str(" points");
    println!("score: {}", r3);

    println!("name is still: {}", name);
}


// Takes a borrow (&String) — does NOT take ownership
fn get_length(s: &String) -> usize {
    s.len()
}   // s goes out of scope here, but since it's just a borrow, nothing is dropped


// Takes a mutable borrow (&mut String) — can change the value
fn add_name(s: &mut String) {
    s.push_str(", Vivek!");
}
