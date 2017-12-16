extern crate advent_of_code_2017;
use advent_of_code_2017::*;

extern crate regex;
use regex::Regex;

fn main() {
    let args = AdventArgs::init();

    let spin_re = Regex::new(r"s(\d+)").unwrap();
    let exchange_re = Regex::new(r"x(\d+)/(\d+)").unwrap();
    let partner_re = Regex::new(r"p(\w)/(\w)").unwrap();
    let input: Vec<Instruction> = args.input[0]
        .split(',')
        .map(|instruction| {
            if let Some(caps) = spin_re.captures(instruction) {
                let spin_amount: usize = caps[1].parse().unwrap();
                Instruction::Spin(spin_amount)
            } else if let Some(caps) = exchange_re.captures(instruction) {
                let position_a: usize = caps[1].parse().unwrap();
                let position_b: usize = caps[2].parse().unwrap();
                Instruction::Exchange(position_a, position_b)
            } else if let Some(caps) = partner_re.captures(instruction) {
                let program_a = caps[1].chars().next().unwrap();
                let program_b = caps[2].chars().next().unwrap();
                Instruction::Partner(program_a, program_b)
            } else {
                panic!("Unhandled instruction: {}", instruction)
            }
        })
        .collect();
    
    let mut states = vec!("abcdefghijklmnop".chars().collect());
    if args.part == 1 {
        let answer = run(&input, &states.last().unwrap());
        println!("{}", answer.iter().collect::<String>());
    } else {
        let repetitions = 1_000_000_000;
        let mut cycle_found = false;
        let mut cycle_start = 0;
        while !cycle_found {
            let next = run(&input, &states.last().unwrap());
            if let Some(i) = states.iter().position(|&ref x| *x == next) {
                cycle_found = true;
                cycle_start = i;
            } else {
                states.push(next);
            }
        }
        println!("Cycle found after pushing {} states", states.len());
        println!("Cycle starts at {} states", cycle_start);
        
        let solution_index = (repetitions - cycle_start) % (states.len() - cycle_start);
        println!("{}", states[solution_index].iter().collect::<String>());
        
    }
}

enum Instruction {
    Spin(usize),
    Exchange(usize, usize),
    Partner(char, char)
}

fn run(instructions: &[Instruction], start: &Vec<char>) -> Vec<char> {
    let mut programs = start.clone();
    for instruction in instructions {
        match instruction {
            &Instruction::Exchange(a, b) => {
                programs.swap(a, b);
            },
            &Instruction::Spin(spin_amount) => {
                for _ in 0..spin_amount {
                    //this may be slow, but will suffice for right now
                    let end = programs.pop().unwrap();
                    programs.insert(0, end);
                }
            },
            &Instruction::Partner(program_a, program_b) => {
                let position_a: usize = programs.iter().position(|&x| x == program_a).unwrap();
                let position_b: usize = programs.iter().position(|&x| x == program_b).unwrap();
                programs.swap(position_a, position_b);
            }
        }
    }
    programs
}
