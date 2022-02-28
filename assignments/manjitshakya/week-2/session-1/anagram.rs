// sh build-n-run.sh assignments/manjitshakya/week-2/session-1/anagram.rs

fn sort_character(value: String) -> String {
    let mut value_character: Vec<char> = value.chars().collect();
    value_character.sort();
    return value_character.into_iter().collect();
}

fn anagram(value1: String, value2: String) {
    let value1_length: usize = value1.chars().count();
    let value2_length: usize = value2.chars().count();
    let failure_msg: &str = "Not Anagram";
    let success_msg: &str = "Anagram";

    if value1_length != value2_length {
        println!("{}", failure_msg);
    } else {
        let value1_sorted: String = sort_character(value1);
        let value2_sorted: String = sort_character(value2);
        println!("After sorting value1: {}, value2: {}", value1_sorted, value2_sorted);
        if value1_sorted.eq_ignore_ascii_case(value2_sorted.to_string().as_str()) {
            println!("{}", success_msg);
        } else {
            println!("{}", failure_msg);
        }
    }
}

fn main() {
    anagram("bueeub".to_string(), "ubbeeu".to_string()); // Anagram
    anagram("dog".to_string(), "god".to_string()); // Anagram
    anagram("dad".to_string(), "ded".to_string()); // Not Anagram
}