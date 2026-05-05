// ─── Concept 8: Ownership ────────────────────────────────────────────────────
//
// Run this file with:   cargo run --bin ownership
//
// Ownership is Rust's most unique concept.
// It's how Rust manages memory without a garbage collector.
//
// 3 simple rules:
//   Rule 1 — Every value has exactly ONE owner
//   Rule 2 — There can only be one owner at a time
//   Rule 3 — When the owner goes out of scope, the value is dropped (freed)
//
// ────────────────────────────────────────────────────────────────────────────

fn main() {

    // ── 1. Every value has one owner ─────────────────────────────────────────
    //    Here, `name` is the owner of the String "Vivek"
    let name = String::from("Vivek");
    println!("Owner: {}", name);
    // When main() ends, name is gone and memory is freed automatically


    // ── 2. Ownership MOVES when you assign to another variable ───────────────
    //    Ownership transfers from s1 to s2
    //    After the move, s1 is no longer valid
    let s1 = String::from("hello");
    let s2 = s1;                        // ownership moved from s1 to s2

    // println!("{}", s1);              // ERROR — s1 no longer owns anything
    println!("s2 owns it now: {}", s2); // only s2 works


    // ── 3. Ownership MOVES into a function too ───────────────────────────────
    //    When you pass a String into a function, ownership moves INTO the function
    //    After the function call, you can no longer use the variable
    let my_string = String::from("Rust");
    takes_ownership(my_string);         // ownership moved into the function

    // println!("{}", my_string);       // ERROR — my_string was moved


    // ── 4. Functions can RETURN ownership back ───────────────────────────────
    //    A function can give ownership back by returning the value
    let s3 = gives_ownership();         // function creates a String and gives it to s3
    println!("s3 = {}", s3);           // s3 owns it now


    // ── 5. Numbers are COPIED, not moved ────────────────────────────────────
    //    Simple stack types (i32, f64, bool) are always copied
    //    Ownership rules don't apply to them
    let x = 5;
    let y = x;                          // x is COPIED into y, not moved
    println!("x = {}, y = {}", x, y);  // both work fine


    // ── 6. Clone — make a copy of Heap data ──────────────────────────────────
    //    If you need to keep the original AND have a copy, use .clone()
    let s4 = String::from("world");
    let s5 = s4.clone();               // makes a full separate copy

    println!("s4 = {}", s4);           // s4 still works
    println!("s5 = {}", s5);           // s5 also works
}


// This function takes ownership of the String passed to it
// When the function ends, the String is dropped from memory
fn takes_ownership(s: String) {
    println!("Got: {}", s);
}   // s is dropped here — memory freed


// This function creates a String and gives ownership to whoever calls it
fn gives_ownership() -> String {
    let s = String::from("given");
    s   // no semicolon = this is returned, ownership goes to the caller
}
