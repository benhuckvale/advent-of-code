use std::fs::File;
use std::io::{self, BufRead};
use std::env;
use std::collections::HashSet;
use itertools::Itertools;

fn find_symbol_positions(line: &str) -> HashSet<usize> {
    line.chars()
        .enumerate()
        .filter(|&(_, c)| c != '.' && c.is_ascii_punctuation())
        .map(|(i, _)| i)
        .collect()
}

fn visualize_symbol_positions(symbol_positions: &HashSet<usize>, line_length: usize) -> String {
    (0..line_length)
        .map(|i| if symbol_positions.contains(&i) { '^' } else { ' ' })
        .collect()
}

/*
 * Sums numbers found in a string touched by positions given
 *
 * This function takes a string input line e.g.:
 * ...10.....20.....30......40
 * And a set of positions
 * 2, 12, 17, 22
 * and returns the sum of all numbers formed from contiguous digits touched by these positions.
 *
 * For example:
 * Position 2 touches the 10:
 * ..X10
 * Whilst position 17 touches the 30:
 * ...10.....20.....X0
 * 20 is also touched, but 40 is not. So the return result is 60.
 *
 */
fn sum_touched_numbers(line: &str, touch_positions: &HashSet<usize>) -> i32 {
    let mut current_number = 0;
    let mut sum = 0;
    let mut symbol_seen = false;

    for (i, c) in line.chars().enumerate() {
        symbol_seen |= touch_positions.contains(&i);
        if c.is_digit(10) {
            let digit = c.to_digit(10).unwrap() as i32;
            current_number = current_number * 10 + digit;
        } else {
            if symbol_seen {
                sum += current_number;
            }
            symbol_seen = touch_positions.contains(&i);
            current_number = 0;
        }
    }
    if symbol_seen {
        sum += current_number
    }
    return sum;
}

fn combine_symbol_positions<'a>(positions: impl IntoIterator<Item = &'a HashSet<usize>>) -> HashSet<usize> {
    positions.into_iter().flat_map(|set| set.iter().cloned()).collect()
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let preprocessed_lines_iter = std::iter::once(String::new()) // Dummy line before
        .chain(reader.lines().map(|line_result| line_result.unwrap()))
        .chain(std::iter::once(String::new())) // Dummy line after
        .map(|line| (find_symbol_positions(&line), line));

    let total: i32 = preprocessed_lines_iter
        .tuple_windows::<(_, _, _)>()
        .filter_map(|window| {
            let (symbol_positions0, _   ) = &window.0;
            let (symbol_positions1, line) = &window.1;
            let (symbol_positions2, _   ) = &window.2;
            let combined_positions = &combine_symbol_positions(vec![symbol_positions0, symbol_positions1, symbol_positions2]);
            let sum: i32 = sum_touched_numbers(line, combined_positions);
            println!("{} => {}", line, sum);
            if false {
                println!("{}", visualize_symbol_positions(combined_positions, line.len()));
            }
            Some(sum)
        })
        .sum();

    println!("Total: {}", total);

    Ok(())
}

