use std::fs::File;
use std::io::{self, BufRead};
use std::env;
use regex::Regex;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let regex = Regex::new(r"^Game (\d+)").expect("Invalid regex");

    let total: i32 = reader
        .lines()
        .filter_map(|line| {
            if let Ok(line) = line {
                if let Some(captures) = regex.captures(&line) {
                    if let Some(group) = captures.get(1) {
                        return group.as_str().parse::<i32>().ok();
                    }
                }
            }
            None
        })
        .sum();

    println!("Total: {}", total);

    Ok(())
}
