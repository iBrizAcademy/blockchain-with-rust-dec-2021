trait Speak {
    fn say(&self) -> String;
}

trait Animal {
    fn animal_type(&self) -> &str;
}

struct Person {
    name: String,
}
struct Dog {}

impl Animal for Dog {
    fn animal_type(&self) -> &str {
        "Dog"
    }
}

impl Animal for Person {
    fn animal_type(&self) -> &str {
        "Person"
    }
}

impl<T: Animal> Speak for T {
    fn say(&self) -> String {
        let animal_type = self.animal_type();
        let animal_says = match animal_type {
            "Dog" => "Woof!",
            "Person" => "Hello! My name is Adam",
            _ => "",
        };
        format!("{} says: {}", animal_type.to_string(), animal_says)
    }
}

fn main() {
    let person = Person { name: "Adam".to_string() };
    let dog = Dog {};

    println!("{}", person.say());
    println!("{}", dog.say());
}
