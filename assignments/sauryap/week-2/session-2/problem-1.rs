// Program to pass ownership of a struct object, to a method and then returning it.

#[derive(Debug)]
struct Company {
    name: String,
    ceo: String,
    age: i32
}

fn main() {

    let c1 = Company {
        name: String::from("XYZ"),
        ceo: String::from("Xi Ping"),
        age: 40
    };

    let c2 = ownershiptransfer(c1);
    println!("{:?}", c2);

}

fn ownershiptransfer(c3:Company) -> Company  {
    println!("Function call - Ownership Transfer");
    return c3;
}