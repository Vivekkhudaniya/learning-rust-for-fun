// ─── Concept 7: Stack vs Heap ────────────────────────────────────────────────
//
// Run this file with:   cargo run --bin stack_vs_heap
//
// Your computer has memory (RAM). Rust uses two parts of it:
//
//   STACK                          HEAP
//   ─────────────────────          ──────────────────────
//   Fast                           Slower
//   Fixed size only                Any size (grows/shrinks)
//   Auto managed                   Managed by ownership rules
//   Numbers, bools, chars          Strings, Vecs, large data
//
// Think of it like this:
//   Stack = a stack of plates — you add/remove from the TOP only, very fast
//   Heap  = a big storage room — you find a free spot and put your stuff there
//
// ────────────────────────────────────────────────────────────────────────────

fn main() {

    // ── 1. Stack examples ─────────────────────────────────────────────────────
    //    These types have a FIXED, known size — they go on the Stack
    //    Fast to create, fast to delete, fast to copy

    let a: i32 = 10;        // whole number     → Stack
    let b: f64 = 3.14;      // decimal number   → Stack
    let c: bool = true;     // true/false        → Stack
    let d: char = 'R';      // single character → Stack

    println!("Stack values: {}, {}, {}, {}", a, b, c, d);

    // Stack values are COPIED when assigned to another variable
    let a2 = a;             // a is copied into a2
    println!("a = {}, a2 = {}", a, a2);  // both work fine


    // ── 2. Heap examples ─────────────────────────────────────────────────────
    //    These types can grow or shrink — they go on the Heap
    //    Rust stores a small "pointer" on the Stack that points to the Heap data

    let name = String::from("Vivek");   // text of unknown length → Heap
    let numbers = vec![1, 2, 3, 4, 5]; // a list that can grow   → Heap

    println!("Heap values: {}, {:?}", name, numbers);

    // Heap values are MOVED, not copied
    let name2 = name;               // ownership moves to name2
    // println!("{}", name);        // ERROR — name no longer owns the data
    println!("name2 = {}", name2);  // only name2 works now


    // ── 3. What actually lives on the Stack for a String ─────────────────────
    //    When you create a String, Rust puts 3 small things on the Stack:
    //      1. pointer  — the address of where the text is on the Heap
    //      2. length   — how many characters are currently in it
    //      3. capacity — how much space is reserved on the Heap
    //
    //    The actual text characters live on the Heap

    let s = String::from("hello");
    println!("\nString: {}", s);
    println!("Length: {}", s.len());        // number of characters
    println!("Capacity: {}", s.capacity()); // space reserved on heap


    // ── 4. Clone — copy Heap data ────────────────────────────────────────────
    //    If you need TWO separate copies of Heap data, use .clone()
    //    This makes a full copy on the Heap — both variables work independently

    let original = String::from("Rust");
    let copy = original.clone();    // full copy made on the Heap

    println!("\noriginal = {}", original);  // still works
    println!("copy     = {}", copy);        // also works


    // ── 5. Summary ───────────────────────────────────────────────────────────
    println!("\n--- Summary ---");
    println!("Stack → fixed size types (i32, f64, bool, char) → fast, auto copied");
    println!("Heap  → flexible size types (String, Vec) → slower, ownership rules apply");
}
