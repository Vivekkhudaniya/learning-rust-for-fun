// ─── Concept 12: Enums ───────────────────────────────────────────────────────
//
// Run this file with:   cargo run --bin enums
//
// An enum lets you define a type that can be one of a few fixed options.
// Like a traffic light — it can only be Red, Yellow, or Green. Nothing else.
//
// ────────────────────────────────────────────────────────────────────────────


// ── 1. Basic enum ─────────────────────────────────────────────────────────────
//    Define the possible options
#[allow(dead_code)]
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

// ── 2. Enum with data attached ────────────────────────────────────────────────
//    Each option can carry its own data
enum Payment {
    Cash(f64),                  // carries an amount
    Card(String),               // carries a card number
    UPI { id: String },         // carries a UPI id (named field)
}

// ── 3. impl on an enum — give it methods ─────────────────────────────────────
impl TrafficLight {
    fn message(&self) -> &str {
        match self {
            TrafficLight::Red    => "Stop!",
            TrafficLight::Yellow => "Get ready...",
            TrafficLight::Green  => "Go!",
        }
    }
}


fn main() {
    // ── Using a basic enum ────────────────────────────────────────────────────
    let light = TrafficLight::Red;
    println!("Light says: {}", light.message());

    let light = TrafficLight::Green;
    println!("Light says: {}", light.message());


    // ── Using an enum with data ───────────────────────────────────────────────
    let p1 = Payment::Cash(500.0);
    let p2 = Payment::Card(String::from("4111-1111-1111-1111"));
    let p3 = Payment::UPI { id: String::from("vivek@upi") };

    // match lets you handle each option separately
    // we will cover match in detail in concept 13
    let payments = vec![p1, p2, p3];

    for payment in &payments {
        match payment {
            Payment::Cash(amount)    => println!("Paid ₹{} in cash", amount),
            Payment::Card(number)    => println!("Paid by card: {}", number),
            Payment::UPI { id }      => println!("Paid via UPI: {}", id),
        }
    }
}
