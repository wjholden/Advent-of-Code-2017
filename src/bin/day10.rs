fn main() {
    let puzzle = include_str!("../../puzzles/day10.txt").trim();
    let mut list: Vec<u8> = (0..=255).collect();
    let lengths: Vec<u8> = puzzle.split(",").flat_map(str::parse).collect();

    println!("Part 1: {}", knot_hash_part1(&mut list, &lengths));
    println!(
        "Part 2: {}",
        advent_of_code_2017::knot_hash(puzzle.as_bytes())
    );
}

fn knot_hash_part1(list: &mut [u8], lengths: &[u8]) -> usize {
    let mut position = 0;
    let n = list.len();

    // Clippy helped spot that we don't need to externally maintain skip_size.
    // We can get it for free using enumerate().
    for (skip_size, length) in lengths.iter().enumerate() {
        // Lengths larger than the size of the list are invalid.
        assert!(*length as usize <= n);

        // Reverse the order of that length of elements in the list, starting with the element at the current position.
        partial_reverse(list, position, *length as usize);

        // Move the current position forward by that length plus the skip size.
        position = (position + (*length as usize) + skip_size) % n;
    }
    list[0] as usize * list[1] as usize
}

fn partial_reverse<T>(list: &mut [T], start: usize, length: usize) {
    let n = list.len();
    for i in 0..length / 2 {
        let j = (start + i) % n;
        let k = (start + length - i - 1) % n;
        list.swap(j, k);
    }
}

#[cfg(test)]
mod day10 {
    use super::*;

    #[test]
    fn test1() {
        let mut list: Vec<u8> = (0..5).collect();
        let lengths = [3, 4, 1, 5];
        assert_eq!(knot_hash_part1(&mut list, &lengths), 12)
    }
}
