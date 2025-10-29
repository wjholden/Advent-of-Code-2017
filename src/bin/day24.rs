fn main() {
    let puzzle = include_str!("../../puzzles/day24.txt").trim();
    println!("Part 1: {}", part1(puzzle));
    // println!("Part 2: {}", part2(puzzle));
}

fn part1(_input: &str) -> usize {
    todo!()
}

#[derive(Debug, PartialEq, Eq)]
struct Component {
    src: u8,
    dst: u8,
}

fn parse(input: &str) -> Vec<Component> {
    let mut result = Vec::new();
    for line in input.lines() {
        let words: Vec<_> = line.split("/").collect();
        let src = words[0].parse().unwrap();
        let dst = words[1].parse().unwrap();
        result.push(Component { src, dst });
    }
    result
}

#[cfg(test)]
mod day24 {
    use super::*;

    const SAMPLE: &str = "0/2
2/2
2/3
3/4
3/5
0/1
10/1
9/10";

    #[test]
    fn test1() {
        assert_eq!(part1(SAMPLE), 0)
    }

    #[test]
    fn test_parser() {
        assert_eq!(
            parse(SAMPLE),
            vec![
                Component { src: 0, dst: 2 },
                Component { src: 2, dst: 2 },
                Component { src: 2, dst: 3 },
                Component { src: 3, dst: 4 },
                Component { src: 3, dst: 5 },
                Component { src: 0, dst: 1 },
                Component { src: 10, dst: 1 },
                Component { src: 9, dst: 10 },
            ]
        )
    }
}
