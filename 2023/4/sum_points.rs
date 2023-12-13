use std::fs::File;
use std::io::{self, BufRead};
use std::env;
use std::collections::HashSet;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let sum: i32 = reader
        .lines()
        .filter_map(|line_result| {
            let line = line_result.unwrap();
            let intersection_set: HashSet<i32> = line
                .split([':', '|'])
                .skip(1)
                .map(|part| -> HashSet<i32> {
                    part.trim()
                        .split(' ')
                        .filter_map(|s| s.parse::<i32>().ok())
                        .collect()
                })
                .reduce(|set1, set2| &set1 & &set2)
                .unwrap();
            Some(2_i32.pow(intersection_set.len() as u32) / 2)
        })
        .sum();

    println!("Sum: {}", sum);

    Ok(())
}
