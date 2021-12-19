struct Person {
  name: String,
  age: u32
}

fn main() {
  let mut person = Person {
    name: String::from("Sandy"),
    age: 25
  };

  mutate_struct(&mut person);
}


fn mutate_struct(obj: &mut Person) {
  let name = &mut obj.name;
  let age = &mut obj.age;
  *name = "mandy".to_string();
  *age += 1;
}