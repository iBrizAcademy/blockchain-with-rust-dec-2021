// Program to pass struct variable as reference and dereferencing to modify value of object

#[derive(Debug)]
struct Company {
    name: String,
    ceo: String,
    age: i32
}

fn main() {

    let mut c1 = Company {
        name: String::from("XYZ"),
        ceo: String::from("Xi Ping"),
        age: 40
    };
    println!("Before Referencing {:?}", c1);

    ownershiptransfer(&mut c1);
    println!("After Referencing {:?}", c1);

}

fn ownershiptransfer(c1:&mut Company) {
    (*c1).name = String::from("ABC");
    (*c1).age = 30;
    
}