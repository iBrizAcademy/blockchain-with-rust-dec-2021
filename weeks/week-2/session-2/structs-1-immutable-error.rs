struct Person {
    first_name: String,
    last_name: String,
    age: i32,
}

fn main() {
    let person1 = Person {
        first_name: String::from("Blockchain"),
        last_name: String::from("Rust"),
        age: 21,
    };
    person1.age = 20;

    println!("{}", person1.age);
}