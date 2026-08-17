
fn main() {
    // Enter your list of numbers here
    let numbers: [u32; 4] = [2, 1, 7, 3];

    let mut biggest = 0;
    
    for n in numbers {
        if n > biggest {
            biggest = n;
        }
    }

    println!("Biggest in list: {biggest}");
}
