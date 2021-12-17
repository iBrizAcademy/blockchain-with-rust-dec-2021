fn main() {
    let mut text = String::from("hello world");
    println!("{}", text);

    append_string(&mut text); // passing reference to text

    println!("{}", text);
}

fn append_string(text: &mut String) {
    *text = text.clone() + "!"; // dereference and append "!"
}