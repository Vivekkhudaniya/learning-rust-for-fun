// ─── Concept 6: Memory Management ───────────────────────────────────────────
//
// Run this file with:   cargo run --bin memory_management
//
// Most languages (Python, JavaScript) use a "Garbage Collector" —
// a background process that automatically cleans up unused memory.
// The problem: it runs at random times and slows your program down.
//
// Rust has NO garbage collector.
// Instead, Rust cleans up memory automatically using simple rules —
// and it does this AT COMPILE TIME, not while the program is running.
// Result: fast programs with no surprise slowdowns.
//
// ────────────────────────────────────────────────────────────────────────────

fn main() {

    // ── 1. Memory is allocated when a variable is created ────────────────────
    //    When this line runs, Rust sets aside memory to store the number 10
    let x = 10;
    println!("x = {}", x);
    // At this point, memory is being used to hold the value 10


    // ── 2. Memory is freed when the variable goes out of scope ───────────────
    //    A "scope" is anything inside { }
    //    When the closing } is reached, ALL variables inside are deleted from memory
    {
        let temp = 99;  // memory is allocated here for temp
        println!("inside scope: temp = {}", temp);
    }
    // temp is GONE here — memory is already freed
    // println!("{}", temp);   // this would be an ERROR


    // ── 3. Heap memory is freed when the owner is done ───────────────────────
    //    Simple values like numbers live on the Stack (fast, small)
    //    Bigger values like String live on the Heap (flexible, larger)
    //    Rust frees Heap memory the moment the owner goes out of scope
    {
        let name = String::from("Vivek");  // memory allocated on the Heap
        println!("name = {}", name);
    }
    // name is GONE here — Heap memory is freed automatically
    // No garbage collector needed. Rust handled it.


    // ── 4. No dangling pointers ──────────────────────────────────────────────
    //    In languages like C, you can accidentally use memory that was already freed
    //    This causes crashes and bugs
    //    Rust PREVENTS this at compile time — it simply won't let your code compile
    //    if you try to use a variable after its memory is freed


    // ── 5. No memory leaks ───────────────────────────────────────────────────
    //    In some languages, memory is allocated but never freed — this is a "leak"
    //    Over time the program uses more and more memory and slows down or crashes
    //    In Rust, memory is ALWAYS freed when the owner goes out of scope
    //    So memory leaks are practically impossible


    println!("\nKey idea:");
    println!("Memory is allocated when a variable is created.");
    println!("Memory is freed when the variable goes out of scope.");
    println!("Rust does this automatically — no garbage collector needed.");
}
