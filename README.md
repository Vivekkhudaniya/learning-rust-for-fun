# Learning Rust for Fun

Learning Rust from zero — one concept at a time, then build 3 real projects.

---

## Install Rust

Go to **https://rustup.rs** and follow the instructions for your computer.

Check it worked:
```bash
rustc --version
cargo --version
```

> **cargo** = Rust's built-in tool to create and run projects. Use `cargo run --bin <name>` to run each concept file.

---

## Part 1 — Learn the Concepts

Go through these in order. Each one builds on the previous.

---

### 1. Basic Syntax
- How a Rust program looks (`fn main()`)
- Printing to screen (`println!`)
- Comments

```rust
fn main() {
    println!("Hello, World!");
}
```

---

### 2. Variables and Data Types
- Creating variables with `let`
- Common types: `i32`, `f64`, `bool`, `String`, `&str`
- Rust can guess the type (type inference)

```rust
let name = "Vivek";
let age = 21;
let price = 9.99;
let is_learning = true;
```

---

### 3. Mutability
- By default, variables in Rust **cannot be changed**
- Add `mut` to allow changes
- Constants with `const` — never change, ever

```rust
let x = 5;          // cannot change
let mut y = 5;      // can change
y = 10;

const MAX: i32 = 100;   // constant
```

---

### 4. Conditionals and Loops
- `if` / `else if` / `else`
- `loop`, `while`, `for`
- `break` and `continue`

```rust
if age >= 18 {
    println!("adult");
}

for i in 1..=5 {
    println!("{}", i);
}
```

---

### 5. Functions
- Writing your own functions with `fn`
- Passing values in (parameters)
- Returning values with `->`

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b   // last line without semicolon = returned
}
```

---

### 6. Memory Management
- Most languages use a garbage collector to clean up memory
- Rust does NOT — it cleans up memory automatically using **ownership rules**
- No garbage collector = faster programs, no surprise pauses
- Memory is freed the moment the owner goes out of scope

---

### 7. Stack vs Heap
- **Stack** — fast, fixed size, stores simple values (numbers, booleans)
- **Heap** — slower, flexible size, stores complex values (Strings, Vecs)
- When you write `let x = 5` → goes on the Stack
- When you write `String::from("hello")` → goes on the Heap
- Ownership is all about managing Heap memory safely

---

### 8. Ownership
- Every value has exactly **one owner**
- When the owner goes away, the value is deleted from memory
- Ownership can **move** from one variable to another

```rust
let s1 = String::from("hello");
let s2 = s1;   // ownership moved to s2 — s1 no longer works

println!("{}", s2);   // ok
// println!("{}", s1);  // ERROR
```

---

### 9. Borrowing and References
- Borrow a value without taking ownership using `&`
- Mutable borrow with `&mut` — borrow AND change
- Only one mutable borrow allowed at a time

```rust
let s = String::from("hello");
let len = get_length(&s);   // s is borrowed, not moved
println!("{} has {} chars", s, len);   // s still works

fn get_length(s: &String) -> usize {
    s.len()
}
```

---

### 10. Structs
- Group related data together into one custom type
- Like creating your own data structure

```rust
struct Person {
    name: String,
    age: u32,
}

let p = Person { name: String::from("Vivek"), age: 21 };
println!("{} is {}", p.name, p.age);
```

---

### 11. Implementing Structs
- Add functions (methods) that belong to a struct using `impl`
- `self` refers to the struct itself (like `this` in other languages)

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

let rect = Rectangle { width: 10, height: 5 };
println!("Area: {}", rect.area());
```

---

### 12. Enums
- A type that can be one of several options
- Much more powerful than enums in other languages

```rust
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

let dir = Direction::Up;
```

---

### 13. Pattern Matching
- `match` = check which value something is and act on it
- Must handle every possible case (Rust forces you to)

```rust
match dir {
    Direction::Up    => println!("going up"),
    Direction::Down  => println!("going down"),
    _                => println!("going sideways"),  // _ = everything else
}
```

---

### 14. Error Handling
- Rust has no exceptions — errors are just values
- `Result<T, E>` — either success (`Ok`) or failure (`Err`)
- Use `match`, `unwrap()`, or `expect()` to handle them

```rust
let result: Result<i32, &str> = Ok(42);

match result {
    Ok(n)  => println!("Success: {}", n),
    Err(e) => println!("Error: {}", e),
}
```

---

### 15. Option Enum
- `Option<T>` — a value that might or might not exist
- Either `Some(value)` or `None`
- Rust's way of replacing `null` (which causes crashes in other languages)

```rust
let found: Option<i32> = Some(5);
let nothing: Option<i32> = None;

match found {
    Some(n) => println!("Found: {}", n),
    None    => println!("Nothing there"),
}
```

---

### 16. Cargo, Packages and External Dependencies
- **Cargo** = Rust's package manager (like npm for JavaScript)
- External packages are called **crates**
- Add them to `Cargo.toml` and Cargo downloads them automatically

```toml
# Cargo.toml
[dependencies]
rand = "0.8"   # adds the random number crate
```

```bash
cargo build   # downloads and builds dependencies
cargo run     # runs your project
```

---

## Part 2 — Build 3 Projects

Once all 16 concepts are done, we build these together.

| # | Project | Concepts Used |
|---|---|---|
| 1 | CLI Calculator | variables, functions, user input, match |
| 2 | To-Do List App | structs, impl, Vec, enums, file saving |
| 3 | Expense Tracker | structs, HashMap, error handling, Option |

---

## Part 3 — Backend with Axum

Once the 3 projects are done, we build a real backend using the Axum framework.

**Stack:**
| Tool | Purpose |
|---|---|
| Axum | Web framework — routes and handlers |
| Tokio | Async runtime |
| SQLx | Database (PostgreSQL) |
| Serde | JSON serialization |

| # | Project | What you'll build |
|---|---|---|
| 4 | REST API with Axum | Routes, handlers, JSON responses, PostgreSQL |

---

## My Progress

**Concepts:**
- [x] 1 — Basic Syntax
- [x] 2 — Variables and Data Types
- [x] 3 — Mutability
- [x] 4 — Conditionals and Loops
- [x] 5 — Functions
- [x] 6 — Memory Management
- [x] 7 — Stack vs Heap
- [x] 8 — Ownership
- [x] 9 — Borrowing and References
- [x] 10 — Structs
- [x] 11 — Implementing Structs
- [x] 12 — Enums
- [x] 13 — Pattern Matching
- [x] 14 — Error Handling
- [x] 15 — Option Enum
- [x] 16 — Cargo, Packages and External Dependencies

**Projects:**
- [x] Project 1 — CLI Calculator
- [ ] Project 2 — To-Do List App
- [ ] Project 3 — Expense Tracker

---

## Helpful Links

- [The Rust Book](https://doc.rust-lang.org/book/) — free official beginner guide
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/) — learn by reading short examples
- [Rustlings](https://github.com/rust-lang/rustlings) — small exercises to practice each concept
