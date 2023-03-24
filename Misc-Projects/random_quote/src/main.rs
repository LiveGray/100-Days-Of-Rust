use serde::{Deserialize, Serialize};
use reqwest::blocking::get;

pub type Response = Vec<Quote>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Quote {
    pub q: String,
    pub a: String,
    pub h: String,
}

fn main() {
    let res = get("https://zenquotes.io/api/random").unwrap();
    let quotes = res.json::<Response>().unwrap();
    for quote in quotes {
        println!("Author: {}", quote.a);
        println!("Quote: {}", quote.q);
    }
}
