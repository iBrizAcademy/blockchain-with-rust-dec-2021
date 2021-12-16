fn main() {
    let text = "Blockchain with Rust!";
    let text1 = text;
    println!("{}", text1);

    let text = String::from("Blockchain with Rust!");
    let text1 = text.clone();
    println!("{}", text);
}