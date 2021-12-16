enum Blockchain {
    SOLANA,
    ETHEREUM,
    BITCOIN,
}

fn main() {
    let blockchain = Blockchain::SOLANA;
    if let Blockchain::SOLANA = blockchain {
        println!("SOLANA");
    } else if let Blockchain::ETHEREUM = blockchain {
        println!("ETHEREUM");
    } else if let Blockchain::BITCOIN = blockchain {
        println!("BITCOIN");
    }
    // match blockchain {
    //     Blockchain::SOLANA => println!("SOLANA"),
    //     Blockchain::ETHEREUM => println!("ETHEREUM"),
    //     Blockchain::BITCOIN => println!("BITCOIN"),
    // }
}