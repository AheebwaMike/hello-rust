/*

Given a word, return the first and last character of that string as a new string

Example:
1. Hello  ==> ho
2. Jinx ==> Jx
3. rust ==> rt
4. %$@%^! ==> %!

*/

fn first_and_last(s: &str) -> String {
    let mut final_str = String::from("");
    let len = s.len();

    final_str.push_str(&s[..1]);
    final_str.push_str(&s[len-1..len]);

    final_str
}

fn main() {
    let name = String::from("Hellenistic");
    let f = first_and_last(&name);
    println!("Name is {f}");
}
