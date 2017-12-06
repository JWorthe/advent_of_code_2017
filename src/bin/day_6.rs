extern crate advent_of_code_2017;
use advent_of_code_2017::*;

fn main() {
    let args = AdventArgs::init();

    let init_layout = parse_space_separated_ints(&args.input[0]).unwrap();
    
    let mut layouts = vec!(init_layout);
    let mut balances = 0;
    let mut cycle_found = false;
    let mut cycle_size = 0;

    while !cycle_found {
        balances += 1;
        let new_layout = find_next_layout(&layouts);
        
        if let Some(index) = layouts.iter().position(|x| *x == new_layout) {
            cycle_found = true;
            cycle_size = layouts.len() - index;
        };
        
        layouts.push(new_layout);
    }

    if args.part == 1 {
        println!("Did {} rebalances", balances);
    } else {
        println!("Cycle was {} long", cycle_size);
    }
}

fn find_next_layout(layouts: &Vec<Vec<i32>>) -> Vec<i32> {
    let previous_layout = layouts.last().unwrap();
    rebalance(&previous_layout)
}

fn rebalance(layout: &Vec<i32>) -> Vec<i32> {
    let biggest_container = layout.iter()
        .enumerate()
        .max_by(|&(ai, &asize), &(bi, &bsize)| {
            asize.cmp(&bsize).then(bi.cmp(&ai))
        })
        .map(|(i, _)| i)
        .unwrap();

    
    let mut new_layout = layout.clone();
    let mut to_redistribute = new_layout[biggest_container];
    new_layout[biggest_container] = 0;
    let mut target = (biggest_container + 1) % layout.len();

    while to_redistribute > 0 {
        new_layout[target] += 1;
        to_redistribute -= 1;
        target = (target + 1) % layout.len();
    }

    new_layout
}

