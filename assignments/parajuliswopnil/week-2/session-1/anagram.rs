use std::iter::Iterator;
use std::iter::FromIterator;
fn main(){
    let string1 = String::from("triangle");
    let string2 = String::from("integral");
    if is_anagram(&string1, &string2) == true{
        println!("They are anagrams");
    }
    else{
        println!("They are not anagrams");
    }
}

fn is_anagram(string1: &str, string2: &str) -> bool{
    let sliced_string1: &str = &string1[..];
    let sliced_string2: &str = &string2[..];
    let mut sliced_string1_chars: Vec<char>= sliced_string1.chars().collect();
    let mut sliced_string2_chars: Vec<char>= sliced_string2.chars().collect();

    sliced_string1_chars.sort_by(|a, b| a.cmp(b));
    sliced_string2_chars.sort_by(|a, b| a.cmp(b));

    println!("{:?}", sliced_string1_chars);
    let sorted_string1 = String::from_iter(sliced_string1_chars);
    let sorted_string2 = String::from_iter(sliced_string2_chars);

    println!("{}", sorted_string1);
    println!("{}", sorted_string2);
    if sorted_string1 == sorted_string2{
        return true;
    }
    else{
        return false;
    }
}