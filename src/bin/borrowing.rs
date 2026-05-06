// ─── Concept 9: Borrowing and References ────────────────────────────────────
//
// Run this file with:   cargo run --bin borrowing
//
// Problem with ownership:
//   Every time you pass a String into a function, ownership moves.
//   After the call, you can no longer use that variable.
//   That's annoying — what if you want to use it again?
//
// Solution: Borrowing
//   Instead of giving ownership, you LEND the value using &
//   The function uses it and gives it back automatically when done.
//   The original owner keeps ownership the whole time.
//
// ────────────────────────────────────────────────────────────────────────────

fn main() {

    // ── 1. The problem without borrowing ─────────────────────────────────────
    //    Once you pass a String into a function, you lose it
    let name = String::from("Vivek");
    takes_ownership(name);
    // println!("{}", name);   // ERROR — name was moved into the function


    // ── 2. Borrowing with & (immutable reference) ─────────────────────────────
    //    & means "lend this value, don't give it away"
    //    The function gets a reference — it can READ but NOT change the value
    let city = String::from("Mumbai");
    print_city(&city);              // we pass &city, not city
    println!("Still have: {}", city);   // city still works — we only lent it


    // ── 3. Multiple borrows at the same time ──────────────────────────────────
    //    You can have as many & (read-only) borrows as you want at once
    let country = String::from("India");
    let r1 = &country;
    let r2 = &country;
    let r3 = &country;
    println!("{}, {}, {}", r1, r2, r3);  // all three work fine


    // ── 4. Mutable reference with &mut ────────────────────────────────────────
    //    If you want the function to also CHANGE the value, use &mut
    //    The variable itself must also be declared with mut
    let mut greeting = String::from("Hello");
    println!("Before: {}", greeting);
    add_name(&mut greeting);            // lend it AND allow changes
    println!("After:  {}", greeting);   // greeting now changed — and we still own it


    // ── 5. Only ONE mutable borrow at a time ──────────────────────────────────
    //    Rust allows EITHER:
    //      - many & borrows (read-only), OR
    //      - exactly ONE &mut borrow
    //    Never both at the same time — this prevents bugs where two parts
    //    of your code try to change the same value simultaneously

    let mut s = String::from("Rust");
    let m1 = &mut s;
    // let m2 = &mut s;             // ERROR — can't have two mutable borrows
    println!("{}", m1);


    // ── 6. References in function signatures ─────────────────────────────────
    //    &String  in the parameter = this function borrows, does NOT take ownership
    //    &mut String              = this function borrows AND can change it
    println!("\nLength of city: {}", get_length(&city));
    println!("city still works: {}", city);
}


// & means this function BORROWS the String — ownership stays with the caller
fn print_city(c: &String) {
    println!("City: {}", c);
}   // c goes out of scope here but since it's a borrow, nothing is dropped


// &mut means this function borrows AND can change the value
fn add_name(s: &mut String) {
    s.push_str(", Vivek!");   // push_str adds text to the end of a String
}


// This function just reads the String and returns its length
fn get_length(s: &String) -> usize {
    s.len()
}


// This function takes ownership — caller loses the value after calling this
fn takes_ownership(s: String) {
    println!("Got: {}", s);
}   // s is dropped here
