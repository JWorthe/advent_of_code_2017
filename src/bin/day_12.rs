extern crate advent_of_code_2017;
use advent_of_code_2017::*;

use std::cmp;

fn main() {
    let args = AdventArgs::init();

    let mut groups: Vec<Vec<i32>> = vec!(vec!(0)); //0 in the first group

    for line in args.input {
        let mut words_iter = line.split_whitespace();
        let current: i32 = words_iter.next().unwrap().parse().unwrap();
        if find_group(&groups, current).is_none() {
            groups.push(vec!(current));
        }
        words_iter.next().unwrap(); //<->
        for other_str in words_iter {
            let other: i32 = other_str.trim_right_matches(",").parse().unwrap();

            match (find_group(&groups, current), find_group(&groups, other)) {
                (Some(current_group), Some(other_group)) if current_group != other_group => {
                    merge_groups(&mut groups, current_group, other_group);
                },
                (Some(_), Some(_)) => {
                },
                (Some(current_group), None) => {
                    groups[current_group].push(other);
                },
                (None, _) => panic!("Current group not found!")
            };
        }
    }

    if args.part == 1 {
        println!("First group has {} members", groups[0].len());
    } else {
        println!("Total of {} groups", groups.len());
    }
}

fn find_group(groups: &Vec<Vec<i32>>, x: i32) -> Option<usize> {
    groups.iter().position(|group| group.contains(&x))
}

fn merge_groups(groups: &mut Vec<Vec<i32>>, a: usize, b: usize) {
    let src = cmp::max(a, b);
    let dest = cmp::min(a, b);

    let mut from = groups.swap_remove(src);
    groups[dest].append(&mut from)
}
