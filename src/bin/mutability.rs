// ─── Concept 3: Mutability ────────────────────────────────────────────────────
//
// Run this file with:   cargo run --bin mutability
//
// ────────────────────────────────────────────────────────────────────────────

fn main() {
    // ── 1. Immutable by default ──────────────────────────────────────────────
    //    In Rust, variables CANNOT be changed unless you say so
    //    This protects you from accidentally overwriting values
    let x = 5;
    println!("x = {}", x);
    // x = 10;  // ERROR: cannot assign twice to immutable variable

    // ── 2. Mutable variables ─────────────────────────────────────────────────
    //    Add `mut` to allow the value to change
    let mut y = 5;
    println!("y before: {}", y);
    y = 10;
    println!("y after:  {}", y);

    // ── 3. Shadowing ─────────────────────────────────────────────────────────
    //    You can re-declare a variable with the same name using `let` again
    //    This is called shadowing — it creates a brand new variable
    //    Unlike `mut`, shadowing can even change the type
    let z = 5;
    let z = z + 1;       // shadows the first z
    let z = z * 2;       // shadows again
    println!("z (shadowed) = {}", z);  // 12

    // shadowing can change the type — mut cannot
    let spaces = "   ";           // &str
    let spaces = spaces.len();    // now it's usize — totally fine with shadowing
    println!("spaces count: {}", spaces);

    // ── 4. Constants ─────────────────────────────────────────────────────────
    //    Declared with `const`, not `let`
    //    NEVER mutable — `mut` is not allowed with const
    //    Must always have a type annotation
    //    Can be declared in any scope, including global
    const MAX_POINTS: u32 = 100_000;
    println!("Max points: {}", MAX_POINTS);

    // ── 5. Mutability with compound types ────────────────────────────────────
    //    mut works the same way for Strings, Vecs, etc.
    let mut name = String::from("Vivek");
    println!("Name before: {}", name);
    name.push_str(" Khudaniya");
    println!("Name after:  {}", name);

    // ── 6. Why immutability matters ──────────────────────────────────────────
    //    Immutable by default = Rust forces you to be intentional about change
    //    If you see `mut`, you know that value is expected to change
    //    If you don't see `mut`, you can trust the value stays the same
    let total = 42;
    println!("Total (safe, won't change): {}", total);
}
