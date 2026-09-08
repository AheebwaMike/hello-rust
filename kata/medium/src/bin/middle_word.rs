/*

Given a sentence (space separated words), return the middle word using these criteria:
1. return the (N+1)/2 th word if N is odd or
2. return the slice containing the (N/2)th and (N/2)th + 1 words
where N is the number of words in the sentence.

Examples:
1. Apples are green ==> are
2. I love to eat green apples ==> to eat

*/

fn word_count(s: &str) -> u32 {
    let s_bytes = s.as_bytes();
    let mut count = 0;

    for &item in s_bytes {
        if item == b' ' {
            count += 1;
        }
    }

    count + 1
}

fn middle_word(s: &str) -> &str {
    let n_words = word_count(&s);
    let is_even = if s.len() % 2 == 0 { true } else { false };

    let mid_idx = if is_even { n_words / 2 - 1 } else { (n_words - 1) / 2 };
    let mut start_idx = 0;
    let mut end_idx = 0;
    let mut w = 0;

    for (i, c) in s.chars().enumerate() {
        if c == ' ' {
            w += 1;
        }

        if w == mid_idx {
            start_idx = i;
            end_idx = if is_even { (w + 1) } else { (w + 2) };
        }

    }
    
    &s[start_idx..end_idx.try_into().unwrap()]
}

fn main() {
    let sentence = String::from("My name is Michael");
    let mid_word = middle_word(&sentence);
    
    println!("\nMiddle word of '{}' is {} words\n", sentence, mid_word);
}
