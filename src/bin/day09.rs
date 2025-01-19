use itertools::Itertools;

fn main() {
    let puzzle = include_str!("../../puzzles/day09.txt").trim();
    let (part1, part2) = solve(puzzle);
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

enum State {
    Input,
    Garbage,
    Cancel
}

fn solve(input: &str) -> (u32, u32) {
    let mut depth = 1;
    let mut score = 0;
    let mut state = match input.chars().next().unwrap() {
        '{' => State::Input,
        '<' => State::Garbage,
        _ => unreachable!()
    };
    let mut part2 = 0;
    for (a, b) in input.chars().tuple_windows() {
        match (a, b, &state) {
            (_, _, State::Cancel) => state = State::Garbage,
            (_, '!', State::Garbage) => state = State::Cancel,
            (_, '<', State::Input) => state = State::Garbage,
            (_, '>', State::Garbage) => state = State::Input,
            (_, _, State::Garbage) => part2 += 1,
            (_, '{', State::Input) => depth += 1,
            (_, '}', State::Input) => {
                score += depth;
                depth -= 1;
            },
            (_, _, State::Input) => (),
        }
    }
    (score, part2)
}

#[cfg(test)]
mod day09 {
    use super::*;

    #[test]
    fn test1() {
        let tests = [
            ("{}", 1),
            ("{{{}}}", 6),
            ("{{},{}}", 5),
            ("{{{},{},{{}}}}", 16),
            ("{<a>,<a>,<a>,<a>}", 1),
            ("{{<ab>},{<ab>},{<ab>},{<ab>}}", 9),
            ("{{<!!>},{<!!>},{<!!>},{<!!>}}", 9),
            ("{{<a!>},{<a!>},{<a!>},{<ab>}}", 3),
        ];
        for test in tests {
            assert_eq!(solve(test.0).0, test.1)
        }
    }
 
    #[test]
    fn test2() {
        let tests = [
            ("<>", 0),
            ("<random characters>", 17),
            ("<<<<>", 3),
            ("<{!>}>", 2),
            ("<!!>", 0),
            ("<!!!>>", 0),
            ("<{o\"i!a,<{i<a>", 10),
        ];
        for test in tests {
            assert_eq!(solve(test.0).1, test.1)
        }
    }   
}