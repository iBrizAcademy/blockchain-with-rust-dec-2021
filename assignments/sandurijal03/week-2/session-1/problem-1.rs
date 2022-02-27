#[derive(Debug)]
struct Person {
  name: String,
  age: u32
}

fn main() {
  let person = display_person_struct();
  println!("{:p}", person.name.as_ptr());

  println!("person = {:?}", person);
  println!("Hello my name is {} and i am {} years old.", person.name, person.age);  
}

fn display_person_struct() -> Person {
  let person = Person {
    name: "sandy".to_string(),
    age: 25
  };
  println!("{:p}", person.name.as_ptr());
  person
}