mod models;

use std::collections::HashMap;
use std::future::Future;
use std::process::exit;
use chrono::prelude::*;
use reqwest::Error;
use crate::models::DataStore;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let ds = DataStore {
        price_history:HashMap::new(),
        stats:HashMap::new()
    };

    match monitor_currency_pairs(ds).await {
        Ok(data_store) => {
            let stats_debug = data_store.stats;
            println!("Stats Results: {stats_debug:#?}");
            exit(0)
        },
        Err(err) => {
            eprintln!("Error: {}", err);
            exit(1);
        }
    }

}

async fn monitor_currency_pairs(mut ds: DataStore)-> Result<DataStore, Error> {
    let resp = match reqwest::get("https://api.gemini.com/v1/pricefeed").await {
        Ok(response) => {
            response
        }
        Err(error) => {
            models::Alert {
                timestamp: Utc::now(),
                log_level: String::from("ERROR"),
                trading_pair: None,
                is_deviation: true,
                data: models::AlertMetaData {
                    error_message: Option::from(error.to_string()),
                    last_price: None,
                    avg_price: None,
                    deviation: None,
                    price_change: None,
                }
            };
            exit(1);
        }
    }.json::<Vec<models::SymbolPricePairAPI>>().await?;

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
                        trading_pair: Option::from(pair_key.to_owned()),
                        is_deviation: true,
                        data: models::AlertMetaData {
                            error_message: None,
                            last_price: Option::from(latest_price),
                            avg_price: Option::from(stats.mean),
                            deviation: Option::from(if stats.std_dev == 0.0 { stats.std_dev } else { latest_price / stats.std_dev }),
                            price_change: Option::from(stats.most_recent_price - latest_price)
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

    Ok(ds)
}
