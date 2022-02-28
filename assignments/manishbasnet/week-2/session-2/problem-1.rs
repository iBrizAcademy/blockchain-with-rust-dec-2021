// Program to pass ownership of a struct object, to a method and then returning it.

struct SimpleStruct {
    name: String,
    address: String,
    age: i32,
}

impl SimpleStruct {
    fn test(&mut self) {
        println!("Name: {}", self.name);
        println!("Address: {}", self.address);
        println!("Age: {}", self.age);
    }
    fn borrow_test(self, simple_struct: SimpleStruct) {
        if self.age == simple_struct.age {
            println!("Both age are same");
        } else {
            println!("Both age are different");
        };
    }
}
fn main() {
    let mut a = SimpleStruct {
        name: String::from("John"),
        address: String::from("KTM"),
        age: 25,
    };
    a.test();
    let b = SimpleStruct {
        name: String::from("Alex"),
        address: String::from("PKR"),
        age: 26,
    };
    b.borrow_test(a)
}