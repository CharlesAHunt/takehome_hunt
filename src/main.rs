use std::collections::HashMap;
use sonic_rs::{Deserialize, Serialize};
use chrono::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let ds = DataStore {
        price_history:HashMap::new(),
        std_deviations:HashMap::new(),
        most_recent_price
    };

    let resp = reqwest::get("https://api.gemini.com/v1/pricefeed")
        .await?
        .json::<Vec<SymbolPricePairAPI>>()
        .await?;

    let resp_iterator = resp.into_iter();

    for val in resp_iterator {
        let price:f32 = val.price.parse().unwrap();
        let pair_stat = ds.std_deviations.get(&val.pair).unwrap();
        if (price - pair_stat.mean) > pair_stat.std_dev {
            let alert = Alert {
                timestamp:  Utc::now(),
                log_level: String::from("INFO"),
                trading_pair: val.pair,
                is_deviation: true,
                data: AlertMetaData {
                    last_price: price,
                    avg_price: pair_stat.mean,
                    deviation: price / pair_stat.std_dev,
                    price_change: ds.most_recent_price - price
                }
            };
            println!("{:?}", alert);
        }



        //Update the map
        //Recalc std dev
        //update latest std dev
    }
    
    println!("{resp:#?}");
    Ok(())
}

struct PairStats {
    std_dev:f32,
    mean: f32
}

struct DataStore {
    price_history: HashMap<String, Vec<SymbolPricePairInternal>>,
    std_deviations: HashMap<String, PairStats>,
    most_recent_price: f32 //Price history vector order is not guaranteed
}
impl DataStore {

}

#[derive(Serialize, Deserialize, Debug)]
struct SymbolPricePairAPI {
    pair: String,
    price: String,
    percentChange24h: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct SymbolPricePairInternal {
    pair: String,
    price: f32
}

#[derive(Serialize, Deserialize, Debug)]
struct Alert {
    timestamp: DateTime<Utc>,
    log_level: String,
    trading_pair: String,
    is_deviation: bool,
    data: AlertMetaData
}

#[derive(Serialize, Deserialize, Debug)]
struct AlertMetaData {
    last_price: f32,
    avg_price: f32,
    deviation: f32,
    price_change: f32
}