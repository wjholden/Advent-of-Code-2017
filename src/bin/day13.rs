use std::collections::HashMap;
use regex::Regex;

fn main() {
    let puzzle = include_str!("../../puzzles/day13.txt").trim();
    let layers = parse(puzzle);
    println!("Part 1: {}", part1(&layers, 0).unwrap());
    println!("Part 2: {:?}", part2(&layers).unwrap());
}

fn parse(input: &str) -> HashMap<u64, u64> {
    let re = Regex::new(r"(?<depth>\d+): (?<range>\d+)").unwrap();
    re.captures_iter(input).map(|cap| {
        let depth = cap["depth"].parse::<u64>().unwrap();
        let range = cap["range"].parse::<u64>().unwrap();
        (depth, range)
    }).collect()
}

fn part1(layers: &HashMap<u64, u64>, delay: u64) -> Option<u64> {
    let mut severity = 0;
    let mut caught = false;

    let n = *layers.keys().max().unwrap();
    for depth in 0..=n {
        if let Some(range) = layers.get(&depth) {
            let time = depth + delay;
            let y = (time) % (2 * (range - 1));
            if y == 0 {
                // caught!
                severity += depth * range;
                caught = true;
            }
        }
    }
    match caught {
        true => Some(severity),
        false => None
    }
}

#[allow(dead_code)]
fn part2_naive(layers: &HashMap<u64, u64>) -> Option<u64> {
    for i in 1.. {
        if part1(layers, i).is_none() {
            return Some(i)
        }
    }
    None
}

/// The naive solution is too slow. We can avoid lots of computational work by
/// stopping the inner loop as soon as we eliminate the candidate solution.
fn part2(layers: &HashMap<u64, u64>) -> Option<u64> {
    'outer: for delay in 1.. {
        for (depth, range) in layers {
            let time = depth + delay;
            let y = (time) % (2 * (range - 1));
            if y == 0 {
                continue 'outer
            }
        }
        return Some(delay)
    }
    None
}

#[cfg(test)]
mod day13 {
    use super::*;

    const SAMPLE: &str = "0: 3
1: 2
4: 4
6: 4";

    #[test]
    fn test1() {
        assert_eq!(part1(&parse(SAMPLE), 0), Some(24))
    }
 
    #[test]
    fn test2() {
        assert_eq!(part2_naive(&parse(SAMPLE)), Some(10))
    }   

    #[test]
    fn test3() {
        assert_eq!(part2(&parse(SAMPLE)), Some(10))
    }
}