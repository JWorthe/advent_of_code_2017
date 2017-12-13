extern crate advent_of_code_2017;
use advent_of_code_2017::*;

use std::collections::HashMap;

fn main() {
    let args = AdventArgs::init();

    let input: HashMap<u32, u32> = args.input.iter().map(|line| {
        let mut split_line = line.split(": ");
        (split_line.next().unwrap().parse().unwrap(), split_line.next().unwrap().parse().unwrap())
    }).collect();

    if args.part == 1 {
        let severity = calculate_severity(&input, 0, &args);
        println!("Severity: {}", severity);
    } else {
        let optimal_delay = (0u32..).find(|&delay| calculate_severity(&input, delay, &args) == 0).unwrap();
        println!("Wait {} picoseconds", optimal_delay);
    }
}

fn calculate_severity(input: &HashMap<u32, u32>, delay: u32, args: &AdventArgs) -> u32 {
    let mut severity = 0;
    let max_depth = input.keys().max().cloned().unwrap();
        
    for depth in 0..max_depth+1 {
        severity += match input.get(&depth) {
            Some(range) => {
                let position = (depth + delay) % (2*range-2);

                if position == 0 {
                    if args.part == 1 {
                        range * depth
                    } else {
                        range * depth + 1
                    }
                } else {
                    0
                }
            },
            None => 0
        };
    }
    severity
}
