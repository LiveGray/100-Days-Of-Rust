use barbecue_skewers_botiboti::{check_skwrs,read_lines};

fn main() {
    let mut skewers = Vec::new();

    // Input in "barbecue.txt"
    if let Ok(lines) = read_lines("barbecue.txt") {
        for line in lines.map_while(Result::ok) {
            skewers.push(line)
        }
    }

    let skewer_refs: Vec<&str> = skewers.iter().map(|s| s.as_str()).collect();

    println!("{:?}", check_skwrs(&skewer_refs));
}