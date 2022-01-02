struct Person {
    first_name: String,
    last_name: String,
    age: i32,
}

impl Person {
    pub fn to_string(&self) -> String {
        return format!("Person (first_name: {}, last_name: {}, age: {})", &self.first_name, &self.last_name, &self.age);
    }
}

fn someFunction(person: Person) -> Person {
    return person;
}

fn main() {
    let person: Person = Person {
        first_name: String::from("Manjit"),
        last_name: String::from("Shakya"),
        age: 23,
    };
    println!("{:?}", person.to_string());
    let person1 = someFunction(person);
    println!("{:?}", person1.to_string());
}