fn main() {
    let s = String::from("Hello");
    let reference = &s;

    println!("The reference is {reference}");
    println!("The reference is still valid: {reference}");
}