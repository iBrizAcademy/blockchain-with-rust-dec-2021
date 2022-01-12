// CLI based crypto ticker
#![allow(warnings)]
#[macro_use]
extern crate serde;
extern crate reqwest;
use serde_json::Error;

use std::io;

// use serde_json::Result;

#[derive(Debug, Serialize, Deserialize)]
pub struct CMCResponse {
    status: Status,
    data: Data,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    id: i64,
    symbol: String,
    name: String,
    amount: i64,
    last_updated: String,
    quote: Quote,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Quote {
    #[serde(rename = "USD")]
    usd: Usd,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Usd {
    price: f64,
    last_updated: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Status {
    timestamp: String,
    error_code: i64,
    error_message: Option<serde_json::Value>,
    elapsed: i64,
    credit_count: i64,
    notice: Option<serde_json::Value>,
}


#[tokio::main]
async fn main() -> Result<(), Error> {

    
    let api_key:String = String::from("79f4f1ab-681e-4809-8c13-6edaf490334e");
    
    let mut symbol = String::new();
    println!("Enter the Symbol for the Token: ");
    let b1 = std::io::stdin().read_line(&mut symbol).unwrap();

    let url = format!(
        "https://pro-api.coinmarketcap.com/v1/tools/price-conversion?symbol={}&amount=1&convert=USD&CMC_PRO_API_KEY={}", symbol, api_key
        
    );
    

    let resp = reqwest::get(url).await.unwrap();
    
    let post: CMCResponse = resp.json().await.unwrap();

    let price = post.data.quote.usd.price;
    println!("The price of {} is {}",symbol, price);

    Ok(())

}
