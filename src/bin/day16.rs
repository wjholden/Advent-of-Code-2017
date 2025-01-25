use std::collections::VecDeque;

fn main() {
    let puzzle = include_str!("../../puzzles/day16.txt").trim();
    println!("Part 1: {}", part1(b'a'..=b'p', puzzle));
    println!("Part 2: {}", part2(b'a'..=b'p', puzzle));
}

fn dance(programs: &mut VecDeque<u8>, dance_move: &str) {
    if dance_move.starts_with("s") {
        let x = dance_move[1..].parse().unwrap();
        programs.rotate_right(x);
    } else if dance_move.starts_with("x") {
        let slash = dance_move.find("/").unwrap();
        let a = dance_move[1..slash].parse().unwrap();
        let b = dance_move[slash+1..].parse().unwrap();
        programs.swap(a, b);
    } else if dance_move.starts_with("p") {
        let a = dance_move.as_bytes()[1];
        let b = dance_move.as_bytes()[3];
        let a = programs.iter().position(|&x| x == a).unwrap();
        let b = programs.iter().position(|&x| x == b).unwrap();
        programs.swap(a, b);
    }
}

fn part1<T: std::iter::Iterator<Item = u8>>(domain: T, input: &str) -> String {
    let mut programs = VecDeque::from_iter(domain);
    input.split(",").for_each(|dance_move| {
        dance(&mut programs, dance_move)
    });
    String::from_utf8(Vec::from(programs)).unwrap()
}

/// This must (again) be an act of puzzle-building wizardry. It doesn't feel
/// safe to assume that the period occurs on the boundary of dance moves, but
/// for my input this does happen after 60 iterations, so we can skip past
/// 999999900 iterations, do the remaining 40, and there's our answer.
fn part2<T: std::iter::Iterator<Item = u8>>(domain: T, input: &str) -> String {
    let mut period_length = 0;
    let mut programs = VecDeque::from_iter(domain);
    while period_length == 0 || !programs.iter().is_sorted() {
        input.split(",").for_each(|dance_move| {
            dance(&mut programs, dance_move)
        });
        period_length += 1;
    }
    for _ in 0..(1000000000 % period_length) {
        input.split(",").for_each(|dance_move| {
            dance(&mut programs, dance_move)
        });
    }
    String::from_utf8(Vec::from(programs)).unwrap()
}

#[cfg(test)]
mod day16 {
    use super::*;

    const SAMPLE: &str = "s1,x3/4,pe/b";

    #[test]
    fn test1() {
        assert_eq!(part1(b'a'..=b'e', SAMPLE), "baedc")
    }
}