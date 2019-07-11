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

// Example cargo run BTCUSDT LTCUSDT | cut -d : -f 2

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
            println!("{}: {}", t, price.price);
            Ok(())
        })
        .map_err(|e| panic!("Error making request: {}", e));

    tokio::run(work);

}

