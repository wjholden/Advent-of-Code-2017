use std::panic;

use nalgebra::{DMatrix, dmatrix};

/// https://stackoverflow.com/questions/42519/how-do-you-rotate-a-two-dimensional-array
/// https://stackoverflow.com/questions/73186934/rust-nalgebra-how-to-modify-a-matrix-block
/// https://stackoverflow.com/questions/77669851/is-there-a-built-in-way-to-rotate-all-values-in-a-matrix-nalgebra
fn main() {
    let puzzle = include_str!("../../puzzles/day21.txt").trim();
    //rotations_and_flips(2);
    //rotations_and_flips(3);
    println!("Part 1: {}", part1(puzzle));
    // println!("Part 2: {}", part2(puzzle));
}

fn part1(input: &str) -> usize {
    let rules = parse(input);
    for Rule {
        input,
        output: _output,
    } in rules
    {
        for pattern in rotations_and_flips(input) {
            println!("{pattern}");
        }
    }
    //todo!()
    0
}

fn rotations_and_flips(m: DMatrix<u8>) -> Vec<DMatrix<u8>> {
    let mut v = Vec::new();
    let mut mat = m.clone();

    let flip = match mat.nrows() {
        2 => dmatrix![0, 1; 1, 0],
        3 => dmatrix![0, 0, 1; 0, 1, 0; 1, 0, 0],
        _ => panic!(),
    };

    for i in 1..=8 {
        v.push(mat.clone());
        if i == 4 {
            mat = mat * &flip;
        } else {
            // Rotate by taking transpose and then swapping columns.
            mat = mat.transpose() * &flip;
        }
    }
    v
}

struct Rule {
    input: DMatrix<u8>,
    output: DMatrix<u8>,
}

fn parse(input: &str) -> Vec<Rule> {
    input
        .lines()
        .map(|line| {
            let mut split_line = line.split(" => ");
            let input = parse_pattern(split_line.next().unwrap());
            let output = parse_pattern(split_line.next().unwrap());
            Rule { input, output }
        })
        .collect()
}

fn parse_pattern(pattern: &str) -> DMatrix<u8> {
    println!("parse {pattern}");
    let n = match pattern.len() {
        5 => 2,
        11 => 3,
        19 => 4,
        _ => panic!(),
    };
    DMatrix::from_row_iterator(
        n,
        n,
        pattern.chars().filter_map(|c| match c {
            '.' => Some(0),
            '#' => Some(1),
            '/' => None,
            _ => panic!(),
        }),
    )
}

#[cfg(test)]
mod day21 {
    use super::*;

    const SAMPLE: &str = "../.# => ##./#../...
.#./..#/### => #..#/..../..../#..#";

    #[test]
    fn test1() {
        //assert_eq!(part1(SAMPLE), 0)
    }
}
