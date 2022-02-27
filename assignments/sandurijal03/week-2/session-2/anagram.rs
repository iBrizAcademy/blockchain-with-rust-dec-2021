fn exact_match(first: &str, second: &str) -> bool {
    let splited_first_string: Vec<&str> = first.split("").collect();
    let splitted_second_string: Vec<&str> = second.split("").collect();
    let mut is_present = true;
    for i in splited_first_string.iter() {
        if !splitted_second_string.contains(i) {
            is_present = false
        }
    }
    return is_present;
}

fn check_anagram(first: &str, second: &str) {
    if exact_match(first, second) {
        println!("{} and {} is anagrams.", first, second);
    } else {
        println!("{} and {} isn't anagrams.", first, second);
    }
}

fn main() {
    check_anagram("({[]})", "(){}[");
    check_anagram("abcde", "dbcae");
    check_anagram("({[]})", "({[)}]");
}
