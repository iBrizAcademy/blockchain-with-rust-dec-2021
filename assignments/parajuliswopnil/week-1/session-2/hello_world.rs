use std::io;

fn main(){
    let mut input = String::new();
    
    println!("Enter your name: ");
    
    match io::stdin().read_line(&mut input){
        Ok(_) => {
            println!("Hello world! {}", input)
        },
        Err(e) => {
            println!("Something went wrong! {}", e)
        }
    }
}