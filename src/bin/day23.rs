use std::{collections::BTreeMap, panic};

use itertools::Itertools;
use primes::PrimeSet;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let puzzle = include_str!("../../puzzles/day23.txt").trim();
    println!("Part 1: {}", part1(puzzle));
    println!("Part 2: {}", part2(puzzle)?);
    Ok(())
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

fn part2(input: &str) -> Result<usize, Box<dyn std::error::Error>> {
    let mut s = primes::Sieve::new();
    let instructions = parse(input);
    let b = instructions[0][2].parse::<i64>()? * instructions[4][2].parse::<i64>()?
        - instructions[5][2].parse::<i64>()?;
    let c = b - instructions[7][2].parse::<i64>()?;
    let step = instructions[30][2].parse::<i64>()?.abs();
    Ok((b..=c)
        .step_by(step as usize)
        .filter(|&x| !s.is_prime(x as u64))
        .count())
}

fn _part2_manual_decompile(input: &str) -> Result<i64, Box<dyn std::error::Error>> {
    let instructions = parse(input);

    let a = 1;
    let mut b: i64;
    let c: i64;
    let mut d;
    let mut e;
    let mut f;
    let mut h = 0;

    if a != 0 {
        b = instructions[0][2].parse::<i64>()? * instructions[4][2].parse::<i64>()?
            - instructions[5][2].parse::<i64>()?;
        c = b - instructions[7][2].parse::<i64>()?;
    } else {
        b = instructions[0][2].parse::<i64>()?;
        c = b;
    }

    while b <= c {
        f = 1;
        d = 2;
        while d < b {
            e = 2;
            while e < b {
                if d * e == b {
                    f = 0;
                }
                e += 1;
            }
            d += 1;
        }

        if f == 0 {
            h += 1;
        }

        b += 17;
    }

    Ok(h)
}

fn parse(input: &str) -> Vec<[&str; 3]> {
    input
        .lines()
        .map(|line| line.split_ascii_whitespace().collect_array().unwrap())
        .collect()
}
