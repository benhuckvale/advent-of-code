use std::fs::File;
use std::io::{self, BufRead};
use std::env;
use std::collections::{HashSet, HashMap};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    // 1->0 + 1 = 1 (2, 3, 4, 5)
    // 2->1 + 1 = 2 (3, 4)
    // 3->2 + 1 = 4 (4, 5)
    let (total_count, _): (i32, HashMap<i32, i32>) = reader
        .lines()
        .enumerate()
        .filter_map(|(usize_i, line_result)| {
            let i = usize_i as i32;
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
            let count = intersection_set.len() as i32;
            Some((i, ((i+1)..(i+1+count))))
            //Some(i, i..i+count)
            //   ^   ^
            //   |   |
            //  id   range of later cards to increment
        })
        .fold((0, HashMap::new()), |(acc_count, mut acc_map), (i, range)| {
            // Get number of times earlier cards incremented (won copy of) this one, and add
            // this card itself
            let this_count = *acc_map.entry(i).or_insert(0) + 1;
            println!("{}: {}", i, this_count);
            // Take each entry in the range, and add this_count to its entry in the map
            for j in range {
                *acc_map.entry(j).or_insert(0) += this_count;
            }
            (acc_count + this_count, acc_map)
        });

    println!("Sum: {}", total_count);

    Ok(())
}
