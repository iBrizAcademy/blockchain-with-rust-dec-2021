// Program to pass struct variable as reference and dereferencing to modify value of object

// #[derive(Debug)]
struct SimpleStruct {
    name: String,
    address: String,
    age: i32,
}

impl SimpleStruct {
    fn change_values(&mut self, new_age: i32, new_name: String, new_address: String) {
        self.age = new_age;
        self.name = new_name;
        self.address = new_address;
    }
}
fn main() {
    let mut var1 = SimpleStruct {
        name: String::from("John"),
        age: 25,
        address: String::from("KTM")
    };
    println!("Name: {:?}", var1.name);
    println!("Age: {:?}", var1.age);
    println!("Address: {:?}", var1.address);

    var1.change_values(15, String::from("Alex"), String::from("PKR"));
    println!("New Name: {:?}", var1.name);
    println!("New Age: {:?}", var1.age);
    println!("New Address: {:?}", var1.address);
}