// ─── Concept 10: Structs ─────────────────────────────────────────────────────
//
// Run this file with:   cargo run --bin structs
//
// A struct lets you group related data together into one custom type.
// Instead of having separate variables for name, age, email —
// you put them all inside one struct called "User" or "Person".
//
// Think of a struct like a form — it has fixed fields, you fill them in.
//
// ────────────────────────────────────────────────────────────────────────────


// ── Defining structs ─────────────────────────────────────────────────────────
//    Write them outside of main, at the top level
//    Each field has a name and a type

struct Person {
    name: String,
    age: u32,
    is_student: bool,
}

struct Rectangle {
    width: f64,
    height: f64,
}

// #[derive(Debug)] allows printing the struct with {:?}
#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}


fn main() {

    // ── 1. Creating a struct instance ─────────────────────────────────────────
    //    Fill in all the fields — all fields are required
    let person1 = Person {
        name: String::from("Vivek"),
        age: 21,
        is_student: true,
    };

    // ── 2. Accessing fields with dot notation ─────────────────────────────────
    println!("Name:       {}", person1.name);
    println!("Age:        {}", person1.age);
    println!("Is student: {}", person1.is_student);


    // ── 3. Mutable struct — all fields become mutable ─────────────────────────
    //    You can't make just one field mutable — it's all or nothing
    let mut person2 = Person {
        name: String::from("Raj"),
        age: 20,
        is_student: false,
    };

    person2.age = 21;
    person2.is_student = true;
    println!("\nUpdated age: {}", person2.age);


    // ── 4. Passing a struct into a function ───────────────────────────────────
    //    Borrow it with & so we keep ownership after the call
    let rect = Rectangle { width: 10.0, height: 5.0 };
    println!("\nRectangle area: {}", area(&rect));


    // ── 5. Function that returns a struct ─────────────────────────────────────
    let p = make_point(3.0, 4.5);
    println!("Point: ({}, {})", p.x, p.y);


    // ── 6. Struct update syntax ───────────────────────────────────────────────
    //    Create a new struct copying some fields from an existing one
    //    ..person2 means "copy all remaining fields from person2"
    let person3 = Person {
        name: String::from("Ankit"),
        ..person2
    };
    println!("\nPerson3 name: {}", person3.name);
    println!("Person3 age:  {}", person3.age);   // copied from person2


    // ── 7. Printing a struct with {:?} ────────────────────────────────────────
    //    Normal {} doesn't work on structs
    //    Add #[derive(Debug)] above the struct to enable {:?} printing
    let p2 = Point { x: 1.0, y: 2.0 };
    println!("\nPoint: {:?}", p2);          // compact
    println!("Point: {:#?}", p2);          // pretty printed (one field per line)
}


// Takes a reference to Rectangle — borrows it, doesn't take ownership
fn area(r: &Rectangle) -> f64 {
    r.width * r.height
}

// Returns a new Point
// Shorthand: when variable name matches field name, write it once
fn make_point(x: f64, y: f64) -> Point {
    Point { x, y }
}
