use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::collections::HashSet;

fn main() {
    let mut args = env::args();
    let input_file_path = args.nth(1).expect("requires one argument: input file path");
    let input_file = File::open(input_file_path).expect("unable to open input file");
    let reader = BufReader::new(input_file);

    let values: Vec<_> = reader.lines().map(|line| line.unwrap().parse::<i64>().unwrap()).collect();
    println!("Part 1 result: {}", values.iter().sum::<i64>());

    let mut freqs = HashSet::new();
    let mut total: i64 = 0;
    for i in 0..1000 {
        for value in values.iter() {
            total += value;
            if !freqs.insert(total) {
                println!("Part 2 result: {} (during iteration {})", total, i);
                return
            }
        }
    }
    println!("Part 2: no frequency convergence after 100 iterations");
}
