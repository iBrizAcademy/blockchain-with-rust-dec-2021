use serde::Deserialize;
use dotenv::dotenv;
use reqwest::header::HeaderMap;
use std::env;


#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct USD {
    price: f64,
    last_updated: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code, non_snake_case)]
pub struct Quotes {
    USD: USD,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct Data {
    id: i32,
    symbol: String,
    name: String,
    amount: i32,
    last_updated: String,
    quote: Quotes,
}

#[derive(Debug, Deserialize)]
pub struct ResponseData {
    data: Data,
}

#[tokio::main]
pub async fn get_bitcoin_price(symbol:&str)->Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let api_key = env::var("API_KEY")?;
    let url = format!("https://pro-api.coinmarketcap.com/v1/tools/price-conversion?symbol={}&amount=1", symbol);
    let mut headers= HeaderMap::new();
    headers.insert("X-CMC_PRO_API_KEY", api_key.parse().unwrap());

    let client=reqwest::Client::new();
    let resp = client
        .get(url)
        .headers(headers)
        .send()
        .await?;

    let resp_json = resp.json::<ResponseData>().await?;
    println!("current price of {:#?} is {:#?} USD\n", resp_json.data.name, resp_json.data.quote.USD.price);

    Ok(())
}

