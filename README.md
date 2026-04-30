# Learning Rust for Fun

Learning Rust from zero — first understand the concepts, then build 3 real projects.

---

## Install Rust

Go to **https://rustup.rs** and follow the instructions for your computer.

Check it worked:
```bash
rustc --version
cargo --version
```

> **cargo** = Rust's built-in tool to create and run projects. You'll use `cargo run` to run your code every time.

---

## Part 1 — Learn the Concepts

Go through these one by one. Don't skip — each one builds on the previous.

---

### 1. Basic Syntax
- How a Rust program looks (`fn main()`)
- Printing to screen (`println!`)
- Running with `cargo run`

```rust
fn main() {
    println!("Hello, World!");
}
```

---

### 2. Variables and Data Types
- Creating variables (`let`)
- Mutable vs immutable (`let mut`)
- Common types: numbers (`i32`, `f64`), text (`String`, `&str`), true/false (`bool`)

```rust
let name = "Devesh";         // can't change
let mut age = 20;            // can change
age = 21;
```

---

### 3. Functions
- Writing your own functions
- Passing values in (parameters)
- Getting values back (return types)

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b   // no semicolon = this is returned
}
```

---

### 4. Control Flow
- `if` / `else if` / `else`
- `loop`, `while`, `for`
- `break` and `continue`

```rust
if age >= 18 {
    println!("adult");
} else {
    println!("minor");
}

for i in 1..=5 {
    println!("{}", i);  // prints 1 2 3 4 5
}
```

---

### 5. Ownership (Rust's most unique concept)
This is what makes Rust different from every other language. Take your time here.

- Every value has one **owner**
- When the owner is gone, the value is gone (no garbage collector needed)
- **Borrowing** — let someone use a value without taking ownership (`&`)

```rust
let s1 = String::from("hello");
let s2 = &s1;   // borrowing — s1 still owns it
println!("{}", s2);
```

> This will feel confusing at first. That's normal. Just keep writing code and it clicks.

---

### 6. Structs
- Group related data together
- Like a custom data type you design

```rust
struct Person {
    name: String,
    age: u32,
}

let p = Person { name: String::from("Devesh"), age: 21 };
println!("{}", p.name);
```

---

### 7. Enums and Pattern Matching
- Enums = a value that can be one of several things
- `match` = check which one it is and act on it

```rust
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

let dir = Direction::Up;

match dir {
    Direction::Up    => println!("going up"),
    Direction::Down  => println!("going down"),
    _                => println!("going sideways"),
}
```

---

### 8. Error Handling
- Rust has no exceptions — errors are just values
- `Option<T>` — something that might or might not exist
- `Result<T, E>` — something that might succeed or fail
- `unwrap()`, `expect()`, `match` to handle them

```rust
let result: Option<i32> = Some(5);

match result {
    Some(n) => println!("got {}", n),
    None    => println!("got nothing"),
}
```

---

### 9. Collections
- `Vec<T>` — a list that can grow and shrink
- `HashMap<K, V>` — store key-value pairs (like a dictionary)

```rust
let mut nums: Vec<i32> = Vec::new();
nums.push(1);
nums.push(2);
nums.push(3);
println!("{:?}", nums);  // [1, 2, 3]
```

---

### 10. Closures and Iterators
- Closures = small inline functions (like arrow functions)
- Iterators = loop over collections in a clean way

```rust
let nums = vec![1, 2, 3, 4, 5];

let doubled: Vec<i32> = nums.iter()
    .map(|x| x * 2)
    .collect();

println!("{:?}", doubled);  // [2, 4, 6, 8, 10]
```

---

## Part 2 — Build 3 Projects

Once you're done with the concepts above, build these 3 projects together.

| # | Project | What It Uses |
|---|---|---|
| 1 | CLI Calculator | variables, functions, user input, match |
| 2 | To-Do List App | structs, Vec, enums, loops, file saving |
| 3 | Expense Tracker | structs, HashMap, error handling, functions |

---

## My Progress

**Concepts:**
- [ ] 1 — Basic Syntax
- [ ] 2 — Variables and Data Types
- [ ] 3 — Functions
- [ ] 4 — Control Flow
- [ ] 5 — Ownership
- [ ] 6 — Structs
- [ ] 7 — Enums and Pattern Matching
- [ ] 8 — Error Handling
- [ ] 9 — Collections
- [ ] 10 — Closures and Iterators

**Projects:**
- [ ] Project 1 — CLI Calculator
- [ ] Project 2 — To-Do List App
- [ ] Project 3 — Expense Tracker

---

## Helpful Links

- [The Rust Book](https://doc.rust-lang.org/book/) — free official beginner guide
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/) — learn by reading short examples
- [Rustlings](https://github.com/rust-lang/rustlings) — small exercises to practice each concept
