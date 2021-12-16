fn main() {
    // string type value is owned by text
    let text = String::from("Blockchain with Rust!");

    // text is freed, ownership to text1
    let text1 = text;

    println!("{}", text);
}