fn hello_world(name: &str) -> String {
    let result = String::from("hello world ") + name;
    result
}

fn main() {
    let message = hello_world("Sandip Rijal");
    println!("{}", message);
}
