//Program to pass ownership of a struct object, to a method and then returning it.
struct Person {
    name: String,
    age: i32,
    address: String
}
fn main() {
    let person = function_call(Person{
        name: String::from("abcd"),
        age: 18,
        address: String::from("abcd"),
    });
    println!("Name: {}, Age: {}, Address: {}", person.name, person.age, person.address);
}

fn function_call(person: Person) -> Person{
    return person;
}