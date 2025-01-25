use std::collections::{BTreeSet, VecDeque};

use advent_of_code_2017::knot_hash;

fn main() {
    let puzzle = include_str!("../../puzzles/day14.txt").trim();
    let (part1, part2) = solve(puzzle);
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

fn solve(input: &str) -> (u32, u32) {
    let hashes: Vec<u128> = (0..=127).map(|i| {
        knot_hash(format!("{input}-{i}").as_bytes()).dense_hash()
    }).collect();

    let part1 = hashes.iter().fold(0, |acc,h| {
        acc + h.count_ones()
    });

    let mut positions: BTreeSet<(i32, i32)> = hashes.iter().enumerate().flat_map(|(row, h)| {
        (0i32..=127).filter_map(move |col| {
            match (*h >> col) & 1 {
                0 => None,
                1 => Some((row as i32, col)),
                _ => unreachable!()
            }
        })
    }).collect();

    let mut part2 = 0;
    while let Some(current) = positions.pop_first() {
        part2 += 1;
        let mut bfs_frontier = VecDeque::new();
        bfs_frontier.push_back(current);
        while let Some(current) = bfs_frontier.pop_front() {
            let (r,c) = current;
            for (r,c) in [(r+1,c),(r-1,c),(r,c+1),(r,c-1)] {
                if positions.remove(&(r,c)) {
                    bfs_frontier.push_back((r,c));
                }
            }
        }
    }

    (part1, part2)
}

#[cfg(test)]
mod day14 {
    use super::*;

    const SAMPLE: &str = "flqrgnkx";

    #[test]
    fn test1() {
        assert_eq!(solve(SAMPLE).0, 8108)
    }
 
    #[test]
    fn test2() {
        assert_eq!(solve(SAMPLE).1, 1242)
    }   
}