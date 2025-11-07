use std::{collections::HashMap, panic};

fn main() {
    let puzzle = include_str!("../../puzzles/day22.txt").trim();
    println!("Part 1: {}", solve(puzzle, 10000, Part::One));
    println!("Part 2: {}", solve(puzzle, 10000000, Part::Two));
}

enum Part {
    One,
    Two,
}

fn solve(input: &str, bursts: usize, part: Part) -> usize {
    let (rows, cols) = dims(input);
    let mut x = cols / 2;
    let mut y = rows / 2;
    let mut dx = 0;
    let mut dy = 1;
    let mut map = parse(input);
    let mut infections = 0;
    for _ in 1..=bursts {
        match part {
            Part::One => {
                // If the current node is infected, it turns to its right.
                // Otherwise, it turns to its left. (Turning is done in-place;
                // the current node does not change.)
                [dx, dy] = match map.entry((x, y)).or_insert(State::Clean) {
                    // https://wjholden.com/advent-of-code-2017-day22-part1.pdf
                    State::Clean => [-dy, dx],
                    State::Infected => [dy, -dx],
                    State::Weakened | State::Flagged => unreachable!(),
                };
                // If the current node is clean, it becomes infected. =
                // Otherwise, it becomes cleaned.
                // (This is done after the node is considered for the purposes of changing direction.)
                map.entry((x, y)).and_modify(|v| {
                    *v = match v {
                        State::Clean => {
                            infections += 1;
                            State::Infected
                        }
                        State::Infected => State::Clean,
                        // You could delete the clean ones and this whole thing would still work,
                        // but from my tests it doesn't make much difference. We aren't visiting
                        // very many positions (just 93764 in part 2) and HashMap scales really well.
                        State::Weakened | State::Flagged => unreachable!(),
                    }
                });
            }
            Part::Two => {
                // Decide which way to turn based on the current node
                [dx, dy] = match map.entry((x, y)).or_insert(State::Clean) {
                    State::Clean => [-dy, dx],
                    State::Infected => [dy, -dx],
                    State::Weakened => [dx, dy],
                    State::Flagged => [-dx, -dy],
                };
                // Modify the state of the current node, as described above.
                map.entry((x, y)).and_modify(|v| {
                    *v = match v {
                        State::Clean => State::Weakened,
                        State::Infected => State::Flagged,
                        State::Weakened => {
                            infections += 1;
                            State::Infected
                        }
                        State::Flagged => State::Clean,
                    }
                });
            }
        }
        // The virus carrier moves forward one node in the direction it is facing.
        [x, y] = [x + dx, y + dy];
    }
    infections
}

#[derive(PartialEq, Eq, Debug)]
enum State {
    Clean,
    Infected,
    Weakened,
    Flagged,
}

fn dims(input: &str) -> (i64, i64) {
    let rows = input.lines().count();
    let cols = input.lines().next().unwrap().len();
    (rows as i64, cols as i64)
}

fn parse(input: &str) -> HashMap<(i64, i64), State> {
    let (rows, _) = dims(input);
    let mut map = HashMap::default();
    for (row, line) in input.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            map.insert(
                // Always an off-by-one error.
                // Also, changing (rows,cols) => (x,y) is not fun.
                (col as i64, rows - (row as i64) - 1),
                match c {
                    '.' => State::Clean,
                    '#' => State::Infected,
                    _ => unreachable!(),
                },
            );
        }
    }
    map
}

#[allow(dead_code)]
fn show(map: &HashMap<(i64, i64), State>) {
    let cmax = *map.keys().map(|(x, _)| x).max().unwrap();
    let cmin = *map.keys().map(|(x, _)| x).min().unwrap();
    let rmin = *map.keys().map(|(_, y)| y).min().unwrap();
    let rmax = *map.keys().map(|(_, y)| y).max().unwrap();
    for r in (rmin..=rmax).rev() {
        for c in cmin..=cmax {
            match map.get(&(c, r)) {
                Some(State::Infected) => print!("#"),
                None | Some(State::Clean) => print!("."),
                _ => unreachable!(),
            }
            print!(" ");
        }
        println!()
    }
}

#[cfg(test)]
mod day22 {
    use super::*;

    const SAMPLE: &str = "..#
#..
...";

    #[test]
    fn test1() {
        assert_eq!(solve(SAMPLE, 10000, Part::One), 5587)
    }

    #[test]
    fn test2() {
        assert_eq!(solve(SAMPLE, 100, Part::Two), 26)
    }

    #[test]
    fn test3() {
        assert_eq!(solve(SAMPLE, 10000000, Part::Two), 2511944)
    }
}
