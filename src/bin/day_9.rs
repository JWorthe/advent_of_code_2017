extern crate advent_of_code_2017;
use advent_of_code_2017::*;

fn main() {
    let args = AdventArgs::init();

    let mut cancelled = false;
    let mut in_garbage = false;
    
    let mut depth = 0;
    let mut total_score = 0;
    let mut total_garbage = 0;
    
    for c in args.input[0].chars() {
        if cancelled {
            cancelled = false;
        } else if c == '!' {
            cancelled = true;
        } else if in_garbage {
            if c == '>' {
                in_garbage = false;
            } else {
                total_garbage += 1;
            }
        } else {
            if c == '<' {
                in_garbage = true;
            } else if c == '{' {
                depth += 1;
                total_score += depth;
            } else if c == '}' {
                depth -= 1;
            }
        }
    }

    if args.part == 1 {
        println!("Total score is {}", total_score);
    } else {
        println!("Total garbage is {}", total_garbage);
    }
}
