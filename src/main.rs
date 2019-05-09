extern crate reqwest;
use serde::{Deserialize};

#[derive(Deserialize)]
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
    let json: Price = reqwest::get(&url).unwrap().json().unwrap();
    return Some(json.price.parse().unwrap());
}
