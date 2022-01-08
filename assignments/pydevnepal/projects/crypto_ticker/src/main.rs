mod serializer;
use serializer::*;
use std::io;

fn main() {
    println!("\n\t CRYPTO TICKER \n");
    let symbol = String::from("btc");
    get_bitcoin_price(&symbol).unwrap();
    loop {
        println!(
            "If you want to get the price of another crypto coin then enter the symbol of the coin \
        else type 'exit' to quit."
        );
        let mut sym = String::new();
        io::stdin()
            .read_line(&mut sym)
            .expect("Failed to read line");
        match sym.trim().to_lowercase().as_str() {
            "exit" => break,
            _ => println!("\nRESULT"),
        }
        get_bitcoin_price(&sym).unwrap();
    }
}
