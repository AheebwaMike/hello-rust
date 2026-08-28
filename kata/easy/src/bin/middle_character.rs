/*

Given a word, return the middle character(s). If the word's length is odd, return the middle character. If even, return the two middle characters.

Example:

get_middle("test") -> "es"
get_middle("testing") -> "t"
get_middle("A") -> "A"

*/

fn middle_char(word: &str) -> &str {
    if word.len() % 2 == 0 {
        word
    } else {
        word
    }
}



fn main() {
    let result = middle_char("hellos");
    println!("Resut: {:?}", result);
}
