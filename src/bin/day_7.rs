extern crate advent_of_code_2017;
use advent_of_code_2017::*;

extern crate regex;
use regex::Regex;

fn main() {
    let args = AdventArgs::init();

    let names_re = Regex::new(r"[a-z]+").unwrap();
    let weight_re = Regex::new(r"\d+").unwrap();

    let tree: Vec<(String, Vec<String>, i32)> = args.input.iter()
        .map(|line| {
            let mut matches = names_re.find_iter(line);
            let base = matches.next().unwrap().as_str().to_string();
            let leaves = matches.map(|m| m.as_str().to_string()).collect();
            let weight = weight_re.find(line).unwrap().as_str().parse().unwrap();
            (base, leaves, weight)
        }).collect();

    let mut possible_roots: Vec<String> = tree.iter().map(|&(ref id, _, _)| id.clone()).collect();
    for &(_, ref leaves, _) in &tree {
        for leaf in leaves  {
            let index = possible_roots.iter().position(|x| x == leaf).unwrap();
            possible_roots.remove(index);
        }
    }
    let root = &possible_roots[0];
    
    if args.part == 1 {
        println!("{:?}", root);
    } else {
        find_unweighted_plate(&root, &tree);
    }
}

fn find_unweighted_plate(root: &String, tree: &Vec<(String, Vec<String>, i32)>) -> i32 {
    let root_node = find_node(&root, &tree);
    let &(_, ref leaves, ref weight) = root_node;
    let leaf_weights: Vec<i32> = leaves.iter().map(|leaf| {
        find_unweighted_plate(&leaf, &tree)
    }).collect();

    if let Some(base_leaf_weight) = leaf_weights.first() {
        if let Some(different_leaf_weight) = leaf_weights.iter().find(|&w| w != base_leaf_weight) {
            println!("Unbalanced plate is off by {}", (different_leaf_weight-base_leaf_weight).abs());
            println!("Towers on plate: {:?} weigh {:?}", leaves, leaf_weights);
            // This still needs some manual work to get to the puzzle
            // output. Take the first unbalanced plate, figure out
            // which is the unbalanced tower visually, find its
            // individual weight in the file, and add/subtract as
            // necessary.
        }
    }

    leaf_weights.iter().sum::<i32>() + weight
}

fn find_node<'a>(name: &String, tree: &'a Vec<(String, Vec<String>, i32)>) -> &'a (String, Vec<String>, i32) {
    tree.iter().find(|&&(ref id, _, _)| id == name).unwrap()
}
