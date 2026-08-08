/*

For a given positive integer n, square every digit and concatenate them.

Example:
square_digits(9119) -> 811181  # because 9^2=81, 1^2=1, 1^2=1, 9^2=81

*/

fn square_every_digit(n: u64) -> u64 {
    let n_string: String = n.to_string();
    
    let mut conc_string = String::from("");

    for c in n_string.chars() {
        let char_int = c.to_string().parse::<u64>().unwrap();
        let char_square = char_int.pow(2);
        let square_char = char_square.to_string();

        conc_string = conc_string + &square_char;
    }

    conc_string.parse::<u64>().unwrap()
}


fn main() {
    // SIMPLE TEST

    // ------- Enter The Required Number Here (n) --------
    let n: u64 = 1209;

    // Enter the expected result (expected) --------------
    let expected: u64 = 14081;
    
    let y = square_every_digit(n);
    let passed = if expected == y {"yes!"} else {"no!"};
    
    println!("
        Number: {n}
        Expected: {expected}
        Computed: {y}
        Test passed: {passed}
    ")
}

