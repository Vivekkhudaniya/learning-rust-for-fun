# Rust Mock Interview Questions & Answers

---

## Rust Basics

### Ownership & Borrowing

**Q1. What is ownership in Rust?**
Ownership is Rust's way of managing memory without a garbage collector. Every value has exactly one owner. When the owner goes out of scope, the value is automatically dropped (memory freed). There are 3 rules:
1. Every value has exactly one owner
2. When the owner goes out of scope, the value is dropped
3. Ownership can be moved or borrowed

**Q2. What happens when you assign one variable to another?**
For heap types like `String`, ownership **moves** — the original variable becomes invalid.
For simple types like `i32`, the value is **copied** — both variables remain valid.
```rust
let s1 = String::from("hello");
let s2 = s1;          // ownership moved — s1 is now invalid
// println!("{}", s1) // ERROR

let x = 5;
let y = x;            // copied — both x and y work fine
```

**Q3. What is the difference between `&` and `&mut`?**
- `&` is an immutable (shared) borrow — you can read the value but not change it. Multiple `&` borrows can exist at the same time.
- `&mut` is a mutable (exclusive) borrow — you can read and change the value. Only one `&mut` can exist at a time, and no `&` can exist alongside it.

**Q4. How many mutable borrows can exist at the same time?**
Only **one**. And when a mutable borrow exists, no immutable borrows can exist at the same time. This prevents data races at compile time.

**Q5. What is the difference between `Copy` and `Clone`?**
- `Copy` is **implicit** and automatic — happens on assignment. Only works for simple types that live on the stack (i32, bool, f64, char). No heap involved.
- `Clone` is **explicit** — you have to call `.clone()`. Can be expensive as it may allocate heap memory. Works for any type that implements `Clone`.
```rust
let x = 5;
let y = x;         // Copy — automatic, free

let s1 = String::from("hi");
let s2 = s1.clone(); // Clone — explicit, allocates
```

---

### Core Concepts

**Q6. What is the difference between `String` and `&str`?**
- `String` — heap allocated, owned, growable. You own it and can change it.
- `&str` — a borrowed reference/view into a string. Fixed, cannot be changed. String literals like `"hello"` are `&str`.

Rule of thumb: use `&str` for function parameters, use `String` when you need to own or modify the text.

**Q7. What is `Option<T>` and why does Rust use it instead of null?**
`Option<T>` is an enum that represents a value that may or may not exist:
- `Some(value)` — a value exists
- `None` — no value

Rust uses it instead of `null` because null in other languages causes crashes (null pointer exceptions). With `Option`, the compiler **forces** you to handle both cases — you can never accidentally use a null value.

**Q8. What is `Result<T, E>` and when do you use it?**
`Result<T, E>` is an enum for operations that can fail:
- `Ok(value)` — success
- `Err(e)` — failure with an error

Use it when something can go wrong — reading a file, parsing input, calling a database. It forces the caller to handle the error instead of ignoring it.

**Q9. What is pattern matching and how does `match` work?**
`match` checks a value against a list of patterns and runs the code for the first one that matches. Rust forces you to handle every possible case — if you miss one, it won't compile.
```rust
match score {
    90..=100 => "A",
    80..=89  => "B",
    _        => "F",   // _ = everything else
}
```

**Q10. What is the difference between `unwrap()`, `expect()`, and the `?` operator?**
- `unwrap()` — extracts the value from `Ok`/`Some`, crashes the program if it's `Err`/`None`
- `expect("message")` — same as `unwrap` but shows your custom message when it crashes. Better for debugging.
- `?` — propagates the error up to the caller instead of crashing. Cleaner way to handle errors in functions that return `Result`.

**Q11. What is a struct and what is `impl`?**
A `struct` groups related data together into one custom type. `impl` lets you attach methods (functions) to that struct.
```rust
struct Person { name: String, age: u32 }

impl Person {
    fn greet(&self) {
        println!("Hi, I'm {}", self.name);
    }
}
```

**Q12. What is an enum and how is it different from other languages?**
An enum defines a type that can be one of several fixed options. In Rust, each variant can also carry its own data — making it much more powerful than enums in Java or C.
```rust
enum Payment {
    Cash(f64),
    Card(String),
    UPI { id: String },
}
```

**Q13. What is mutability — difference between `let` and `let mut`?**
- `let` — variable is immutable by default, cannot be changed after assignment
- `let mut` — variable is mutable, can be changed

