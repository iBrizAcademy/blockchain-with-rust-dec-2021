fn main() {
    let text = String::from("Blockchain with Rust!");
    println!("{:p}", text.as_ptr());
    function_call(text);
    println!("{:p}", text.as_ptr());
}

fn function_call(string : String) {
    println!("{:p}", string.as_ptr());
}