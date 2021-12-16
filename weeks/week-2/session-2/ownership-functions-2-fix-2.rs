fn main() {
    let text = function_call();
    println!("{:p}", text.as_ptr());
}

fn function_call() -> String{
    let text = String::from("Blockchain with Rust!");
    println!("{:p}", text.as_ptr());
    return text;
}