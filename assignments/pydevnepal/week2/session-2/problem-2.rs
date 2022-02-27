// - Program to pass struct variable as reference and dereferencing to modify value of object

// #[derive(Debug)]
struct SimpleStruct {
    name: String,
    age: i32,
}

impl SimpleStruct {
    fn change_values(&mut self, new_age: i32, new_name: String) {
        self.age = new_age;
        self.name = new_name;
    }
}
fn main() {
    let mut var1 = SimpleStruct {
        name: String::from("Hello"),
        age: 20,
    };
    println!("Name: {:?}", var1.name);
    println!("Age: {:?}", var1.age);

    var1.change_values(5, String::from("Hello Again"));
    println!("New Name: {:?}", var1.name);
    println!("New Age: {:?}", var1.age);
}
