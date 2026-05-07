# Rust: Most Common Questions, Answered

A practical reference covering the questions Rust developers ask most often. Code snippets are minimal but runnable; explanations focus on the *why* behind each answer.

---

## 1. Ownership & Borrowing

### What is ownership and how does it work?

Every value in Rust has a single **owner** — a variable responsible for cleaning it up. When the owner goes out of scope, the value is dropped (memory freed, files closed, etc.). The three rules:

1. Each value has exactly one owner.
2. When the owner goes out of scope, the value is dropped.
3. Ownership can be **moved** (transferred) or **borrowed** (referenced temporarily).

```rust
let s1 = String::from("hello");
let s2 = s1;            // ownership MOVED to s2
// println!("{}", s1);  // ❌ compile error: s1 no longer valid
```

This eliminates whole classes of bugs (use-after-free, double-free) at compile time, with no garbage collector.

### What's the difference between `&`, `&mut`, and owned values?

| Form | Meaning | Rule |
|---|---|---|
| `T` | Owned value | You can do anything with it |
| `&T` | Shared (immutable) reference | Many readers allowed simultaneously |
| `&mut T` | Exclusive (mutable) reference | Only one at a time, no shared refs alongside |

The borrow checker enforces: *either* one mutable reference *or* any number of shared references — never both. This is how Rust prevents data races at compile time.

### Why does the borrow checker reject my code?

The most common causes:

- **Holding a shared reference while trying to mutate** — e.g., iterating a `Vec` while pushing to it.
- **Returning a reference to a local** — the local dies when the function returns; the reference would dangle.
- **Mutable + shared overlap** — storing a `&mut` somewhere while a `&` to the same data still exists.

Fixes usually fall into three buckets: (a) restructure so borrows don't overlap, (b) clone if cheap, (c) use interior mutability (`RefCell`, `Mutex`) when the rule genuinely needs to be deferred to runtime.

### What's the difference between `Copy` and `Clone`?

- `Clone` is **explicit** and can be expensive — `s.clone()` may allocate.
- `Copy` is **implicit** and must be cheap — bitwise copy, no heap. Assignment duplicates instead of moving.

Primitives (`i32`, `bool`, `char`, etc.) are `Copy`. `String`, `Vec<T>`, `Box<T>` are not — they own heap data, so duplicating them requires an allocation, which Rust refuses to do silently.

A type can only be `Copy` if all its fields are `Copy`. Every `Copy` type must also be `Clone`.

### When should I use `Rc`, `Arc`, `RefCell`, or `Mutex`?

| Type | Sharing | Mutation | Thread-safe |
|---|---|---|---|
| `Rc<T>` | Multiple owners | No | ❌ |
| `Arc<T>` | Multiple owners | No | ✅ |
| `RefCell<T>` | Single owner | Interior, runtime-checked | ❌ |
| `Mutex<T>` / `RwLock<T>` | Single owner | Interior, locked | ✅ |

Common combos:
- `Rc<RefCell<T>>` — shared mutable state, single-threaded (graph nodes, etc.).
- `Arc<Mutex<T>>` — shared mutable state across threads.

Reach for these only when the borrow checker genuinely can't see what you're doing. For most code, plain `&` and `&mut` are enough.

### How do I avoid fighting the borrow checker?

- **Smaller scopes.** Introduce blocks `{ ... }` so borrows end sooner.
- **Take what you need, then return.** Compute the value, end the borrow, then mutate.
- **Index instead of holding references** when iterating-and-mutating a collection.
- **Use `split_at_mut` / `iter_mut`** for getting multiple mutable references into disjoint parts of one collection.
- **Clone strategically.** A `.clone()` in a hot path is bad; in setup code it's often fine.

---

## 2. Lifetimes

### What are lifetimes and why do I need them?

A lifetime is a compile-time label saying "this reference is valid for at least this region of code." The compiler uses lifetimes to ensure no reference outlives the data it points to.

