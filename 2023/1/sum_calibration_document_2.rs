use std::fs::File;
use std::io::{self, BufRead};
use std::env;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let mut sum: u32 = 0;

    const REPLACEMENTS: [&str; 10] = ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

    for line_result in reader.lines() {
        let line = line_result?.trim().to_string();
        println!("{}", line);
        let modified_line = REPLACEMENTS.iter().enumerate().fold(line, |acc, (i, s)| {
            let replacement_char = char::from(i as u8 + 48);
            acc.replace(s, &format!("{}{}{}", s, replacement_char, s))
        });
        println!("{}", modified_line);

        let filtered_chars: Vec<char>= modified_line
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
