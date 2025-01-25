use core::fmt;

pub struct Hash {
    sparse_hash: Vec<u8>,
}

impl Hash {
    pub fn dense_hash(&self) -> u128 {
        let mut result = 0u128;
        for chunk in self.sparse_hash.chunks(16) {
            let h = chunk.iter().fold(0, |a,b| a ^ b);
            result = (result << 8) | (h as u128);
        }
        result
    }
}

impl fmt::Display for Hash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:016x}", self.dense_hash())
    }
}

/// In days 10 and 14 we need the Knot Hash algorithm.
pub fn knot_hash(lengths: &[u8]) -> Hash {
    let mut list: Vec<u8> = (0..=255).collect();
    let mut position = 0;
    let mut skip_size = 0;
    let n = list.len();
    for _ in 0..64 {
        for length in lengths.iter().chain([17u8, 31, 73, 47, 23].iter()) {
            // Lengths larger than the size of the list are invalid.
            assert!(*length as usize <= n);

            // Reverse the order of that length of elements in the list, starting with the element at the current position.
            partial_reverse(&mut list, position, *length as usize);

            // Move the current position forward by that length plus the skip size.
            position = (position + (*length as usize) + skip_size) % n;

            // Increase the skip size by one.
            skip_size += 1;
        }
    }

    Hash {
        sparse_hash: list
    }
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
mod knot_hash_lib {
    use super::*;
 
    #[test]
    fn tests() {
        let examples = [
            ("", 0xa2582a3a0e66e6e86e3812dcb672a272_u128),
            ("AoC 2017", 0x33efeb34ea91902bb2f59c9920caa6cd_u128),
            ("1,2,3", 0x3efbe78a8d82f29979031a4aa0b16a9d_u128),
            ("1,2,4", 0x63960835bcdc130f0b66d7ff4f6a5a8e_u128),
        ];
        for (input, expect) in examples {
            assert_eq!(knot_hash(input.as_bytes()).dense_hash(), expect)
        }
    }
}