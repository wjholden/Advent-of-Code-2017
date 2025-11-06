use std::{collections::HashMap, panic};

use nalgebra::{DMatrix, dmatrix};
use num::Integer;

/// https://stackoverflow.com/questions/42519/how-do-you-rotate-a-two-dimensional-array
/// https://stackoverflow.com/questions/73186934/rust-nalgebra-how-to-modify-a-matrix-block
/// https://stackoverflow.com/questions/77669851/is-there-a-built-in-way-to-rotate-all-values-in-a-matrix-nalgebra
///
/// https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=2304b673c9b0e19be466f56abd93b372
/// https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=46052c0249aabda2589e07ae376597de
fn main() {
    let puzzle = include_str!("../../puzzles/day21.txt").trim();
    println!("Part 1: {}", solve(puzzle, 5));
    println!("Part 2: {}", solve(puzzle, 18));
}

fn solve(input: &str, iterations: usize) -> u64 {
    let enhancer = Enhancer::new(input);
    let mut mat = dmatrix![0, 1, 0; 0, 0, 1; 1, 1, 1];
    for _ in 1..=iterations {
        mat = enhancer.enhance(mat);
    }
    mat.sum()
}

struct Enhancer {
    rules: HashMap<DMatrix<u64>, DMatrix<u64>>,
}

impl Enhancer {
    pub fn new(input: &str) -> Self {
        let base_rules = Enhancer::parse(input);
        let mut rules = HashMap::new();
        for (input, output) in base_rules {
            for permutation in rotations_and_flips(input) {
                rules.insert(permutation, output.clone());
            }
        }
        Self { rules }
    }

    fn parse(input: &str) -> HashMap<DMatrix<u64>, DMatrix<u64>> {
        input
            .lines()
            .map(|line| {
                let mut split_line = line.split(" => ");
                if let (Some(l1), Some(l2)) = (split_line.next(), split_line.next()) {
                    let input = Enhancer::parse_pattern(l1);
                    let output = Enhancer::parse_pattern(l2);
                    (input, output)
                } else {
                    panic!()
                }
            })
            .collect()
    }

    fn parse_pattern(pattern: &str) -> DMatrix<u64> {
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

    pub fn enhance(&self, m1: DMatrix<u64>) -> DMatrix<u64> {
        let n1 = m1.nrows();
        let s1 = if n1.is_even() { 2 } else { 3 };
        let s2 = if n1.is_even() { 3 } else { 4 };
        let g = n1 / s1;
        let n2 = g * s2;
        let mut m2 = DMatrix::zeros(n2, n2);
        for r in 0..g {
            for c in 0..g {
                // https://users.rust-lang.org/t/nalgebra-best-practice-to-assign-a-matrix-to-a-part-submatrix-of-an-existing-matrix/132945/8
                let v1 = DMatrix::from(m1.view((s1 * r, s1 * c), (s1, s1)));
                let output = self.rules.get(&v1).unwrap();
                let mut v2 = m2.view_mut((s2 * r, s2 * c), (s2, s2));
                output.add_to(&DMatrix::zeros(s2, s2), &mut v2);
            }
        }
        m2
    }
}

fn rotations_and_flips(m: DMatrix<u64>) -> Vec<DMatrix<u64>> {
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

#[cfg(test)]
mod day21 {
    use super::*;

    const SAMPLE: &str = "../.# => ##./#../...
.#./..#/### => #..#/..../..../#..#";

    #[test]
    fn test1() {
        assert_eq!(solve(SAMPLE, 2), 12)
    }
}
