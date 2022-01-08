//- Program to pass ownership of a struct object, to a method and then returning it.

struct SimpleStruct {
    name: String,
    age: i32,
}

impl SimpleStruct {
    fn test(&mut self) {
        println!("my name is: {}", self.name);
        println!("my age is: {}", self.age);
    }
    fn borrow_test(self, st: SimpleStruct) {
        if self.age == st.age {
            println!("we are of same age");
        } else {
            println!("we are of different age");
        };
    }
}
fn main() {
    let mut a = SimpleStruct {
        name: String::from("Hello world"),
        age: 20,
    };
    a.test();
    let b = SimpleStruct {
        name: String::from("Hello world"),
        age: 20,
    };
    b.borrow_test(a)
}
