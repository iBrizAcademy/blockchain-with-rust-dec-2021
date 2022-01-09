struct Person {
    name: String,
    age: u8,
}

fn main() {
    let p = Person {
        name: String::from("Adam"),
        age: 30,
    };

    println!("{}", p.name);
    println!("{}", p.age);
}