use std::{collections::VecDeque, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    let puzzle = include_str!("../../puzzles/day17.txt").trim().parse()?;
    println!("Part 1: {}", part1(puzzle));
    println!("Part 2: {}", part2(puzzle));
    Ok(())
}

fn part1(steps: usize) -> usize {
    let mut buffer = VecDeque::from([0]);
    let mut position = 0;
    for i in 1..=2017 {
        position = (position + steps) % buffer.len() + 1;
        buffer.insert(position, i);
    }
    buffer[position+1]
}

/// Found a very, very helpful post on Reddit that helped me to read the "good
/// news" closely. You don't actually care about what's in the buffer.
/// You only need to keep track of when we would write to position 1.
fn part2(steps: usize) -> usize {
    let mut p1 = 0;
    let mut position = 0;
    for i in 1..=50_000_000 {
        position = (position + steps) % i + 1;
        if position == 1 {
            p1 = i;
        }
    }
    p1
}

#[cfg(test)]
mod day17 {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(part1(3), 638)
    }  
}