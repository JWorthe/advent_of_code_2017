extern crate advent_of_code_2017;
use advent_of_code_2017::*;

use std::collections::HashSet;

fn main() {
    let args = AdventArgs::init();
    let input_width = args.input[0].len();
    let input_height = args.input.len();

    let mut position = Point {
        x: (input_width / 2) as i32,
        y: (input_height / 2) as i32,
    };
    let mut direction = Direction::Up;

    let mut weakened: HashSet<Point> = HashSet::new();
    let mut infected: HashSet<Point> = HashSet::new();
    let mut flagged: HashSet<Point> = HashSet::new();
    
    for (y, line) in args.input.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                infected.insert(Point {
                    x: x as i32,
                    y: y as i32
                });
            }
        }
    }

    let mut infections_caused = 0;

    let bursts = if args.part == 1 {
        10_000
    } else {
        10_000_000
    };
    
    for _ in 0..bursts {
        if args.part == 1 {
            if infected.contains(&position) {
                direction = direction.rotate_right();
                infected.remove(&position);
            } else {
                direction = direction.rotate_left();
                infected.insert(position);
                infections_caused += 1;
            }
        }
        else {
            if weakened.contains(&position) {
                infected.insert(position);
                weakened.remove(&position);
                infections_caused += 1;
            } else if infected.contains(&position) {
                direction = direction.rotate_right();
                flagged.insert(position);
                infected.remove(&position);
            } else if flagged.contains(&position) {
                direction = direction.rotate_right().rotate_right();
                flagged.remove(&position);
            } else {
                direction = direction.rotate_left();
                weakened.insert(position);
            }
        }
        position = position.shift(&direction);
    }

    println!("Infections caused {}", infections_caused);
    
}
