// // This is my first rust program

const CONCEPTS: (&str, &str) = (
    "Mutability",
    "Datatypes"
);

// Terminal decorator function
fn decor_up(fn_name: &str, _is_end: bool) {
    println!("\n============================");
    println!("Running code on: {fn_name}");
    println!("============================");
}

fn decor_down() {
    println!("----------------------------\n");
}

// mutability
fn mutability() {
    decor_up(CONCEPTS.0, false);
    let name = "Mike";
    let mut age = 22;
    println!();
    println!("His name is {name} and he's {age} years old");

    age = 36;
    println!("His new age now {age}\n");
    decor_down()
}

fn main() {
    mutability();
    mutability();
}
