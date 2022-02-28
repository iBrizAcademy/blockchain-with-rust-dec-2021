//Program to pass struct variable as reference and dereferencing to modify value of object
struct Person {
    name: String,
    age: i32,
    address: String
}
fn main() {
    let mut person = Person {
        name: String::from("abcd"),
        age: 18,
        address: String::from("abcd"),
    };
    println!("Name: {}, Age: {}, Address: {}", person.name, person.age, person.address);
    function_call(&mut person);
    println!("Name: {}, Age: {}, Address: {}", person.name, person.age, person.address);
}

fn function_call(person: &mut Person){
    *person = Person{
        name: person.name.clone(),
        age: 55,
        address: String::from("ddddd")
    };
}