use std::{collections::BTreeMap, panic};

use itertools::Itertools;

fn main() {
    let puzzle = include_str!("../../puzzles/day23.txt").trim();
    println!("Part 1: {}", part1(puzzle));
    // println!("Part 2: {}", part2(puzzle));
}

fn part1(input: &str) -> usize {
    let mut muls = 0;
    let mut registers: BTreeMap<&str, i64> = BTreeMap::new();
    let instructions = parse(input);
    let mut inst_ptr = 0i64;
    while (inst_ptr as usize) < instructions.len() {
        let y = instructions[inst_ptr as usize][2];
        let y = match y.parse::<i64>() {
            Ok(y) => y,
            Err(_) => *registers.entry(y).or_default(),
        };
        inst_ptr += match instructions[inst_ptr as usize][0..2] {
            ["set", x] => {
                registers.insert(x, y);
                1
            }
            ["sub", x] => {
                *registers.entry(x).or_default() -= y;
                1
            }
            ["mul", x] => {
                muls += 1;
                *registers.entry(x).or_default() *= y;
                1
            }
            ["jnz", "1"] => y,
            ["jnz", x] => match registers.get(x) {
                None | Some(0) => 1,
                Some(_) => y,
            },
            _ => panic!(),
        };
    }
    muls
}

fn parse(input: &str) -> Vec<[&str; 3]> {
    input
        .lines()
        .map(|line| line.split_ascii_whitespace().collect_array().unwrap())
        .collect()
}
