use nalgebra::DMatrix;

fn main() {
    let puzzle = include_str!("../../puzzles/day24.txt").trim();
    println!("Part 1: {}", part1(puzzle));
    println!("Part 2: {}", part2(puzzle));
}

fn part1(input: &str) -> usize {
    let g = parse(input);
    let mut strength = 0;
    for i in g.edges(0) {
        strength = strength.max(0 + i + g.strongest(&mut vec![(0, i)]));
    }
    strength
}

fn part2(input: &str) -> usize {
    let g = parse(input);
    let mut strength = 0;
    let mut length = 0;
    for i in g.edges(0) {
        let (s, l) = g.longest(&mut vec![(0, i)]);
        let s = 0 + i + s;
        if l > length {
            (strength, length) = (s, l);
        } else if l == length && s > strength {
            (strength, length) = (s, l);
        }
    }
    strength
}

struct Graph {
    m: DMatrix<bool>,
}

impl Graph {
    fn edges(&self, vertex: usize) -> Vec<usize> {
        self.m
            .row(vertex)
            .iter()
            .enumerate()
            .filter_map(|(i, val)| if *val { Some(i) } else { None })
            .collect()
    }

    fn strongest(&self, path: &mut Vec<(usize, usize)>) -> usize {
        let mut strength = 0;
        let u = path[path.len() - 1].1;
        for v in self.edges(u) {
            if path.contains(&(u, v)) || path.contains(&(v, u)) {
                continue;
            }
            path.push((u, v));
            //println!("{} {u}/{v}", " ".repeat(path.len() * 2));
            strength = strength.max(u + v + self.strongest(path));
            path.pop();
        }
        strength
    }

    fn longest(&self, path: &mut Vec<(usize, usize)>) -> (usize, usize) {
        let mut strength = 0;
        let mut length = path.len();
        let u = path[path.len() - 1].1;
        for v in self.edges(u) {
            if path.contains(&(u, v)) || path.contains(&(v, u)) {
                continue;
            }
            path.push((u, v));
            let (s, l) = self.longest(path);
            let s = u + v + s;
            if l > length {
                (strength, length) = (s, l);
            } else if l == length && s > strength {
                (strength, length) = (s, l);
            }
            path.pop();
        }
        (strength, length)
    }
}

fn parse(input: &str) -> Graph {
    let n = input
        .trim()
        .split(|c| c == '/' || c == '\n')
        .filter_map(|s| s.parse::<usize>().ok())
        .max()
        .unwrap();

    let mut m = DMatrix::from_element(n + 1, n + 1, false);

    for line in input.lines() {
        let words: Vec<_> = line.split("/").collect();
        let src = words[0].parse().unwrap();
        let dst = words[1].parse().unwrap();
        m[(src, dst)] = true;
        m[(dst, src)] = true;
    }
    Graph { m }
}

#[cfg(test)]
mod day24 {
    use nalgebra::dmatrix;

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
        assert_eq!(part1(SAMPLE), 31)
    }

    #[test]
    fn test2() {
        assert_eq!(part2(SAMPLE), 19)
    }

    #[test]
    fn test_parser() {
        let m = dmatrix![
            false,true,true,false,false,false,false,false,false,false,false;
            true,false,false,false,false,false,false,false,false,false,true;
            true,false,true,true,false,false,false,false,false,false,false;
            false,false,true,false,true,true,false,false,false,false,false;
            false,false,false,true,false,false,false,false,false,false,false;
            false,false,false,true,false,false,false,false,false,false,false;
            false,false,false,false,false,false,false,false,false,false,false;
            false,false,false,false,false,false,false,false,false,false,false;
            false,false,false,false,false,false,false,false,false,false,false;
            false,false,false,false,false,false,false,false,false,false,true;
            false,true,false,false,false,false,false,false,false,true,false;
        ];
        println!("{}", parse(SAMPLE).m);
        println!("{}", &m);
        assert_eq!(parse(SAMPLE).m, m)
    }

    #[test]
    fn test_edges() {
        let m = parse(SAMPLE);
        assert_eq!(m.edges(0), vec![1, 2]);
        assert_eq!(m.edges(10), vec![1, 9])
    }
}
