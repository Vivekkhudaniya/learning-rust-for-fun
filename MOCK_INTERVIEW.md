# Rust Mock Interview Questions

---

## Rust Basics

### Ownership & Borrowing
1. What is ownership in Rust?
2. What happens when you assign one variable to another?
3. What is the difference between `&` and `&mut`?
4. How many mutable borrows can exist at the same time?
5. What is the difference between `Copy` and `Clone`?

### Core Concepts
6. What is the difference between `String` and `&str`?
7. What is `Option<T>` and why does Rust use it instead of null?
8. What is `Result<T, E>` and when do you use it?
9. What is pattern matching and how does `match` work?
10. What is the difference between `unwrap()`, `expect()`, and the `?` operator?
11. What is a struct and what is `impl`?
12. What is an enum and how is it different from other languages?
13. What is mutability — difference between `let` and `let mut`?
14. What is shadowing?

### Memory
15. Stack vs Heap — what goes where?
16. How does Rust manage memory without a garbage collector?
17. When is memory freed in Rust?

---

## Rust Backend (Axum)

### Concepts
18. What is Tokio and why do we need it?
19. What does `async/await` do?
20. What is `#[tokio::main]`?
21. What is Axum and how do you define a route?
22. What is a handler function in Axum?
23. What is `Json<T>` in Axum?
24. What is the `Path` extractor in Axum?
25. What is Serde and why do we use `#[derive(Serialize, Deserialize)]`?

### Practical
26. Write a basic GET route in Axum.
27. Write a POST route that accepts a JSON body.
28. How do you extract a path parameter like `/users/:id`?

---

## General Backend
29. Tell me about Rust — why would you choose it over Python or JavaScript for a backend?
