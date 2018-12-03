extern crate aoc2018;
extern crate regex;
extern crate lazy_static;

use std::io::prelude::*;
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use regex::Regex;
use lazy_static::lazy_static;
use aoc2018::get_input_file;

#[derive(Debug)]
pub struct Claim {
    id: i32,
    left: i32,
    top: i32,
    width: i32,
    height: i32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_claim() {
        let claim = parse_claim("#123 @ 3,2: 5x4");
        assert_eq!(claim.id, 123);
        assert_eq!(claim.left, 3);
        assert_eq!(claim.top, 2);
        assert_eq!(claim.width, 5);
        assert_eq!(claim.height, 4);
    }
}

pub fn parse_claim(s: &str) -> Claim {
    lazy_static! {
        static ref claim_re: Regex = Regex::new("#(\\d+) @ (\\d+),(\\d+): (\\d+)x(\\d+)").unwrap();
    }
    let captures = claim_re.captures(s).unwrap();
    Claim{
        id: captures.get(1).unwrap().as_str().parse::<i32>().unwrap(),
        left: captures.get(2).unwrap().as_str().parse::<i32>().unwrap(),
        top: captures.get(3).unwrap().as_str().parse::<i32>().unwrap(),
        width: captures.get(4).unwrap().as_str().parse::<i32>().unwrap(),
        height: captures.get(5).unwrap().as_str().parse::<i32>().unwrap(),
    }
}

// Short problem description:
// Given a set of rectangles, find the total area covered by the intersection of at least any pair of rectangles
// i.e. length of the set {x: (x ∈ A && x ∈ B) ∀ {(A, B) ∈ X: A != B} }
//
// Because the total area is sufficiently small we can use a simple HashMap to track the intersections.
fn main() {
    let claims: Vec<_> = get_input_file().lines().map(|x| x.unwrap()).map(|x| parse_claim(&x)).collect();
    let mut covered = HashMap::new();
    for claim in claims.iter() {
        for x in claim.left..claim.left + claim.width {
            for y in claim.top..claim.top + claim.height {
                let acc = covered.entry((x,y)).or_insert(0);
                *acc += 1
            }
        }
    }
    let total_cover = covered.iter().filter(|(_, &v)| v > 1).count();
    println!("Part 1: {}", total_cover);
    // Part two: which rectangle has no intersection?
    // We already know the intersecting squares, so we just need to find a claim where each element is only viisted once.
    'outer: for claim in claims.iter() {
        for x in claim.left..claim.left + claim.width {
            for y in claim.top..claim.top + claim.height {
                if let Entry::Occupied (o) = covered.entry((x, y)) {
                    if *o.get() != 1 {
                        continue 'outer;
                    }
                }
            }
        }
        println!("Part 2: Non-overlapping claim #{}", claim.id)
    }
}