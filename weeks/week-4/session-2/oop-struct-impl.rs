pub struct Person {
    name: String,
    age: u8,
}

impl Person {
    pub fn get_intro(&self) -> String {
        format!("Hello, I am {}, age {}! ", self.name, self.age)
    }
}

fn main() {
    let p = Person {
        name: String::from("Adam"),
        age: 30,
    };

    println!("{}", p.name);
    println!("{}", p.age);

    println!("{}", p.get_intro());
}