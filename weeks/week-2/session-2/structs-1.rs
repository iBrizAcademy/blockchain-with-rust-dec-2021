struct Person {
    first_name: String,
    last_name: String,
    age: i32,
}

fn main() {
    let mut person1 = Person {
        first_name: String::from("Blockchain"),
        last_name: String::from("Rust"),
        age: 21,
    };
    println!("{}", person1.age);
}