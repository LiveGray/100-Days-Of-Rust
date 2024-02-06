use std::io;

const NUM_DAYS_IN_YEAR: u32 = 365;

fn main() {

    let guess = get_input();
    let age: u32 = calculate_age(&guess);
    println!("You are roughly {age} days old!");

    fn get_input() -> u32 {
        loop {
            println!("Please input your age in years:");
            let mut guess = String::new();
            io::stdin()
                .read_line(&mut guess)
                .expect("Could not read age. Make sure an integer is used.");
            let guess: u32 = match guess.trim().parse() {
                Ok(num) => num, 
                Err(_) => continue,
            };
            return guess;
        }
    }

    fn calculate_age(num: &u32) -> u32 {
        num * NUM_DAYS_IN_YEAR
    }
}
