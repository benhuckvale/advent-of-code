use std::fs::File;
use std::io::{self, BufRead};
use std::env;
use std::collections::HashSet;
use itertools::Itertools;

fn find_symbol_positions(line: &str) -> HashSet<usize> {
    line.chars()
        .enumerate()
        .filter(|&(_, c)| c == '*')
        .map(|(i, _)| i)
        .collect()
}

/*
 * Return int value of contiguous digits passing through string index given.
 *
 * Given a position in a string, identify if there is a digit char at that position and if there
 * is, uncover the remaining contiguous digits forward and backward. Evaluate those contiguous
 * digits and return the integer value. Otherwise return None.
 */
fn parse_contiguous_digits(s: &str, position: usize) -> Option<i32> {
    let bytes = s.as_bytes();
    let mut start = position;
    let mut end = position;
    while start > 0 && bytes[start - 1].is_ascii_digit() {
        start -= 1;
    }
    while end < s.len() - 1 && bytes[end + 1].is_ascii_digit() {
        end += 1;
    }
    let digits_str: String = s[start..=end].chars().collect();
    digits_str.parse().ok()
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
            let (_,                line_before ) = &window.0;
            let (symbol_positions, line        ) = &window.1;
            let (_,                line_after  ) = &window.2;

            let sum: i32 = symbol_positions
                .iter()
                .filter_map(|&position| {
                    // Parse numbers seen around symbol position:
                    // nw n ne
                    //  w * e
                    // sw s se
                    // And then sum the product of all meaningful pairings.
                    //
                    // Certain pairings do not make sense to consider. E.g. nw*n, because those
                    // are contiguous. So if we parse a digit at n, then nw and ne are not
                    // considered (set to None).
                    let w = parse_contiguous_digits(line, position - 1).unwrap_or(0);
                    let e = parse_contiguous_digits(line, position + 1).unwrap_or(0);

                    let n_opt = parse_contiguous_digits(line_before, position);
                    let n = n_opt.unwrap_or(0);
                    let (nw, ne) = match n_opt {
                        Some(_) => (0, 0),
                        None => (
                            parse_contiguous_digits(line_before, position - 1).unwrap_or(0),
                            parse_contiguous_digits(line_before, position + 1).unwrap_or(0),
                        ),
                    };

                    let s_opt = parse_contiguous_digits(line_after, position);
                    let s = s_opt.unwrap_or(0);
                    let (sw, se) = match s_opt {
                        Some(_) => (0, 0),
                        None => (
                            parse_contiguous_digits(line_after, position - 1).unwrap_or(0),
                            parse_contiguous_digits(line_after, position + 1).unwrap_or(0),
                        ),
                    };
                    if false {
                        println!("{} {} {}; {} * {}; {} {} {}", nw, n, ne, w, e, sw, s, se);
                    }
                    Some(
                        nw*ne + nw*w + nw*e + nw*sw + nw*s + nw*se + 
                        n*w + n*e + n*sw + n*s + n*se +
                        ne*w + ne*e + ne*sw + ne*s + ne*se +
                        w*e + w*sw + w*s + w*se +
                        e*sw + e*s + e*se +
                        sw*se
                    )
                })
                .sum();
            println!("{} => {}", line, sum);
            Some(sum)
        })
        .sum();

    println!("Total: {}", total);

    Ok(())
}

