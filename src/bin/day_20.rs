extern crate advent_of_code_2017;
use advent_of_code_2017::*;

extern crate regex;
use regex::Regex;

#[macro_use]
extern crate lazy_static;

use std::str::FromStr;

fn main() {
    let args = AdventArgs::init();

    let mut particles: Vec<Particle> = args.input.iter()
        .map(|line| line.parse().unwrap())
        .collect();

    // I took eventually to be after a largish number. Seemed to work
    // out, but I'm sure there is a more mathematical way to work it
    // out.
    for _ in 0..1000 {
        particles = particles.iter().map(|p| p.step()).collect();
        if args.part == 2 {
            let before_collisions = particles.clone();
            particles.retain(|p| {
                before_collisions.iter().filter(|p2| p2.position == p.position).count() == 1
            });
        }
    }

    if args.part == 1 {
        let (closest, _) = particles.iter().enumerate().min_by_key(|&(_, p)| p.position.manhattan_distance()).unwrap();
        println!("Closest to 0: {}", closest);
    } else {
        let remaining = particles.iter().count();
        println!("Remaining: {}", remaining);
    }

}

#[derive(Debug, Clone)]
struct Particle {
    position: Point3d,
    velocity: Point3d,
    acceleration: Point3d
}


impl Particle {
    fn step(&self) -> Particle {
        let v = self.velocity + self.acceleration;
        Particle {
            position: self.position + v,
            velocity: v,
            acceleration: self.acceleration
        }
    }
}

impl FromStr for Particle {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static!{
            static ref RE: Regex = Regex::new(r"p=<(-?\d+),(-?\d+),(-?\d+)>, v=<(-?\d+),(-?\d+),(-?\d+)>, a=<(-?\d+),(-?\d+),(-?\d+)>").unwrap();
        };
        
        let caps = RE.captures(s).unwrap();
        Ok(Particle {
            position: Point3d {
                x: caps[1].parse().unwrap(),
                y: caps[2].parse().unwrap(),
                z: caps[3].parse().unwrap()
            },
            velocity: Point3d {
                x: caps[4].parse().unwrap(),
                y: caps[5].parse().unwrap(),
                z: caps[6].parse().unwrap()
            },
            acceleration: Point3d {
                x: caps[7].parse().unwrap(),
                y: caps[8].parse().unwrap(),
                z: caps[9].parse().unwrap()
            }
        })
    }
}

