use std::io;
mod day1;
mod day2;
fn main() {
    
    println!("Please enter which day you want to run: ");

    let mut input = String::new();

    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            // Trim whitespace and newline characters from the input
            let trimmed_input = input.trim();

            match trimmed_input.parse::<u32>() {
                Ok(day) => {
                    match day {
                        1 => day1::day1(),
                        2 => day2::day2(),
                        _ => println!("Invalid day number"),
                    }
                },
                Err(error) => println!("Error parsing input: {}", error),
            }
        }
        Err(error) => println!("Error reading input: {}", error),
    }


}