You don't usually *write* lifetimes — the compiler infers them via **lifetime elision rules**. You only annotate when the compiler can't figure out the relationship between input and output references.

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```

The `'a` says: the returned reference lives at least as long as the shorter of `x` and `y`.

### What does `'static` mean?

`'static` means "valid for the entire program." Two distinct uses:

- **Reference lifetime**: `&'static str` — string literals have this lifetime because they're baked into the binary.
- **Trait bound**: `T: 'static` — the type contains no non-`'static` references. This does *not* mean the value lives forever; an owned `String` is `'static` because it borrows nothing.

Most "needs `'static`" errors in async code are about the second meaning: tasks may outlive the function spawning them, so they can't borrow locals.

### How do I annotate lifetimes in structs and functions?

```rust
struct Excerpt<'a> {
    text: &'a str,
}

impl<'a> Excerpt<'a> {
    fn text(&self) -> &str { self.text }   // elided
}
```

Any struct holding a reference must declare its lifetime. The struct can't outlive the data it points to.

### Why does the compiler say my reference doesn't live long enough?

You're holding a reference to something that will be dropped before you stop using it. Common causes:

- Returning `&` to a local variable.
- Storing a reference in a struct that outlives the referent.
- Async tasks borrowing local variables (the task may outlive the scope).

Fix: extend the data's lifetime (move it into the struct/task by value), or restructure so the reference isn't needed.

---

## 3. Strings

### What's the difference between `String` and `&str`?

- `String` — heap-allocated, growable, **owned**.
- `&str` — a borrowed view (pointer + length) into a `String`, a literal, or any UTF-8 byte sequence.

Rule of thumb: take `&str` as a function parameter, return `String` when ownership is needed.

```rust
fn greet(name: &str) -> String {
    format!("Hello, {name}!")
}
```

### How do I convert between them?

```rust
let owned: String = String::from("hi");
let borrowed: &str = &owned;          // String → &str (deref coercion)
let owned2: String = borrowed.to_string();  // &str → String
let bytes: Vec<u8> = owned.into_bytes();    // String → Vec<u8>
let back = String::from_utf8(bytes).unwrap();  // Vec<u8> → String (validates UTF-8)
```

### Why can't I index into a string with `s[0]`?

Rust strings are UTF-8. `s[0]` is ambiguous: do you want the first byte, the first scalar value, or the first grapheme? These differ for non-ASCII text. The compiler refuses the choice.

Use the appropriate iterator:

```rust
s.bytes().next();      // u8 — first byte
s.chars().next();      // char — first scalar value
s.get(0..4);           // Option<&str> — slice by byte range, panics on non-boundary
```

For graphemes ("👨‍👩‍👧" is one grapheme made of multiple chars), use the `unicode-segmentation` crate.

### How do I split, trim, or parse strings?

```rust
let parts: Vec<&str> = "a,b,c".split(',').collect();
let trimmed = "  hi  ".trim();
let n: i32 = "42".parse().unwrap();           // explicit type required
let n: Result<i32, _> = "42".parse();         // or handle the error
```

---

## 4. Error Handling

### When should I use `Result` vs `Option` vs `panic!`?

- `Option<T>` — value may be absent, but absence isn't an error (e.g., `HashMap::get`).
- `Result<T, E>` — operation may fail with explanation (I/O, parsing, network).
- `panic!` — invariant violated; the program is in an undefined state and can't continue safely.

Library code should almost never panic on bad input — return `Result`. Application code can panic for genuinely impossible states.

### What does the `?` operator do?

It propagates errors. `expr?` expands to roughly:

```rust
match expr {
    Ok(v) => v,
    Err(e) => return Err(From::from(e)),
}
```

So `let file = File::open("foo")?;` either binds the file or early-returns the error. Works on `Option` too. The `From` conversion is what lets you mix error types — as long as `From<InnerError> for OuterError` is implemented.

### How do I create custom error types? `thiserror` vs `anyhow`?

