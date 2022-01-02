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

fn someFunction(person: &mut Person){
    let mut person1 = &mut *person;
    person1.first_name = String::from("Manjeet");
}

fn main(){
    let mut person: Person = Person{
        first_name: String::from("Manjit"),
        last_name: String::from("Shakya"),
        age: 23
    };
    println!("{}", person.to_string());
    someFunction( &mut person);
    println!("{}", person.to_string());

}