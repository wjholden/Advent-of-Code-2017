use std::collections::HashMap;

use num::Complex;

fn main() {
    let puzzle = include_str!("../../puzzles/day19.txt");
    let (part1, part2) = solve(puzzle);
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

fn solve(input: &str) -> (String, u64) {
    let grid: HashMap<Complex<i32>, char> = input.lines().enumerate().fold(HashMap::default(), |mut acc, (row, line)| {
        for (col, c) in line.char_indices() {
            if c != ' ' {
                acc.insert(Complex::new(row as i32, col as i32), c);
            }
        }
        acc
    });
    
    // find the start position
    let mut start = Complex::new(0,0);
    while grid.get(&start).is_none() {
        start += Complex::i();
    }
    
    let mut position = start;
    let mut direction = Complex::new(1, 0);
    let mut letters = String::default();
    let mut steps = 0;

    loop {
        match grid.get(&position) {
            None => break,
            Some('|') | Some('-') => (),
            Some('+') => {
                direction *= Complex::i();
                if !grid.contains_key(&(position + direction)) {
                    direction *= -1;
                }
            },
            Some(c) => {
                letters.push(*c);
            }
        }
        position += direction;
        steps += 1;
    }

    (letters, steps)
}

#[cfg(test)]
mod day19 {
    use super::*;

    const SAMPLE: &str = "     |          
     |  +--+    
     A  |  C    
 F---|----E|--+ 
     |  |  |  D 
     +B-+  +--+";

    #[test]
    fn test1() {
        assert_eq!(solve(SAMPLE).0, "ABCDEF")
    }

    #[test]
    fn test2() {
        assert_eq!(solve(SAMPLE).1, 38)
    }
}