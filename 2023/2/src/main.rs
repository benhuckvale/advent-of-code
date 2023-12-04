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

    let id_regex = Regex::new(r"^Game (\d+)").expect("Invalid regex");
    let colour_regex = Regex::new(r"(\d+)\s(red|green|blue)").expect("Invalid regex");

    let allowed_max_for_colour: HashMap<_, _> = vec![
        ("red".to_string(), 12),
        ("green".to_string(), 13),
        ("blue".to_string(), 14),
    ]
    .into_iter()
    .collect();

    let total: i32 = reader
        .lines()
        .filter_map(|line| {
            if let Ok(line) = line {
                if let Some(captures) = id_regex.captures(&line) {
                    if let Some(group) = captures.get(1) {
                        let id_opt: Option<i32> = group.as_str().parse::<i32>().ok();
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
                        let result = max_value_for_colour.iter().all(|(key, value)| {
                            allowed_max_for_colour.get(key).map_or(true, |&other_value| value <= &other_value)
                        });
                        return if result { id_opt } else { None };
                    }
                }
            }
            None
        })
        .sum();

    println!("Total: {}", total);

    Ok(())
}
