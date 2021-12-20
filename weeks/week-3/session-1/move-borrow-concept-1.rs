fn main() {
    let str_value = String::from("Blockchain with Rust");
    move_borrow(str_value);

    println!("{:?}", str_value);
}

fn move_borrow(str_value: String) {
    println!("{:?}",str_value );
}
















// move ownership