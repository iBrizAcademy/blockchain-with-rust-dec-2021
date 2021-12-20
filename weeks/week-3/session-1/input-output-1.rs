use std::io::Write;


fn main(){
    let mut name = String::new();
    println!("Name? :");
    let name_bytes = std::io::stdin().read_line(&mut name).unwrap();
    println!("Welcome to Blockchain in Rust, {}", name);
    println!("# bytes: {}", name_bytes);

    println!("\n\n");
    std::io::stdout().write("Welcome to Blockchain in Rust.\n".as_bytes()).unwrap();
}