mod encapsulation {
    pub struct Person {
        name: String,
        age: u8,
    }

    impl Person {
        pub fn new(name: String, age: u8) -> Person {
            Person { name, age }
        }

        pub fn get_intro(&self) -> String {
            format!("Hello, I am {}, age {}! ", self.name, self.age)
        }
    }
}

fn main() {
    use encapsulation::*;


    let person = Person::new(String::from("Adam"), 25);
    println!("{}", person.name);
}