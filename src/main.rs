// extern crate reqwest;
extern crate hyper;
extern crate hyper_tls;
extern crate serde_json;

use hyper::Client;
use hyper_tls::HttpsConnector;

use futures::{stream, Future, Stream}; // 0.1.25

use hyper::http::Uri;
use tokio; // 0.1.15

use std::env;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
struct Price {
    mins: u16,
    price: String,
}

// Examples:
// bticker BTCUSDT LTCUSDT | cut -d : -f 2
// bticker BTCUSDT ETHUSDT DOTBTC LINKBTC | sort | tr '\n' '|'

fn get_precision(pr: i32, v: &f32) -> f32 {
    let log10 = v.log10().ceil() as i32;
    let new_pr = (log10-pr) as f32;
    let modder = 10_f32.powf(new_pr);
    let vdiff = v%modder;
    let mut result = v-vdiff;
    if result < 1_f32 {
        let multiplier = 10_f32.powf(-new_pr);
        result = (result * multiplier).round()/multiplier;
    } 
    return result
}

fn format_price(x: f32) -> String {
    if x >= 10000.0 {
        format!("{}", get_precision(5, &x))
    } else if x >= 10.0 {
        format!("{}", get_precision(4, &x))
    } else {
        format!("{}", get_precision(3, &x))
    }
}

fn main() {
    let https = HttpsConnector::new(4).expect("TLS initialization failed");
    let client = Client::builder()
        .build::<_, hyper::Body>(https);
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("Provide argument");
    }

    let work = stream::iter_ok(args.into_iter().skip(1))
        .map(move |ticker| {
            let uri = format!("https://api.binance.com/api/v3/avgPrice?symbol={}", (ticker).to_uppercase()).parse::<Uri>().unwrap();
            client.get(uri)
            .and_then(|res| {
                res.into_body().concat2()
            })
            .and_then(move |body| {
                let price: Price = serde_json::from_slice(&(body)).unwrap();
                Ok(((ticker).to_uppercase(), price))
            })
        })
        .buffer_unordered(5)
        .for_each(|(t, price)| {
            println!("{}: {}", t, format_price(price.price.parse::<f32>().unwrap()));
            Ok(())
        })
        .map_err(|e| panic!("Error making request: {}", e));
    tokio::run(work);
}

