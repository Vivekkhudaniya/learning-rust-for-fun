// ─── Concept 11: Implementing Structs ────────────────────────────────────────
//
// Run this file with:   cargo run --bin impl_structs
//
// In concept 10 we learned how to define structs and group data together.
// Now we attach functions (called methods) directly to a struct using `impl`.
//
// Think of `impl` like giving your struct its own abilities.
// A Rectangle struct can calculate its own area, perimeter, etc.
//
// ────────────────────────────────────────────────────────────────────────────


#[derive(Debug)]
struct Rectangle {
    width: f64,
    height: f64,
}

// ── impl block — attach methods to Rectangle ─────────────────────────────────
//    All methods for a struct go inside one `impl` block
impl Rectangle {

    // ── 1. Associated function (no self) — works like a constructor ───────────
    //    Called with Rectangle::new(...)  not  rect.new(...)
    //    `self` is not a parameter — it doesn't need an instance to work
    fn new(width: f64, height: f64) -> Rectangle {
        Rectangle { width, height }
    }

    // ── 2. Method (&self) — reads data, doesn't change anything ──────────────
    //    `&self` means "borrow the struct, read-only"
    //    Called with  rect.area()
    fn area(&self) -> f64 {
        self.width * self.height
    }

    fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }

    fn is_square(&self) -> bool {
        self.width == self.height
    }

    // ── 3. Method (&mut self) — can change the struct's data ─────────────────
    //    `&mut self` means "borrow the struct, allow changes"
    //    The instance must be declared with `mut` to call this
    fn scale(&mut self, factor: f64) {
        self.width *= factor;
        self.height *= factor;
    }

    // ── 4. Method that takes another struct as a parameter ───────────────────
    //    Can this rectangle fit inside another?
    fn fits_inside(&self, other: &Rectangle) -> bool {
        self.width < other.width && self.height < other.height
    }
}


#[derive(Debug)]
struct Circle {
    radius: f64,
}

impl Circle {
    fn new(radius: f64) -> Circle {
        Circle { radius }
    }

    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }

    fn circumference(&self) -> f64 {
        2.0 * std::f64::consts::PI * self.radius
    }
}


fn main() {
    // ── Using Rectangle ───────────────────────────────────────────────────────
    let rect = Rectangle::new(10.0, 5.0);  // associated function — no instance needed
    println!("Rectangle: {:?}", rect);
    println!("Area:      {}", rect.area());
    println!("Perimeter: {}", rect.perimeter());
    println!("Is square: {}", rect.is_square());

    // ── Mutable method — scale the rectangle ─────────────────────────────────
    let mut rect2 = Rectangle::new(4.0, 4.0);
    println!("\nBefore scale: {:?}", rect2);
    rect2.scale(2.0);
    println!("After scale:  {:?}", rect2);
    println!("Is square:    {}", rect2.is_square());

    // ── Method with another struct as parameter ───────────────────────────────
    let small = Rectangle::new(3.0, 2.0);
    let large = Rectangle::new(10.0, 8.0);
    println!("\nSmall fits inside large: {}", small.fits_inside(&large));
    println!("Large fits inside small: {}", large.fits_inside(&small));

    // ── Using Circle ──────────────────────────────────────────────────────────
    let circle = Circle::new(5.0);
    println!("\nCircle: {:?}", circle);
    println!("Area:          {:.2}", circle.area());
    println!("Circumference: {:.2}", circle.circumference());
}