Rust defaults to immutable to make code safer and more predictable.

**Q14. What is shadowing?**
Shadowing means re-declaring a variable with the same name using `let`. Unlike `mut`, shadowing creates a brand new variable and can even change the type.
```rust
let x = 5;
let x = x + 1;      // shadows the first x — new variable
let x = "hello";    // can even change the type
```

---

### Memory

**Q15. Stack vs Heap — what goes where?**
- **Stack** — fast, fixed size. Simple values like numbers, booleans, and structs of known size go here.
- **Heap** — slower, flexible size. Complex types like `String`, `Vec`, `Box` go here. They store a small header on the stack (pointer + length) pointing to data on the heap.

**Q16. How does Rust manage memory without a garbage collector?**
Through ownership. Every value has one owner. When the owner goes out of scope, Rust automatically calls `drop()` and frees the memory. No garbage collector needed — it's all determined at compile time.

**Q17. When is memory freed in Rust?**
The moment the owner goes out of scope. Rust inserts the cleanup code at compile time — so memory is freed immediately and deterministically, with no pauses or GC overhead.

---

## Rust Backend (Axum)

**Q18. What is Tokio and why do we need it?**
Tokio is an async runtime for Rust. A backend server needs to handle many requests at the same time. Tokio lets your code do multiple things concurrently — while waiting for a database response, it handles other requests instead of sitting idle.

**Q19. What does `async/await` do?**
`async` marks a function as asynchronous — it returns a `Future` instead of running immediately. `await` pauses the current task until the future completes, but lets other tasks run in the meantime. Together they let you write concurrent code that looks sequential.

**Q20. What is `#[tokio::main]`?**
It's a macro that sets up the Tokio async runtime and makes your `main` function async. Without it, you can't use `async/await` in `main`.

**Q21. What is Axum and how do you define a route?**
Axum is a web framework built on top of Tokio for building HTTP APIs. You define routes using a `Router`:
```rust
let app = Router::new()
    .route("/users", get(get_users))
    .route("/users", post(create_user));
```

**Q22. What is a handler function in Axum?**
A handler is an async function that handles a specific route. It receives extractors as parameters (like JSON body, path params) and returns a response.
```rust
async fn get_users() -> Json<Vec<User>> {
    Json(vec![...])
}
```

**Q23. What is `Json<T>` in Axum?**
`Json<T>` is both an extractor and a response type. As a parameter it deserializes the request body from JSON into type `T`. As a return type it serializes `T` into a JSON response.

**Q24. What is the `Path` extractor in Axum?**
`Path` extracts values from the URL path. For a route like `/users/:id`, you use `Path(id): Path<u32>` to get the id from the URL.

**Q25. What is Serde and why do we use `#[derive(Serialize, Deserialize)]`?**
Serde is a library for serializing and deserializing data. Adding `#[derive(Serialize, Deserialize)]` to a struct automatically generates the code to convert it to/from JSON. Without it, Axum can't convert your structs to JSON responses.

**Q26. Write a basic GET route in Axum.**
```rust
async fn get_users() -> Json<Vec<User>> {
    let users = vec![
        User { id: 1, name: String::from("Vivek") }
    ];
    Json(users)
}

let app = Router::new().route("/users", get(get_users));
```

**Q27. Write a POST route that accepts a JSON body.**
```rust
async fn create_user(Json(body): Json<CreateUser>) -> Json<User> {
    let user = User { id: 1, name: body.name };
    Json(user)
}

let app = Router::new().route("/users", post(create_user));
```

**Q28. How do you extract a path parameter like `/users/:id`?**
```rust
async fn get_user(Path(id): Path<u32>) -> Json<User> {
    // id is now available as a u32
    Json(User { id, name: String::from("Vivek") })
}

let app = Router::new().route("/users/:id", get(get_user));
```

---

## General Backend

**Q29. Tell me about Rust — why would you choose it over Python or JavaScript for a backend?**
Rust gives you performance close to C with memory safety guaranteed at compile time. There's no garbage collector so there are no surprise pauses — latency is predictable. You also catch an entire class of bugs (null pointers, data races, use-after-free) before the code even runs.

Python and JavaScript are easier to write and faster to prototype in, but they have a GC, are slower, and many bugs only show up at runtime. For a high-traffic backend where performance and reliability matter, Rust is a strong choice. For a quick internal tool or MVP, Python or JavaScript is probably faster to ship.
