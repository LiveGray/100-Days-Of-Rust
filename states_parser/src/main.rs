use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;

#[derive(Debug, Serialize, Deserialize)]
struct State {
    state: String,
    abbreviation: String,
    capital: String,
}

fn main() {

    let mut file = File::open("src/states.json").expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read file");
    let states: Vec<State> = serde_json::from_str(&contents).expect("Unable to parse JSON");


    let names: Vec<String> = states.iter().map(|s| s.state.clone()).collect();
    let abbreviations: Vec<String> = states.iter().map(|s| s.abbreviation.clone()).collect();
    let capitals: Vec<String> = states.iter().map(|s| s.capital.clone()).collect();

    let mut i = 0;
    let len = names.len();

    while i < len {
        println!("Name: {:?} | Abbreviation: {:?} | Capital: {:?}", names[i], abbreviations[i], capitals[i]);
        i = i+1;
    }
}
