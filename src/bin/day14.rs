use advent_of_code_2017::knot_hash;

fn main() {
    let puzzle = include_str!("../../puzzles/day14.txt").trim();
    let hash = knot_hash(list, lengths, rounds);
    println!("Part 1: {}", part1(puzzle));
    // println!("Part 2: {}", part2(puzzle));
}

fn part1(_input: &str) -> usize {
    0
}

#[cfg(test)]
mod day14 {
    use super::*;

    const SAMPLE: &str = "flqrgnkx";

    #[test]
    fn test1() {
        let hash = SAMPLE.parse::<u128>().unwrap();
        println!("{hash}");
        assert_eq!(part1(SAMPLE), 0)
    }
 
    #[test]
    fn test2() {
        //assert_eq!(part2(SAMPLE), 0)
    }   
}