use std::fs::File;

fn main() {
    // let f = File::open("hello.txt").unwrap();
    let f = File::open("hello.txt").expect("You can put the error message here");
}
