use std::io;
use is_johny_making_progress_botiboti::{parse_miles, progress_days};

fn main() {
    println!("Enter a list of miles (comma separated):");

    let mut input = String::new();
    
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line.");

    let miles: Vec<u32> = parse_miles(&input);

    println!("{}", &progress_days(&miles))
}
