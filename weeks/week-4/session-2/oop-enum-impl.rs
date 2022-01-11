#[derive(Debug)]
enum Blockchain {
    SOLANA,
    ETHEREUM,
    BITCOIN,
}
impl Blockchain{
    fn selected_transaction(&self) -> String {
        match self {
            Blockchain::SOLANA =>
                String::from("Doing transaction in SOLANA"),
            Blockchain::ETHEREUM =>
                String::from("Doing transaction in ETHEREUM"),
            Blockchain::BITCOIN =>
                String::from("Doing transaction in BITCOIN"),
        }
    }
}
fn main() {
    let solana = Blockchain::SOLANA;
    println!("{}", solana.selected_transaction());

    let ethereum = Blockchain::ETHEREUM;
    println!("{}", ethereum.selected_transaction());

    let bitcoin = Blockchain::BITCOIN;
    println!("{}", bitcoin.selected_transaction());

}