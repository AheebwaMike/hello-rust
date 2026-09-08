struct SentenceStats {
    text: String
}

impl SentenceStats {
    fn word_count(&self) -> usize {
        self.text.split_whitespace().count()
    }
    fn first_word(&self) -> Option<&str> {
        self.text.split_whitespace().next()
    }
    fn last_word(&self) -> Option<&str> {
        self.text.split_whitespace().last()
    }
    fn longest_word(&self) -> Option<&str> {
        self.text.split_whitespace().max_by_key(|word| word.len())
    }
    fn repeated_words(&self) -> Vec<&str> {
        let mut word_count = std::collections::HashMap::new();
        for word in self.text.split_whitespace() {
            *word_count.entry(word).or_insert(0) += 1;
        }
        word_count.into_iter().filter(|&(_, count)| count > 1).map(|(word, _)| word).collect()
    }
    fn print_stats(&self) {
        println!("Word count: {}", self.word_count());
        println!("First word: {:?}", self.first_word().unwrap_or("No words found"));
        println!("Last word: {:?}", self.last_word().unwrap_or("No words found"));
        println!("Longest word: {:?}", self.longest_word().unwrap_or("No words found"));
        println!("Repeated words: {:?}", self.repeated_words());
    }

    fn just_an_associated_function() {
        println!("This is called by an associated function, not a method.");
    }
}


fn main() {
    let text = String::from("energy can neither be created nor destroyed, it can only be changed from one form to another");
    let text_stats = SentenceStats { text };

    text_stats.print_stats();
    SentenceStats::just_an_associated_function();

}