#[derive(Debug)]
enum Blockchain {
    SOLANA,
    ETHEREUM,
    BITCOIN,
}

fn main() {
    let solana = Blockchain::SOLANA;
    let ethereum = Blockchain::ETHEREUM;
    let bitcoin = Blockchain::BITCOIN;

    println!("{:?}", solana);
    println!("{:?}", ethereum);
    println!("{:?}", bitcoin);
}