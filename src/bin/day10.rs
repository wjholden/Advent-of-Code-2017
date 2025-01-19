fn main() {
    let puzzle = include_str!("../../puzzles/day10.txt").trim();
    let mut list: Vec<usize> = (0..256).collect();
    let lengths: Vec<usize> = puzzle.split(",").flat_map(str::parse).collect();

    println!("Part 1: {}", knot_hash(&mut list, &lengths, 1));
    println!("Part 2: {}", part2(puzzle));
}

fn knot_hash(list: &mut [usize], lengths: &[usize], rounds: usize) -> usize {
    let mut position = 0;
    let mut skip_size = 0;
    let n = list.len();
    for _ in 0..rounds {
        for length in lengths {
            // Lengths larger than the size of the list are invalid.
            assert!(*length <= n);

            // Reverse the order of that length of elements in the list, starting with the element at the current position.
            partial_reverse(list, position, *length);

            // Move the current position forward by that length plus the skip size.
            position = (position + length + skip_size) % n;

            // Increase the skip size by one.
            skip_size += 1;
        }
    }
    list[0] * list[1]
}

fn part2(input: &str) -> String {
    let extra = [17u8, 31, 73, 47, 23];
    let lengths: Vec<usize> = input.bytes().chain(extra).map(|b| b as usize).collect();
    let mut list: Vec<usize> = (0..256).collect();
    //let lengths: Vec<usize> = input.split(",").flat_map(str::parse).collect();
    //println!("{}: {lengths:?}", bytes.len());
    let _ = knot_hash(&mut list, &lengths, 64);
    let mut dense_hash = String::new();
    for chunk in list.chunks(16) {
        let h = chunk.iter().fold(0, |a,b| a ^ b);
        dense_hash = format!("{dense_hash}{h:02x}");
    }
    dense_hash
}

fn partial_reverse<T>(list: &mut [T], start: usize, length: usize) {
    let n = list.len();
    for i in 0..length/2 {
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
        let mut list: Vec<usize> = (0..5).collect();
        let lengths = [3,4,1,5];
        assert_eq!(knot_hash(&mut list, &lengths, 1), 12)
    }
 
    #[test]
    fn test2() {
        let examples = [
            ("", "a2582a3a0e66e6e86e3812dcb672a272"),
            ("AoC 2017", "33efeb34ea91902bb2f59c9920caa6cd"),
            ("1,2,3", "3efbe78a8d82f29979031a4aa0b16a9d"),
            ("1,2,4", "63960835bcdc130f0b66d7ff4f6a5a8e"),
        ];
        for (input, expect) in examples {
            assert_eq!(part2(input), expect)
        }
    }   
}