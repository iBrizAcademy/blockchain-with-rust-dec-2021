#[macro_use]
extern crate serde_derive;

#[derive(Debug, Serialize, Deserialize)]
pub struct CMCResponse {
    status: Status,
    data: Vec<Data>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    id: i64,
    symbol: String,
    name: String,
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
    let client = reqwest::Client::new();
    let res = client.get("https://pro-api.coinmarketcap.com/v1/cryptocurrency/listings/latest?start=1&limit=10&convert=USD")
                    .header("X-CMC_PRO_API_KEY", "f4663923-075d-4ec5-b1fc-7d1525d07b4d")
                    .send()
                    .await?;
    let response : CMCResponse = res.json::<CMCResponse>().await?;
    let solana: Vec<&Data> = response
                            .data
                            .iter()
                            .filter(|data| data.symbol == "SOL")
                            .collect();
    println!("Price of solana today is: {}", solana[0].quote.usd.price);
    Ok(())
}