- **`thiserror`** — for **libraries**. You define a structured enum of error variants that callers can match on.
- **`anyhow`** — for **applications**. One opaque `anyhow::Error` type that wraps anything, with context chaining. Quick and ergonomic when you don't need callers to programmatically distinguish errors.

```rust
// library — thiserror
#[derive(thiserror::Error, Debug)]
pub enum DbError {
    #[error("not found")]
    NotFound,
    #[error("io: {0}")]
    Io(#[from] std::io::Error),
}

// application — anyhow
fn run() -> anyhow::Result<()> {
    let cfg = std::fs::read_to_string("config.toml")
        .context("reading config")?;
    Ok(())
}
```

### How do I convert between error types?

Implement `From<InnerError> for OuterError` (or use `#[from]` in `thiserror`). Then `?` will convert automatically.

---

## 5. Traits & Generics

### What's the difference between `impl Trait` and `dyn Trait`?

- `impl Trait` — **static dispatch**, monomorphized at compile time. One concrete type per call site, zero runtime cost.
- `dyn Trait` — **dynamic dispatch** through a vtable. One pointer at runtime, enables heterogeneous collections.

```rust
fn make_iter() -> impl Iterator<Item = i32> { 0..10 }     // one concrete type
fn make_box() -> Box<dyn Iterator<Item = i32>> {          // any type that fits
    Box::new(0..10)
}
```

Use `impl Trait` by default. Use `dyn Trait` when you need a `Vec<Box<dyn Trait>>` of different types, or when generics would explode binary size.

### What are trait objects and when should I use them?

A trait object is `dyn Trait` behind a pointer (`&dyn`, `Box<dyn>`, `Arc<dyn>`). It erases the concrete type, allowing runtime polymorphism. Use them for:

- Heterogeneous collections: `Vec<Box<dyn Drawable>>`.
- Plugin systems where types aren't known at compile time.
- Reducing monomorphization bloat in generic-heavy code.

Trait objects require the trait to be **object-safe** — no generic methods, no `Self` in return position (with some exceptions).

### What's the difference between static and dynamic dispatch?

Static dispatch picks the function at compile time (generics, `impl Trait`). The compiler generates one specialized version per type — fast but bigger binaries. Dynamic dispatch picks at runtime via a vtable lookup — one extra indirection, but one binary path for all types.

In practice the perf difference is usually negligible; choose based on flexibility needs.

### How do associated types differ from generic parameters?

- **Generic parameter**: caller picks the type. A trait can be implemented many times with different parameters.
- **Associated type**: implementation picks the type. One implementation per type.

`Iterator` uses an associated `Item` because each iterator has exactly one item type — `impl Iterator<Item = i32> for Counter`. If `Item` were a generic, you could implement `Iterator<i32>` and `Iterator<String>` for `Counter`, which is rarely what you want.

### What is the orphan rule?

You can implement trait `T` for type `U` only if **either** `T` or `U` is defined in your crate. Without this rule, two crates could implement the same trait for the same foreign type with conflicting behavior, and the linker would have no principled way to resolve it.

Workarounds: define a wrapper newtype (`struct MyVec(Vec<T>)`) and implement on the wrapper.

---

## 6. Async & Concurrency

### What's the difference between `async`/`await` and threads?

- **Threads** are OS-level, preemptively scheduled, ~1 MB stack each. Good for CPU-bound work.
- **`async`/`await`** is cooperative, single-stack futures multiplexed onto a small thread pool. Good for I/O-bound work where most time is spent waiting (network, disk).

`async fn foo() -> T` returns a `Future<Output = T>` — a state machine that does nothing until polled by an executor.

### Tokio vs async-std — which should I use?

**Tokio**, in 2026. async-std is essentially unmaintained and most of the async ecosystem (axum, reqwest, sqlx, tonic) targets Tokio. For new projects, the choice is Tokio + smol for niche cases.

