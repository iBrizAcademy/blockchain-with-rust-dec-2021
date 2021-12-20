fn main() {
    let str_value = String::from("Blockchain with Rust");
    fn_move(str_value);
    println!("Moved: {:?}", str_value);

    fn_move(str_value);
    println!("Moved: {:?}", str_value);
}

fn fn_borrow(str_value: &String) {
    println!("{:?}",str_value );
}

fn fn_move(str_value: String) {
    println!("{:?}",str_value );
}