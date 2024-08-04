use std::collections::HashSet;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let resp = reqwest::get("https://api.gemini.com/v1/pricefeed")
        .await?
        .json::<Vec<SymbolPricePair>>()
        .await?;


    println!("{resp:#?}");

    Ok(())
}

use sonic_rs::{Deserialize, Serialize};
// sonic-rs re-exported them from serde
// or use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct SymbolPricePair {
    pair: String,
    price: String,
    percentChange24h: String,
}