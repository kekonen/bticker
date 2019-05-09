extern crate reqwest;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
struct Price {
    mins: u16,
    price: String,
}

fn main() {
    match get_latest_price("BTCUSDT") {
        Some(price) => println!("{}", price),
        None => println!("Kek!"),
    }
}

fn get_latest_price(symbol: &str) -> Option<f32> {
    let url = format!("https://api.binance.com/api/v3/avgPrice?symbol={}", symbol);
    match reqwest::get(&url){
        Ok(mut response) => {
            match response.json::<Price>() {
                Ok(json) => {
                    //println!("{:?}", json.price);
                    //return Some(3.12);
                    match json.price.parse::<f32>() {
                        Ok(p) => return Some(p),
                        _ => return None,
                    };
                },
                _ => return None,
            };
        },
        _ => return None,
    };
    //return Some(json.price.parse().unwrap());
}
