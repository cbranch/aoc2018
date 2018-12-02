extern crate aoc2018;

use std::io::prelude::*;
use std::collections::HashMap;
use aoc2018::get_input_file;

// First bool indicates exactly two of a letter, second indicates exactly three
fn box_id_checker(id: &str) -> (bool, bool) {
    let mut accs = HashMap::new();
    for c in id.chars() {
        let acc = accs.entry(c).or_insert(0);
        *acc += 1
    }
    let has_two = accs.iter().any(|(_, &v)| v == 2);
    let has_three = accs.iter().any(|(_, &v)| v == 3);
    (has_two, has_three)
}

fn box_id_single_char_mismatch(first: &str, second: &str) -> (bool, String) {
    let chars: String = first.chars().zip(second.chars()).filter(|(a, b)| a == b).map(|(a, _)| a).collect();
    (chars.len() == first.len() - 1, chars)
}

fn find_prototype_box_ids(box_ids: &Vec<String>) -> String {
    for (i, first) in box_ids.iter().enumerate() {
        for second in &box_ids[i + 1..] {
            let (similar, chars) = box_id_single_char_mismatch(first, second);
            if similar {
                return chars
            }
        }
    }
    "".to_string()
}

fn main() {
    let box_ids: Vec<_> = get_input_file().lines().map(|x| x.unwrap()).collect();

    let (two, three): (Vec<_>, Vec<_>) = box_ids.iter().map(|x| box_id_checker(&x)).unzip();
    let two_count = two.iter().filter(|x| **x).count();
    let three_count = three.iter().filter(|x| **x).count();
    println!("Checksum for box IDs: {}", two_count * three_count);
    println!("Common letters: {}", find_prototype_box_ids(&box_ids));
}