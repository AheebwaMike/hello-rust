
fn main() {
    // Enter your list of numbers here
    let numbers = [2, 4, 5, 6, 8, 125, 43, 123, 7];

    let mut biggest = 0;
    for n in numbers {
        if n > biggest {
            biggest = n;
        }
    }

    println!("Biggest in list: {biggest}");
}