### Why does my future need to be `Send`?

Multi-threaded executors (Tokio's default) may move a future between worker threads at await points. For that to be safe, all data held across `.await` must be `Send`. Common culprits:

- `Rc<T>` held across await — use `Arc<T>`.
- `RefCell<T>` held across await — use `Mutex<T>` (or `tokio::sync::Mutex` if locked across await).
- `MutexGuard` held across await — drop it before awaiting, or use the async mutex.

### How do I share state between async tasks?

```rust
let state = Arc::new(Mutex::new(0));

for _ in 0..10 {
    let state = Arc::clone(&state);
    tokio::spawn(async move {
        let mut n = state.lock().await;
        *n += 1;
    });
}
```

For frequent reads, use `RwLock`. For message-passing instead of shared state, use `tokio::sync::mpsc` channels — often cleaner.

### What's `Pin` and why do I need it?

Some futures are **self-referential** — they contain pointers into their own state. Moving them in memory would invalidate those pointers. `Pin<&mut T>` is the type-system promise: "this won't move."

You almost never construct `Pin` yourself. You hit it when implementing `Future` by hand or using libraries like `pin-project`. For day-to-day async code, the `async fn` syntax handles it invisibly.

---

## 7. Cargo & Tooling

### How do workspaces work?

A workspace groups multiple crates with a shared `Cargo.lock` and `target/` directory. Top-level `Cargo.toml`:

```toml
[workspace]
members = ["app", "lib-core", "lib-utils"]
resolver = "2"
```

Run `cargo build` from the root to build everything; `cargo build -p lib-core` for one member. Great for splitting a large project without juggling versions.

### `cargo build` vs `cargo build --release`?

- `cargo build` — debug profile. No optimizations, fast compile, debug symbols, runs slow.
- `cargo build --release` — release profile. Full optimizations, slow compile, runs 10–100× faster.

Always benchmark and ship release. Debug builds can be misleadingly slow.

### How do I manage features and conditional compilation?

```toml
[features]
default = ["json"]
json = ["dep:serde_json"]
postgres = ["dep:tokio-postgres"]
```

```rust
#[cfg(feature = "json")]
pub fn to_json(&self) -> String { /* ... */ }
```

Features should be **additive** — enabling one shouldn't break consumers who didn't enable it.

### How do I publish a crate to crates.io?

```bash
cargo login                # paste token from crates.io
cargo publish --dry-run    # check
cargo publish              # ship it
```

Requires `name`, `version`, `description`, `license` in `Cargo.toml`. Versions are immutable once published; you can `yank` but not delete.

---

## 8. Smart Pointers & Memory

### When do I use `Box<T>`?

- **Recursive types** — a struct that contains itself needs indirection: `enum List { Cons(i32, Box<List>), Nil }`.
- **Trait objects** — `Box<dyn Trait>`.
- **Large values** — keep them off the stack.
- **Transferring ownership of heap data** without copying.

### What's the difference between the stack and heap in Rust?

- **Stack** — fixed-size values, automatically managed, very fast. Function locals, primitives, structs of known size.
- **Heap** — dynamically sized or longer-lived data, allocated via `Box`, `Vec`, `String`, `Rc`, etc. Slightly slower but flexible.

Rust gives you precise control: a `Vec<T>` is a stack-allocated header (pointer + length + capacity) pointing to a heap buffer.

### How does `Drop` work?

`Drop` is Rust's destructor. When a value goes out of scope, its `drop` method runs — closing files, freeing memory, releasing locks. You implement it for cleanup logic:

```rust
impl Drop for Connection {
    fn drop(&mut self) { self.close(); }
}
```

You can't call `drop` directly on a value (the compiler would still try to drop it again). To drop early, use `std::mem::drop(value)`.

---

## 9. Comparisons

### Rust vs Go — which should I learn?

- **Go** — simpler language, GC, brilliant tooling for concurrent network services. Optimized for team productivity.
- **Rust** — no GC, stronger type system, predictable performance, harder learning curve. Optimized for correctness and control.

Pick Go for backend APIs and microservices where dev velocity wins. Pick Rust when you need predictable latency, embedded targets, systems work, or want stronger compile-time guarantees. Many shops use both.

### Rust vs C++ — what are the real differences?

- **Memory safety** by default in Rust; opt-in (and easy to violate) in C++.
- **Tooling**: cargo is unified and excellent; C++ tooling is fragmented.
- **Backwards compatibility**: C++ carries decades of it; Rust is younger and cleaner.
- **Ecosystem**: C++ is vast and mature, especially for graphics/HPC; Rust is younger but growing fast.
- **Learning curve**: Rust is steep upfront (borrow checker); C++ is steep forever (UB landmines).

### Is Rust ready for [web / game dev / embedded]?

- **Web backend** — yes. axum, actix-web, tonic, sqlx, diesel. Solid.
- **Web frontend (WASM)** — usable. leptos, dioxus, yew. Smaller ecosystem than React.
- **Game dev** — Bevy is exciting but young; if you need Unity/Unreal-level tooling today, you're not there yet.
- **Embedded** — strong. `no_std` works well, embassy for async embedded, broad MCU support.
- **CLI tools, network services, systems software** — Rust's sweet spots.

### Why is Rust so hard to learn?

The borrow checker forces you to make ownership and lifetime decisions explicit that other languages hide (with GCs, defensive copies, or undefined behavior). Three to six months in, most people stop fighting it and find it clarifying. The early friction is the language teaching you a model, not picking on you.

---

## 10. Unsafe & FFI

### When is it okay to use `unsafe`?

`unsafe` doesn't disable the borrow checker — it unlocks five extra abilities: dereferencing raw pointers, calling unsafe functions, accessing/modifying mutable statics, implementing unsafe traits, and accessing union fields.

Legitimate uses:

- FFI calls (any C function is unsafe).
- Performance-critical primitives where bounds checks must go.
- Building safe abstractions over inherently unsafe operations (e.g., implementing `Vec`).

If you can't write a comment explaining *why* the unsafe block upholds Rust's invariants, don't write it.

### How do I call C from Rust (and vice versa)?

**Rust calling C:**
```rust
extern "C" {
    fn abs(input: i32) -> i32;
}
fn main() {
    unsafe { println!("{}", abs(-3)); }
}
```

**C calling Rust:**
```rust
#[no_mangle]
pub extern "C" fn add(a: i32, b: i32) -> i32 { a + b }
```

For nontrivial bindings, use `bindgen` (auto-generate Rust bindings from C headers) or `cbindgen` (generate C headers from Rust).

### What invariants must I uphold in `unsafe` blocks?

The big ones, paraphrased from the Rustonomicon:

- No dangling or misaligned pointers when dereferencing.
- No data races.
- No aliasing violations (no `&mut` overlapping with another `&mut` or `&`).
- All values must be valid for their type (no `bool` that's neither 0 nor 1, no invalid enum discriminants).
- Don't break the invariants of safe abstractions you build on top of.

Violating any of these is **undefined behavior**, which the compiler is allowed to assume cannot happen — leading to bugs that are far worse than a crash.

---

## Resources to keep handy

- **The Rust Book** — `doc.rust-lang.org/book` — the canonical free intro.
- **Rust by Example** — `doc.rust-lang.org/rust-by-example` — same material, code-first.
- **Rustonomicon** — `doc.rust-lang.org/nomicon` — the dark arts of unsafe Rust.
- **Async Book** — `rust-lang.github.io/async-book` — async fundamentals.
- **`std` docs** — `doc.rust-lang.org/std` — better than most languages' stdlib docs.
- **`cargo doc --open`** — your project's docs plus all dependencies, locally.

---

*Have a topic you'd like expanded — async patterns, lifetimes in depth, building a real crate? Happy to go deeper on any section.*