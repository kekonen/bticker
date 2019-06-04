extern crate reqwest;
use std::env;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
struct Price {
    mins: u16,
    price: String,
}

// Example cargo run BTCUSDT LTCUSDT | cut -d : -f 2

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => println!("Provide argument"),
        _ => {
            for ticker in &args[1..] {
                let ticker = (ticker).to_uppercase();
                match get_latest_price(&ticker) {
                    Some(price) => println!("{}: {}", ticker, price),
                    None => println!("Kek!"),
                }
            }
        }
    };

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
