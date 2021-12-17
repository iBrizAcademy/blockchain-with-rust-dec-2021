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

    let mut person2 = Person {
        age: 11,
        ..person1
    };

    println!("{}", person2.age);
    println!("{}", person2.first_name);
}