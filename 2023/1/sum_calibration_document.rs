use std::fs::File;
use std::io::{self, BufRead};
use std::env;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let mut sum: u32 = 0;

    for line in reader.lines() {
        let filtered_chars: Vec<char>= line?
            .chars()
            .filter(|c| c.is_numeric())
            .collect();

        if let Some(first_char) = filtered_chars.first() {
            if let Some(last_char) = filtered_chars.last() {
                let value: u32 = first_char.to_digit(10).unwrap_or(0) * 10 + last_char.to_digit(10).unwrap_or(0);
                println!("{}", value);
                sum += value;
            }
        }
    }

    println!("Sum: {}", sum);

    Ok(())
}
