fn main() {
    check_anagram("abcd", "dbac");
    check_anagram("car", "fan");
}
fn check_contains(first: &str, second: &str) -> bool{
    let mut contains = true;
    for c in first.chars() {
        if !second.contains(c){
            contains = false;
        }
    }
    return contains;
}
fn check_anagram(first: &str, second: &str){
    if check_contains(first, second) && check_contains(second, first){
        println!("{} and {} are anagrams", first, second);
    }
    else{
        println!("{} and {} are not anagrams", first, second);
    }
}
