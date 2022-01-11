use std::fs::File;

fn main() {
    // open returns a Result enum
    let f = File::open("hello.txt");
    println!("{:?}",f)
    // let f:u32 = File::open("hello.txt");
    // line 7 commented code will throw error due to type mismatch

}
