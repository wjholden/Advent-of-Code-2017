use std::{collections::HashMap, fmt};

use nalgebra::Vector3;
use regex::Regex;

fn main() {
    let puzzle = include_str!("../../puzzles/day20.txt").trim();
    let mut particles = parse(puzzle);
    println!("Part 1: {}", part1(&particles));
    println!("Part 2: {}", part2(&mut particles));
}

#[derive(Debug, Hash, PartialEq, Eq)]
struct Particle {
    p: Vector3<i64>,
    v: Vector3<i64>,
    a: Vector3<i64>,
}

impl Particle {
    /// Each tick, all particles are updated simultaneously.
    /// A particle's properties are updated in the following order:
    /// - Increase the X velocity by the X acceleration.
    /// - Increase the Y velocity by the Y acceleration.
    /// - Increase the Z velocity by the Z acceleration.
    /// - Increase the X position by the X velocity.
    /// - Increase the Y position by the Y velocity.
    /// - Increase the Z position by the Z velocity.
    fn tick(&mut self) {
        self.v += self.a;
        self.p += self.v;
    }

    fn magnitude_of_acceleration(&self) -> i64 {
        self.a[0].abs() + self.a[1].abs() + self.a[2].abs()
    }
}

impl fmt::Display for Particle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "p=<{},{},{}>, v=<{},{},{}>, a=<{},{},{}>",
            self.p[0], self.p[1], self.p[2],
            self.v[0], self.v[1], self.v[2],
            self.a[0], self.a[1], self.a[2])
    }
}

fn parse(input: &str) -> Vec<Particle> {
    let re = Regex::new(r"p=<\s?(?P<px>-?\d+),\s?(?P<py>-?\d+),\s?(?P<pz>-?\d+)>, v=<\s?(?P<vx>-?\d+),\s?(?P<vy>-?\d+),\s?(?P<vz>-?\d+)>, a=<\s?(?P<ax>-?\d+),\s?(?P<ay>-?\d+),\s?(?P<az>-?\d+)>").unwrap();
    re.captures_iter(input).map(|cap| {
        Particle {
            p: Vector3::new(cap["px"].parse().unwrap(), cap["py"].parse().unwrap(), cap["pz"].parse().unwrap()),
            v: Vector3::new(cap["vx"].parse().unwrap(), cap["vy"].parse().unwrap(), cap["vz"].parse().unwrap()),
            a: Vector3::new(cap["ax"].parse().unwrap(), cap["ay"].parse().unwrap(), cap["az"].parse().unwrap()),
        }
    }).collect()
}

fn part1(particles: &[Particle]) -> usize {
    // We don't actually need to run the simulation for part 1. We just need
    // to find the minimum acceleration.
    particles.iter().enumerate().fold((0, i64::MAX), |(index, min_acceleration), (position, particle)| {
        let a = particle.magnitude_of_acceleration();
        if a < min_acceleration {
            (position, a)
        } else {
            (index, min_acceleration)
        }
    }).0
}

fn part2(particles: &mut [Particle]) -> usize {
    let mut survivors: Vec<&mut Particle> = Vec::from_iter(particles);
    for _ in 1..100 {
        let mut counts: HashMap<Vector3<i64>, u32> = HashMap::new();
        for particle in survivors.iter_mut() {
            particle.tick();
            *counts.entry(particle.p).or_default() += 1;
        }
        survivors.retain(|particle| counts.get(&particle.p) == Some(&1));
    }
    survivors.len()
}

#[cfg(test)]
mod day20 {
    use super::*;

    const SAMPLE: &str = "p=<-6,0,0>, v=< 3,0,0>, a=< 0,0,0>    
p=<-4,0,0>, v=< 2,0,0>, a=< 0,0,0>
p=<-2,0,0>, v=< 1,0,0>, a=< 0,0,0>
p=< 3,0,0>, v=<-1,0,0>, a=< 0,0,0>";

    #[test]
    fn test2() {
        let mut particles: Vec<Particle> = parse(SAMPLE);
        assert_eq!(part2(&mut particles), 1)
    }
}