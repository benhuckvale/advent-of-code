use std::fs::File;
use std::io::{self, BufRead};
use std::env;

fn numbers_from_string(string: &str) -> Vec<u64> {
    string.split_whitespace()
          .filter_map(|s| s.parse().ok())
          .collect()
}

/*
 * Return exclusive count of all integers between two floats.
 *
 * Exclusive means to not count the integers that coincide with the values given.
 */
fn count_integers_between(start: f64, end: f64) -> usize {
    let excluded_start_integer = (start - f64::EPSILON).floor() as i64;
    let excluded_end_integer = (end + f64::EPSILON).ceil() as i64;
    return (excluded_start_integer..excluded_end_integer).count() as usize - 1;
}

/*
 * Returns exclusive count of all integers between solutions of inequality equation.
 *
 * Equation of interest is:
 * s < Tt - t^2
 * where s is the "distance" that needs to be exceeded, t is the time spent increasing speed, and T is
 * total "time" both doing that and moving.
 * Using the conventional expression of a quadratic equation for t:
 * t^2 - Tt + s < 0
 * Solving for t:
 * t = ( T +- sqrt(T^2 - 4s) ) / 2
 * We want to count all the integer solutions between these two solutions.  i.e. find the count of
 * all integer values of t that fulfil the inequality.
 */
fn count_ways(time: u64, distance: u64) -> u64 {
    let ftime = time as f64;
    let discriminant = ftime*ftime - 4.0 * (distance as f64);
    let sqrt = discriminant.sqrt();
    let solution0 = (ftime - sqrt)/2.0;
    let solution1 = (ftime + sqrt)/2.0;
    let count = count_integers_between(solution0, solution1) as u64;
    println!("{} {} {} {} {} {}", time, distance, discriminant, solution0, solution1, count);
    count
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let mut lines = reader.lines().take(2).map(|line| line.unwrap_or_default());

    if let (Some(first_line), Some(second_line)) = (lines.next(), lines.next()) {

        let times: Vec<u64> = numbers_from_string(&first_line);
        let distances: Vec<u64> = numbers_from_string(&second_line);

        let sum: u64 = times.into_iter().zip(distances.into_iter()).map(|(time, distance)| {
            count_ways(time, distance)
        }).fold((|| 1)(), |acc, result| acc * result);

        println!("Sum: {}", sum);

    }

    Ok(())
}
