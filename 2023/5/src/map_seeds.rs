use std::fs::read_to_string;
use std::io;
use std::ops::Range;
use std::env;
use regex::Regex;
use std::collections::HashMap;

type StringStringsMap = HashMap<String, Vec<String>>;

/*
 * Parse a "key values config" file
 *
 * Given an input string, which should contain newlines, this function looks for mappings of a key,
 * delimited by a colon, to values. The values can be spread over several lines, but are marked as
 * ending for that key by a blank line.
 *
 * Returns a HashMap mapping each key to its values array.
 */
pub fn parse_key_values_config(input: &str) -> StringStringsMap {
    let mut result = HashMap::new();
    let regex_pattern = r"(?m)^([^:\n]+):\s*([^:\n]+(?:\s*[^:\n]+)*)$";
    let regex = Regex::new(&regex_pattern).unwrap();

    for capture in regex.captures_iter(input) {
        let key = capture[1].trim().to_string();
        let values = capture[2]
            .split('\n')
            .map(|v| v.trim().to_string())
            .collect::<Vec<_>>();
        result.insert(key, values);
    }

    result
}

fn dump_key_values(map: &StringStringsMap) {
    for (key, values) in map {
        println!("Key: {}; Values: {:?}", key, values);
    }
}

#[cfg(test)]
mod parse_key_values_config_tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_parse_key_values_config() {
        println!("hello");
        let input = indoc! {"
            key0: value0

            key1:
            value1
            value2

            key2:
            value3
        "};

        let expected_result = StringStringsMap::from([
            ("key0".to_string(), vec!{"value0".to_string()}),
            ("key1".to_string(), vec!{"value1".to_string(), "value2".to_string()}),
            ("key2".to_string(), vec!{"value3".to_string()}),
        ]);
        let result = parse_key_values_config(&input);

        assert_eq!(result, expected_result);
    }
}


#[derive(Debug)]
struct OffsetIntervalMap {
    intervals: Vec<(Range<i64>, i64)>,
}


impl OffsetIntervalMap {
    fn new() -> Self {
        OffsetIntervalMap { intervals: Vec::new() }
    }

    fn insert(&mut self, range: Range<i64>, value: i64) {
        self.intervals.push((range, value));
    }

    fn get(&self, key: i64) -> Option<i64> {
        self.intervals
            .iter()
            .find_map(|(interval, value)| {
                if interval.contains(&key) {
                    let offset = key - interval.start;
                    Some(offset + value)
                } else {
                    None
                }
            })
            .or_else(|| Some(key))
    }
}


#[cfg(test)]
mod offset_interval_map_tests {
    use super::*;

    #[test]
    fn test_offset_interval_map() {
        let mut offset_map = OffsetIntervalMap::new();

        // Add test mappings
        offset_map.insert(0..5, 10);
        offset_map.insert(20..30, 50);

        // Test entries within the ranges
        assert_eq!(offset_map.get(3), Some(13));
        assert_eq!(offset_map.get(25), Some(55));

        // Test an entry outside the example ranges (using the key itself as default)
        assert_eq!(offset_map.get(40), Some(40));
    }
}


fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let content: String = read_to_string(file_path).expect("read file");
    let key_values = parse_key_values_config(&content);
    //dump_key_values(&key_values);

    let mut map: HashMap<String, (String, OffsetIntervalMap)> = HashMap::new();
    let mut start_key = String::new();
    let mut start_values: Vec<i64> = Vec::new();

    for (key, values) in key_values.iter() {
        // Define a regex pattern for extracting map names
        let regex_pattern = r"^(?P<from_map>[^\s]+)-to-(?P<to_map>[^\s]+)\s+map$";
        let regex = Regex::new(regex_pattern).unwrap();
        if let Some(captures) = regex.captures(key) {
            if let (Some(from_map), Some(to_map)) = (captures.name("from_map"), captures.name("to_map")) {
                let from_map_string: String = from_map.as_str().to_string();
                let to_map_string: String = to_map.as_str().to_string();
                let mut offset_map: OffsetIntervalMap = OffsetIntervalMap::new();

                // Have the keys, now handle the values
                for line in values.iter().flat_map(|s| s.split('\n')) {
                    let parts: Vec<i64> = line.split_whitespace().map(|s| s.parse().unwrap()).collect();
                    let (start1, start2, count) = (parts[0], parts[1], parts[2]);
                    // Store range mapping:
                    offset_map.insert(start2..start2+count, start1);
                }

                map.entry(
                    from_map_string.clone()
                ).or_insert(
                    (to_map_string.clone(), offset_map)
                );
 
            }
        } else {
            // It is the line with the start values (the seeds)
            start_values = values
                .join(" ")
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();
            // Would have used:
            //start_key = key.clone();
            // But 'seeds' is different to 'seed', so we cannot rely on that. So have to hardcode:
            start_key = "seed".to_string();
        }
    }

    if false {
        println!("map: {:?}", map);
        println!("start_key: {}", start_key);
        println!("start_values: {:?}", start_values);
    }

    const REQUIRED_FINAL_KEY: &str = "location";

    let result: Option<i64> = start_values
        .chunks_exact(2)
        .flat_map(|pair| {
            if let [start, count] = pair {
                println!("{} {}", start, count);
                let range = *start..(*start + *count);
                range.map(|start_value| {
                    if start_value % 10000000 == 0 {
                        println!("{}", start_value);
                    }
                    std::iter::successors(
                        Some((start_key.clone(), start_value)),
                        |(key, value)| {
                            //println!("key: {}, value: {}", key, value);
                            match map.get(key) {
                                Some(&(ref next_key, ref offset_map)) => {
                                    Some((next_key.clone(), offset_map.get(*value).unwrap_or_default()))
                                },
                                None => None,
                            }
                        }
                    )
                    .last()
                    .and_then(|(final_key, final_value)| {
                        //println!("{} {}", final_key, final_value);
                        if final_key == REQUIRED_FINAL_KEY {
                            Some(final_value)
                        } else {
                            None
                        }
                    })
                }).min()
            } else {
                None
            }
        })
        .flatten()
        .min();

    match result {
        Some(value) => println!("Lowest: {}", value),
        None => println!("Lookup failed."),
    }

    Ok(())
}
