// ─── Concept 11: Implementing Structs ────────────────────────────────────────
//
// Run this file with:   cargo run --bin impl_structs
//
// In concept 10 we learned how to group data into a struct.
// Now we give that struct its own functions using `impl`.
//
// Think of it like this:
//   struct  = the data (name, age)
//   impl    = the abilities (greet, birthday)
//
// ────────────────────────────────────────────────────────────────────────────


struct Person {
    name: String,
    age: u32,
}

impl Person {

    // ── 1. new() — creates a Person ──────────────────────────────────────────
    //    This is called an associated function — no `self` needed
    //    You call it like:  Person::new(...)
    fn new(name: String, age: u32) -> Person {
        Person { name, age }
    }

    // ── 2. &self — reads the struct, doesn't change it ───────────────────────
    //    `self` refers to the Person this method is called on
    //    You call it like:  person.greet()
    fn greet(&self) {
        println!("Hi, I'm {} and I'm {} years old.", self.name, self.age);
    }

    fn is_adult(&self) -> bool {
        self.age >= 18
    }

    // ── 3. &mut self — can change the struct's data ───────────────────────────
    //    The variable must be declared with `mut` to call this
    fn birthday(&mut self) {
        self.age += 1;
        println!("{} is now {} years old.", self.name, self.age);
    }
}


fn main() {
    // ── Creating a Person using new() ─────────────────────────────────────────
    let mut vivek = Person::new(String::from("Vivek"), 21);

    // ── Calling methods on it ─────────────────────────────────────────────────
    vivek.greet();
    println!("Is adult: {}", vivek.is_adult());

    // ── Mutating method — changes the age ────────────────────────────────────
    vivek.birthday();
    vivek.greet();

    // ── Another person ────────────────────────────────────────────────────────
    let raj = Person::new(String::from("Raj"), 16);
    raj.greet();
    println!("Is adult: {}", raj.is_adult());
}
