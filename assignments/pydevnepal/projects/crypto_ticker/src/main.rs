
use reqwest::header::HeaderMap;
use serde::Deserialize;
use dotenv::dotenv;
use std::env;
use std::io;


#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct USD{
	price: f64,
	last_updated:String
}

#[derive(Debug, Deserialize)]
#[allow(dead_code, non_snake_case)]
struct Quotes{
	USD: USD
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct Data{
    id: i32,
    symbol: String,
    name: String,
    amount: i32,
    last_updated: String,
    quote: Quotes
}

#[derive(Debug, Deserialize)]
struct ResponseData{
    data: Data,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let api_key = env::var("API_KEY")?;

    println!("Please enter symbol of the cyrpto: ");
    let mut sym = String::new();
    io::stdin()
            .read_line(&mut sym)
            .expect("Failed to read line");
    let url = format!("https://pro-api.coinmarketcap.com/v1/tools/price-conversion?symbol={}&amount=1", sym);

    let mut headers= HeaderMap::new();
    headers.insert("X-CMC_PRO_API_KEY", api_key.parse().unwrap());

    let client=reqwest::Client::new();
    let resp = client
        .get(url)
        .headers(headers)
        .send()
        .await?;

    let resp_json = resp.json::<ResponseData>().await?;
    println!("current price is {:#?}", resp_json.data.quote.USD.price);

    Ok(())
}