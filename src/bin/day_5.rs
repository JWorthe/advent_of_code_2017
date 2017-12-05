extern crate advent_of_code_2017;
use advent_of_code_2017::*;

fn main() {
    let args = AdventArgs::init();

    let mut jumps: Vec<i32> = args.input.iter().map(|line| line.parse().unwrap()).collect();
    let mut steps_taken = 0;
    let mut current_position: i32 = 0;

    while current_position >= 0 && (current_position as usize) < jumps.len() {
        let previous_position = current_position;
        current_position += jumps[current_position as usize];

        if args.part == 1 || jumps[previous_position as usize] < 3 {
            jumps[previous_position as usize] += 1;
        } else {
            jumps[previous_position as usize] -= 1;
        }
        
        steps_taken += 1;
    }

    println!("Escaped in {} jumps", steps_taken);
}
