// ─── Concept 1: Basic Syntax ───────────────────────────────────────────────
//
// Run this file with:   cargo run --bin basic_syntax
//
// ────────────────────────────────────────────────────────────────────────────

fn main() {
    // 1. Printing to the screen
    //    println! is a macro (notice the !)
    //    Macros are like functions but more powerful — you'll see them a lot in Rust
    println!("Hello, World!");

    // 2. Printing with a value inside
    //    {} is a placeholder — it gets replaced by the value after the comma
    let name = "Devesh";
    println!("Hello, {}!", name);

    // 3. Printing multiple values
    let x = 5;
    let y = 10;
    println!("{} + {} = {}", x, y, x + y);

    // 4. Comments
    //    Single line comment  →  //
    //    Multi line comment   →  /* ... */

    /* This is a
       multi-line comment */

    // 5. Every Rust program starts from fn main()
    //    fn     = keyword to define a function
    //    main   = the name (this one is special — it's where the program begins)
    //    ()     = no inputs
    //    { }    = the body of the function (your code goes here)

    println!("This is my first Rust program!");
}
