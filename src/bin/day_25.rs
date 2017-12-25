extern crate advent_of_code_2017;
use advent_of_code_2017::*;

extern crate regex;
use regex::Regex;

use std::slice::Iter;
use std::collections::HashSet;

fn main() {
    let args = AdventArgs::init();
    let program = parse(&args.input);

    let mut position: i32 = 0;
    let mut state = program.state(program.start).expect("Started out of program bounds");
    let mut tape = HashSet::new();
    
    for _ in 0..program.iterations {
        let instruction = if tape.contains(&position) {
            &state.if1
        } else {
            &state.if0
        };
        if instruction.write {
            tape.insert(position);
        } else {
            tape.remove(&position);
        }
        position += instruction.offset;
        state = program.state(instruction.next).expect("Redirected to unknown state");
    }

    println!("{}", tape.len());
}

fn parse(input: &Vec<String>) -> Program {
    let state_re = Regex::new(r"state (\w)").unwrap();
    let iterations_re = Regex::new(r"(\d+) steps").unwrap();
    let write_re = Regex::new(r"Write the value (\d)").unwrap();
    let move_re = Regex::new(r"Move one slot to the (\w+)").unwrap();

    let mut lines = input.iter();
    let start = parse_char(&mut lines, &state_re);
    let iterations = parse_number(&mut lines, &iterations_re);
    
    let mut states = Vec::new();
    while let Some(heading) = lines.next() {
        states.push(State {
            id: state_re.captures(heading).unwrap()[1].chars().next().unwrap(),
            if0: parse_instruction(&mut lines, &write_re, &move_re, &state_re),
            if1: parse_instruction(&mut lines, &write_re, &move_re, &state_re)
        });
    }

    Program {
        start: start,
        iterations: iterations,
        states: states
    }
}

fn parse_char(lines: &mut Iter<String>, re: &Regex) -> char {
    re.captures(
        lines.next().unwrap()
    ).unwrap()[1].chars().next().unwrap()
}

fn parse_number(lines: &mut Iter<String>, re: &Regex) -> u32 {
    re.captures(
        lines.next().unwrap()
    ).unwrap()[1].parse().unwrap()
}
fn parse_direction(lines: &mut Iter<String>, re: &Regex) -> i32 {
    if re.captures(
        lines.next().unwrap()
    ).unwrap()[1] == *"left" {
        -1
    } else {
        1
    }
}

fn parse_bool(lines: &mut Iter<String>, re: &Regex) -> bool {
    re.captures(
        lines.next().unwrap()
    ).unwrap()[1] == *"1"
}

fn parse_instruction(lines: &mut Iter<String>, write_re: &Regex, offset_re: &Regex, next_re: &Regex) -> Instruction {
    lines.next();
    Instruction {
        write: parse_bool(lines, &write_re),
        offset: parse_direction(lines, &offset_re),
        next: parse_char(lines, &next_re)
    }
}

#[derive(Debug)]
struct Program {
    start: char,
    iterations: u32,
    states: Vec<State>
}

impl Program {
    fn state(&self, i: char) -> Option<&State> {
        self.states.iter().find(|s| s.id == i)
    }
}

#[derive(Debug)]
struct State {
    id: char,
    if0: Instruction,
    if1: Instruction
}

#[derive(Debug)]
struct Instruction {
    write: bool,
    offset: i32,
    next: char
}


