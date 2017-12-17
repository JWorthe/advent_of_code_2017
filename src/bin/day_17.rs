extern crate advent_of_code_2017;
use advent_of_code_2017::*;

fn main() {
    let args = AdventArgs::init();
    let step_size: usize = args.input[0].parse().unwrap();

    let inserts = if args.part == 1 {
        2017
    } else {
        50_000_000
    };

    let mut buffer = Vec::with_capacity(inserts + 1);
    buffer.push(0);
    let mut position = 0;

    for i in 0..inserts as u32 {
        let to_insert = i+1;
        // the +1 is because they want it to insert AFTER the element
        // that adding position ends on
        position = ((position + step_size) % buffer.len()) + 1;
        if args.part == 2 && position != 1 {
            // for big vectors, push is MUCH more efficient than
            // insert (O(C) vs O(n)). In part 2, we want the element
            // after 0, which will always be index 1. It only needs to
            // be inserted into the right place if it's actually going
            // to be in position 1.
            //
            // If I wasn't meshing the solution with part 1, there
            // probably wouldn't even be a vector, just tracking the
            // length and index 1.
            buffer.push(to_insert);
        } else {
            buffer.insert(position, to_insert);
        }
    }

    let answer_position = if args.part == 1 {
        (position+1)%buffer.len()
    } else {
        1
    };
    
    let answer = buffer[answer_position];
    println!("{}", answer);
}
