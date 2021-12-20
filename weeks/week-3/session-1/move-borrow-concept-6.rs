fn main() {
    let str_value = String::from("Blockchain with Rust");
    fn_borrow(&str_value);
    println!("Borrowed: {:?}", str_value);

    fn_borrow(&str_value);
    println!("Borrowed: {:?}", str_value);
}

fn fn_borrow(str_value: &String) {
    println!("{:?}",str_value );
}

fn fn_move(str_value: String) {
    println!("{:?}",str_value );
}