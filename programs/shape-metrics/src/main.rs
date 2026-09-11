use std::f64::consts::PI;

#[derive(Debug)]
enum Shape {
    Circle(f64),
    Triangle(f64, f64, f64),
    Rectangle(f64, f64),
    Square(f64)
}

impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Circle(r) => PI * r * r,
            Shape::Triangle(a, b, c) => {
                let s = (a + b + c) / 2.0;
                (s * (s - a) * (s - b) * (s - c)).sqrt()
            },
            Shape::Rectangle(s1, s2) => s1 * s2,
            Shape::Square(s) => s * s
        }
    }

    fn perimeter(&self) -> f64{
        match self {
            Shape::Circle(r) => 2.0 * PI * r,
            Shape::Triangle(a, b, c) => a + b + c,
            Shape::Rectangle(s1, s2) => 2.0 * (s1 + s2),
            Shape::Square(s) => 4.0 * s
        }
    }

    fn description(&self) -> String {
        match self {
            Shape::Circle(r) => format!("Circle (radius: {r}) — Area: {:.2} sq. units, Perimeter: {:.2} units", self.area(), self.perimeter()),
            Shape::Triangle(a, b, c) => format!("Triangle (a: {a}, b: {b}, c: {c}) — Area: {:.2} sq. units, Perimeter: {:.2} units", self.area(), self.perimeter()),
            Shape::Rectangle(s1, s2) => format!("Rectangle (s1: {s1}, s2: {s2}) — Area: {:.2} sq. units, Perimeter: {:.2} units", self.area(), self.perimeter()),
            Shape::Square(s) => format!("Square (s: {s}) — Area: {:.2} sq. units, Perimeter: {:.2} units", self.area(), self.perimeter())
        }
    }
}


fn print_shapes(shapes: &[Shape]) {
    println!("\n--- Shape Collection ---");
    for shape in shapes {
        println!("{}", shape.description());
    }
}

fn largest_shape(shapes: &[Shape]) -> Option<&Shape> {
    let mut biggest = shapes.first()?;
    for shape in shapes {
        if shape.area() > biggest.area() {
            biggest = shape
        }
    }
    Some(biggest)
}

fn main() {
    let shapes = [
        Shape::Circle(1.0),
        Shape::Circle(0.7),
        Shape::Square(6.2),
        Shape::Triangle(3.0, 4.0, 5.0),
        Shape::Rectangle(2.3, 10.0)
    ];
    
    print_shapes(&shapes);

    println!("\n--- Shape with largest area ---");
    match largest_shape(&shapes) {
        Some(biggest) => println!("Largest: {}", biggest.description()),
        None => println!("No shapes to compare."),
    }

    println!("\n--- Bonus ---");
    let is_first_circle_text = if let Shape::Circle(r) = shapes[0] { 
        format!("First shape is a circle with radius {r}") 
    } else { 
        "First shape is not a circle".to_string()
     };
    println!("{}", is_first_circle_text);
}
