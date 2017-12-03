extern crate advent_of_code_2017;
use advent_of_code_2017::*;

use std::collections::HashMap;

fn main() {
    use Direction::*;
    
    let args = AdventArgs::init();
    let input = args.one_number_input().unwrap();

    let mut memory: HashMap<Point, i32> = HashMap::new();
    let mut last_allocated = 1;
    
    let mut current = Point {
        x: 0,
        y: 0
    };
    memory.insert(current, last_allocated);

    let mut steps_per_direction = 1;
    let mut steps_to_next_turn = 1;
    let mut turns_to_spiral_increase = 2;

    let mut current_index = 1;
    let mut current_direction = Right;

    while (args.part == 1 && current_index != input) || (args.part == 2 && last_allocated < input) {
        current = current.shift(&current_direction);
        current_index += 1;

        steps_to_next_turn -= 1;
        if steps_to_next_turn == 0 {
            current_direction = current_direction.rotate_left();
            turns_to_spiral_increase -= 1;
            if turns_to_spiral_increase == 0 {
                steps_per_direction += 1;
                turns_to_spiral_increase = 2;
            }
            
            steps_to_next_turn = steps_per_direction;
        }

        if args.part == 2 {
            last_allocated = memory.get(&current.left()).cloned().unwrap_or(0) +
                memory.get(&current.right()).cloned().unwrap_or(0) +
                memory.get(&current.up()).cloned().unwrap_or(0) +
                memory.get(&current.down()).cloned().unwrap_or(0) +
                memory.get(&current.up().left()).cloned().unwrap_or(0) +
                memory.get(&current.up().right()).cloned().unwrap_or(0) +
                memory.get(&current.down().left()).cloned().unwrap_or(0) +
                memory.get(&current.down().right()).cloned().unwrap_or(0);
            
            memory.insert(current, last_allocated);
        }
    }

    println!("{:?}", current);
    println!("Distance: {}", current.x.abs() + current.y.abs());
    println!("Last Allocated Value: {}", last_allocated);
    
}
