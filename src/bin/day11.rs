fn main() {
    let puzzle = include_str!("../../puzzles/day11.txt").trim();
    let (part1, part2) = solve(puzzle);
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

fn solve(input: &str) -> (i32, i32) {
    let mut distance = 0;
    let mut x = 0;
    let mut y = 0;
    for step in input.split(",") {
        match step {
            "n" => y += 2,
            "s" => y -= 2,
            "nw" => {
                x -= 1;
                y += 1;
            }
            "sw" => {
                x -= 1;
                y -= 1;
            }
            "ne" => {
                x += 1;
                y += 1;
            }
            "se" => {
                x += 1;
                y -= 1;
            }
            _ => unreachable!(),
        };
        distance = distance.max(hex_distance(x, y));
    }
    (hex_distance(x, y), distance)
}

fn hex_distance(x: i32, y: i32) -> i32 {
    x.abs() + (y.abs() - x.abs()) / 2
}

#[cfg(test)]
mod day11 {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(solve("ne,ne,ne").0, 3)
    }

    #[test]
    fn test2() {
        assert_eq!(solve("ne,ne,sw,sw").0, 0)
    }

    #[test]
    fn test3() {
        assert_eq!(solve("ne,ne,s,s").0, 2)
    }

    #[test]
    fn test4() {
        assert_eq!(solve("se,sw,se,sw,sw").0, 3)
    }
}
