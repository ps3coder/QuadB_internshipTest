fn shortest_word(s: &str) -> Option<&str> {
    s.split_whitespace()
        .min_by_key(|word| word.len())
}

fn main() {
    let sentence = "Hello my self ps3coder.I am Backend Developer";
    match shortest_word(&sentence) {
        Some(word) => println!("The shortest word is: {}", word),
        None => println!("The input string is empty."),
    }
}


// 1)We use the min_by_key method to find the word with the minimum length
// 2)The function returns an Option<&str>. If the string is empty or contains no words, it returns None.
// 3) In the main function, we handle the Option and print the shortest word if it exists.
// 4) We use the split_whitespace method to split the string into words based on whitespace.
