use core::panic;
use std::{collections::{HashMap, VecDeque}, str::SplitAsciiWhitespace};

fn main() {
    let puzzle = include_str!("../../puzzles/day18.txt").trim();
    println!("Part 1: {}", part1(puzzle));
    println!("Part 2: {}", part2(puzzle));
}

fn part1(input: &str) -> i64 {
    let instructions: Vec<Vec<&str>> = input.lines().map(str::split_ascii_whitespace).map(SplitAsciiWhitespace::collect).collect();
    let mut ip: i64 = 0;
    let mut played = 0;
    let mut recovered = 0;
    let mut registers = HashMap::new();
    while recovered == 0 && 0 <= ip && ip < instructions.len() as i64 {
        match instructions[ip as usize][..] {
            ["snd", x] => {
                played = *registers.entry(x).or_default();
                ip += 1;
            },
            ["set", x, y] => {
                let y = match y.parse::<i64>() { Ok(y) => y, Err(_) => *registers.entry(y).or_default() };
                registers.insert(x, y);
                ip += 1;
            },
            ["add", x, y] => {
                // Major lesson learned here: unwrap_or and unwrap_or_else
                // differ in eager and lazy evaluation. unwrap_or is eager.
                // This means that if whatever you put in unwrap_or has a side-
                // effect, then you're going to see that side-effect whether
                // unwrap succeeds or fails.
                let y = y.parse::<i64>().unwrap_or_else(|_| *registers.entry(y).or_default());
                *registers.entry(x).or_default() += y;
                ip += 1;
            },
            ["mul", x, y] => {
                let y = y.parse::<i64>().unwrap_or_else(|_| *registers.entry(y).or_default());
                *registers.entry(x).or_default() *= y;
                ip += 1;
            },
            ["mod", x, y] => {
                let y = y.parse::<i64>().unwrap_or_else(|_| *registers.entry(y).or_default());
                *registers.entry(x).or_default() %= y;
                ip += 1;
            },
            ["rcv", x] => {
                let x = x.parse::<i64>().unwrap_or_else(|_| *registers.entry(x).or_default());
                if x != 0 {
                    //println!("recover {played}");
                    recovered = played;
                }
                ip += 1;
            },
            ["jgz", x, y] => {
                let y = y.parse::<i64>().unwrap_or_else(|_| *registers.entry(y).or_default());
                if *registers.get(x).unwrap_or(&0) > 0 {
                    ip += y;
                } else {
                    ip += 1;
                }
            },
            _ => panic!()
        };
    }
    recovered
}

fn part2(input: &str) -> u64 {
    let instructions: Vec<Vec<&str>> = input.lines().map(str::split_ascii_whitespace).map(SplitAsciiWhitespace::collect).collect();

    let mut p0 = Program::default();
    let mut p1 = Program::default();
    p1.registers.insert("p", 1);

    loop {
        if p0.queue.is_empty() && p1.queue.is_empty() && p1.sent > 0 {
            return p1.sent
        }

        p0.tick(&mut p1, &instructions);
        p1.tick(&mut p0, &instructions);
    }
}

#[derive(Default, Debug)]
struct Program<'a> {
    ip: usize,
    queue: VecDeque<i64>,
    sent: u64,
    registers: HashMap<&'a str, i64>,
}

impl<'a> Program<'a> {
    fn tick(&mut self, other: &mut Self, instructions: &[Vec<&'a str>]) {

        match instructions[self.ip][..] {
            ["snd", x] => {
                let x = x.parse::<i64>().unwrap_or_else(|_| *self.registers.entry(x).or_default());
                other.queue.push_back(x);
                self.sent += 1;
                self.ip += 1;
            },
            ["rcv", x] => {
                if let Some(y) = self.queue.pop_front() {
                    self.registers.insert(x, y);
                    self.ip += 1;
                } // otherwise block
            },
            ["set", x, y] => {
                let y = y.parse::<i64>().unwrap_or_else(|_| *self.registers.entry(y).or_default());
                self.registers.insert(x, y);
                self.ip += 1;
            },
            ["add", x, y] => {
                let y = y.parse::<i64>().unwrap_or_else(|_| *self.registers.entry(y).or_default());
                *self.registers.entry(x).or_default() += y;
                self.ip += 1;
            },
            ["mul", x, y] => {
                let y = y.parse::<i64>().unwrap_or_else(|_| *self.registers.entry(y).or_default());
                *self.registers.entry(x).or_default() *= y;
                self.ip += 1;
            },
            ["mod", x, y] => {
                let y = y.parse::<i64>().unwrap_or_else(|_| *self.registers.entry(y).or_default());
                *self.registers.entry(x).or_default() %= y;
                self.ip += 1;
            },
            ["jgz", x, y] => {
                // Found the bug! x can be an integer!
                let x = x.parse::<i64>().unwrap_or_else(|_| *self.registers.entry(x).or_default());
                if x > 0 {
                    let y = y.parse::<i64>().unwrap_or_else(|_| *self.registers.entry(y).or_default());
                    assert_ne!(y, 0);
                    if y < 0 {
                        self.ip -= y.unsigned_abs() as usize; // sometimes these unsigned array indices are annoying.
                    } else {
                        self.ip += y as usize;
                    }
                } else {
                    self.ip += 1;
                }
            }
            _ => panic!()
        }
    }
}

#[cfg(test)]
mod day18 {
    use super::*;

    const SAMPLE: &str = "set a 1
add a 2
mul a a
mod a 5
snd a
set a 0
rcv a
jgz a -1
set a 1
jgz a -2";

    const SAMPLE2: &str = "snd 1
snd 2
snd p
rcv a
rcv b
rcv c
rcv d";

    #[test]
    fn test1() {
        assert_eq!(part1(SAMPLE), 4)
    } 

    #[test]
    fn test2() {
        assert_eq!(part2(SAMPLE2), 3)
    } 
}