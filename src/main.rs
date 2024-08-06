mod models;

use std::collections::HashMap;
use chrono::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    //This is a stand-in for an actual persistent datastore, perhaps would use something like etcd instead
    let ds = models::DataStore {
        price_history:HashMap::new(),
        stats:HashMap::new()
    };

    let f = monitor_currency_pairs(ds).await?;
    monitor_currency_pairs(f).await?;

    Ok(())
}

async fn monitor_currency_pairs(mut ds: models::DataStore)-> Result<models::DataStore, reqwest::Error> {
    let resp = reqwest::get("https://api.gemini.com/v1/pricefeed")
        .await?
        .json::<Vec<models::SymbolPricePairAPI>>()
        .await?;

    let resp_iterator = resp.into_iter();

    for val in resp_iterator {
        let latest_price:f32 = val.price.parse().unwrap();
        let pair_key = &val.pair;
        let pair_stat = match ds.stats.get(pair_key) {
            None => {
                let price_history = match ds.price_history.get(pair_key) {
                    None => vec![latest_price],
                    Some(history) => history.to_owned()
                };
                ds.price_history.insert(pair_key.to_owned(), price_history);
                models::PairStats{
                    std_dev: 0.0,
                    mean: latest_price,
                    most_recent_price: latest_price
                }
            },
            Some(stats) => {
                if (latest_price - stats.mean) == stats.std_dev {
                    let alert = models::Alert {
                        timestamp: Utc::now(),
                        log_level: String::from("WARN"),
                        trading_pair: pair_key.to_owned(),
                        is_deviation: true,
                        data: models::AlertMetaData {
                            last_price: latest_price,
                            avg_price: stats.mean,
                            deviation: if stats.std_dev == 0.0 { 0.0 } else { latest_price / stats.std_dev },
                            price_change: stats.most_recent_price - latest_price
                        }
                    };
                    println!("{:?}", sonic_rs::to_string(&alert).unwrap());
                };
                let price_history = match ds.price_history.get(pair_key) {
                    None => vec![latest_price],
                    Some(history) =>  [history.to_owned(), vec![latest_price]].concat()
                };
                ds.price_history.insert(pair_key.to_owned(), price_history.to_owned());
                let new_std_dev = statistical::standard_deviation(&price_history, None);
                let new_mean = statistical::mean(&price_history);
                models::PairStats{
                    std_dev: new_std_dev,
                    mean: new_mean,
                    most_recent_price: latest_price
                }
            }
        };

        ds.stats.insert(pair_key.to_owned(), pair_stat);
    }

    let debug_stats = &ds.stats;
    // println!("{debug_stats:#?}");
    Ok(ds)
}
