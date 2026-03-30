use std::collections::HashMap;

fn main() {
    let text = "the quick brown fox jumps over the lazy fox";
    let word_count = word_counter(text);
    println!("{:?}", word_count);
}

fn word_counter(word: &str) -> HashMap<String, usize>{
    let mut counter: HashMap<String, usize> = HashMap::new();
    for word in word.split_whitespace(){
        let lower_word = word.to_lowercase();
        *counter.entry(lower_word).or_insert(0) += 1;
    }
    counter
}