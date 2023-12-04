use std::fs::File;
use std::io::{self, BufRead};
use std::env;
use regex::Regex;
use std::collections::HashMap;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let colour_regex = Regex::new(r"(\d+)\s(red|green|blue)").expect("Invalid regex");

    let total: i32 = reader
        .lines()
        .filter_map(|line| {
            if let Ok(line) = line {
                let max_value_for_colour = colour_regex.captures_iter(&line)
                    .filter_map(|captures| {
                        let colour = captures.get(2).map(|m| m.as_str().to_string());
                        let value = captures.get(1).and_then(|m| m.as_str().parse::<i32>().ok());
                        colour.zip(value)
                    })
                    .fold(HashMap::new(), |mut acc, (colour, value)| {
                        let entry = acc.entry(colour).or_insert(value);
                        *entry = value.max(*entry);
                        acc
                    });
                let product: i32 = max_value_for_colour.values().cloned().fold(1, |acc, x| acc * x);
                return Some(product);
            }
            None
        })
        .sum();

    println!("Total: {}", total);

    Ok(())
}
