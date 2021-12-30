trait Speak {
    fn say(&self) -> String;
}

struct Person {
    name : String,
}
struct Dog {}

impl Speak for Person {
    fn say(&self) -> String {
        format!("Hello, my name is {}", self.name)
    }
}

impl Speak for Dog {
    fn say(&self) -> String {
        format!("Woof!")
    }
}

fn main() {
    let person = Person { name : "Adam".to_string() };
    let dog = Dog {};

    println!("Person Says: {}", person.say());
    println!("Dog Says: {}", dog.say());
}
