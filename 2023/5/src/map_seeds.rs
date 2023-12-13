use std::fs::read_to_string;
use std::io;
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
mod tests {
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

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let content: String = read_to_string(file_path).expect("read file");
    let key_values = parse_key_values_config(&content);
    dump_key_values(&key_values);

    Ok(())
}
