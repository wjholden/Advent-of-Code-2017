use advent_of_code_2017::knot_hash;

fn main() {
    let puzzle = include_str!("../../puzzles/day14.txt").trim();
    println!("Part 1: {}", part1(puzzle));
    println!("Part 2: {}", part2(puzzle));
}

fn part1(input: &str) -> u32 {
    (0..=127).fold(0, |acc,i| {
        let s = format!("{input}-{i}");
        let h = knot_hash(s.as_bytes());
        acc + h.dense_hash().count_ones()
    })
}

fn part2(_input: &str) -> u32 {
    0
}

#[cfg(test)]
mod day14 {
    use super::*;

    const SAMPLE: &str = "flqrgnkx";

    #[test]
    fn test1() {
        assert_eq!(part1(SAMPLE), 8108)
    }
 
    #[test]
    fn test2() {
        assert_eq!(part2(SAMPLE), 1242)
    }   
}