use dotenv::dotenv;
use reqwest::header::{ACCEPT, CONTENT_TYPE};
use serde::Deserialize;
use std::io;

use serde::Serialize;

#[derive(Deserialize, Debug)]
pub struct CMCResponse {
    status: Status,
    data: Vec<Data>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    id: i64,
    symbol: String,
    name: String,
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
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv()?;

    let api_key = std::env::var("API_KEY")?;
    let start = "1";
    let limit = "10";
    let convert = "USD";

    let uri = format!("https://pro-api.coinmarketcap.com/v1/cryptocurrency/listings/latest?start={}&limit={}&convert={}", start, limit, convert);

    let client = reqwest::Client::new();
    let response = client
        .get(uri)
        .header("X-CMC_PRO_API_KEY", format!("{}", api_key))
        .header(CONTENT_TYPE, "application/json")
        .header(ACCEPT, "application/json")
        .send()
        .await?;

    let resp_json = response.json::<CMCResponse>().await?;
    let status = resp_json.status;
    let data = resp_json.data;

    let mut crypto_name = String::new();

    println!("Please enter the crypto you want to find the result of");

    io::stdin()
        .read_line(&mut crypto_name)
        .expect("Please enter string");

    let get_desired = |crypto_name| {
        // let hello = data.into_iter().filter(|crypto| crypto.name == crypto_name);
        // println!("{:#?}", hello);

        println!("{:#?}", status);
        for crypto in data {
            if crypto.name == crypto_name {
                println!("{:#?} ", crypto);
            }
        }
    };

    get_desired(crypto_name.trim());

    Ok(())
}
