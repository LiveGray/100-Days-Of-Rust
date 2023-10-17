use std::io::{stdin,stdout,Write};

const NO_OF_DAYS_IN_YEAR: u32 = 365;

fn convert_age_to_days(age: u32) -> u32 {
    return age * NO_OF_DAYS_IN_YEAR;
}

fn main() {

   loop {
        // Prompt user for age
        let mut age_str = String::new();

        println!("Plese enter your age in years. We will convert it to days.");
        // Ensure that output is emitted immediately before reading 
        // user input.
        let _ = stdout().flush();
        stdin().read_line(&mut age_str).expect("Failed to read line");

        let age = match age_str.trim().parse::<u32>() {
            Ok(num) => num,
            Err(e) => {
                println!("Error: {}", e);
                continue;
            }
        };
        println!("You are {} days old!", convert_age_to_days(age));
        break;
   }
  

   // Parse to int

   // Convert to days 

   // Output results


}
