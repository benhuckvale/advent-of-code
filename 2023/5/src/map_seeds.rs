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
        if let Some((range, value)) = self.intervals.iter().rev().find(|&&(ref r, _)| r.contains(&key)) {
            Some(value + key - range.start)
        } else {
            Some(key)
        }
    }

    fn get_with_interval(&self, key: i64) -> Option<(i64, usize)> {
        if let Some((index, (range, value))) = self.intervals.iter().enumerate().rev().find(|&(_, &(ref r, _))| r.contains(&key)) {
            // One-based index (sorry Dijkstra)...
            Some((value + key - range.start, index + 1))
        } else {
            // ...because we'll use 0 as the id to indicate no range was matched
            Some((key, 0))
        }
    }
}


#[cfg(test)]
mod offset_interval_map_tests {
    use super::*;

    #[test]
    fn test_offset_interval_map() {
        let mut offset_map = OffsetIntervalMap::new();

        // Add test mappings
        offset_map.insert(0..5, 10); // which will get id 1
        offset_map.insert(20..30, 50); //which will get id 2

        // Test entries within the ranges
        assert_eq!(offset_map.get(3), Some(13));
        assert_eq!(offset_map.get(25), Some(55));

        // Test an entry outside the example ranges (using the key itself as default)
        assert_eq!(offset_map.get(40), Some(40));

        // Test entries within the ranges using get_with_interval, returning the interval id
        assert_eq!(offset_map.get_with_interval(3), Some((13, 1)));
        assert_eq!(offset_map.get_with_interval(25), Some((55, 2)));

        // Test an entry outside the example ranges
        // which is expecting to return the key itself as default with interval id 0
        assert_eq!(offset_map.get_with_interval(40), Some((40, 0)));
    }
}

/*
 * Computes final successor value and path of interval ids taken to reach it via interval maps given.
 */
fn ranges_succession_path(start_key: &str, start_value: i64, map: &HashMap<String, (String, OffsetIntervalMap)>) -> (i64, Vec<i64>) {
    let mut path = Vec::new();
    let mut current_key = start_key.to_string();
    let mut current_value = start_value;

    loop {
        match map.get(&current_key) {
            Some(&(ref next_key, ref offset_map)) => {
                match offset_map.get_with_interval(current_value) {
                    Some((next_value, index)) => {
                        path.push(index as i64);
                        current_key = next_key.clone();
                        current_value = next_value;
                    }
                    None => break (current_value, path),
                }
            }
            None => break (current_value, path),
        }
    }
}


/*
 * Performs binary search to find first value in the range start to end where is_equal(value)
 * evaluates to False, on the assumption that after that all remaining values in the range
 * also evaluate to False. Apart from that condition the range does not otherwise have to be
 * ordered.
 */
fn binary_search_first_not_equal<F>(start: i64, end: i64, mut is_equal: F) -> i64
where
    F: FnMut(i64) -> bool
{
    let mut low = start;
    let mut high = end;

    while low < high {
        let mid = low + (high - low) / 2;
        match is_equal(mid) {
            true => low = mid + 1,
            false => high = mid
        }
    }

    low
}

mod binary_search_tests {
    use super::binary_search_first_not_equal;

    #[test]
    fn test_binary_search_first_not_equal() {
        // Test case: Searching for the first element not equal to 2
        let values = vec![2, 2, 2, 2, 2, 3, 3, 3, 4, 5];
        let is_equal = |x: i64| values[x as usize] == 2;
        let result = binary_search_first_not_equal(0, values.len() as i64, is_equal);
        assert_eq!(result, 5);
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
                let mut results = Vec::new();

                let mut current_start = range.start;
                while current_start < range.end {
                    let (final_value, current_path) = ranges_succession_path(&start_key, current_start, &map);
                    results.push(final_value);

                    current_start = binary_search_first_not_equal(current_start, range.end, |element| {
                        let (_, path) = ranges_succession_path(&start_key, element, &map);
                        path == current_path
                    });
                }

                Some(results.into_iter().min().unwrap())
            } else {
                None
            }
        })
        .min();

    match result {
        Some(value) => println!("Lowest: {}", value),
        None => println!("Lookup failed."),
    }

    Ok(())
}
