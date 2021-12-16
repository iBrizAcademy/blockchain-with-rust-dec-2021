fn main() {
    let text = "Blockchain with Rust!"; // String literal
    // stored in readonly memory - not in stack or heap, but pointer is stored in stack
    let text1 = text; // text1 owns copy of content of text
    println!("{}", text);

    let text = String::from("Blockchain with Rust!"); // string type value is owned by text
    let text1 = text; // text moved to text1, since String type cannot create copy
    // ownership of text is transferred to text1  - this is move semantics in Rust
    println!("{}", text);
}