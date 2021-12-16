fn main() {
    let text = "Blockchain with Rust!";
    println!("{:p}", text);
    function_call(text);
    println!("{:p}", text);
}

fn function_call(string : &str){
    println!("{:p}", string);
}