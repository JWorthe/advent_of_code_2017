extern crate advent_of_code_2017;
use advent_of_code_2017::*;

extern crate regex;
use regex::Regex;

#[macro_use]
extern crate lazy_static;

use std::str::FromStr;
use std::collections::HashMap;
use std::cmp;

fn main() {
    let args = AdventArgs::init();
    
    let instructions: Vec<Instruction> = args.input.iter()
        .map(|line| line.parse().unwrap())
        .collect();
    
    let mut memory: HashMap<String, i32> = HashMap::new();
    let mut max_mem_ever = 0;
    
    for instruction in instructions {
        if instruction.is_true(&memory) {
            instruction.execute(&mut memory);
            max_mem_ever = cmp::max(max_mem_ever, memory.values().max().cloned().unwrap_or(0));
        }
    }

    let max_mem = memory.values().max().cloned().unwrap_or(0);

    if args.part == 1 {
        println!("Highest value in memory is {}", max_mem);
    } else {
        println!("Highest value in memory ever is {}", max_mem_ever);
    }
}


struct Instruction {
    target_register: String,
    action: String,
    value: i32,
    condition_register: String,
    condition: String,
    condition_value: i32
}

#[derive(Debug)]
struct InstructionParseError {
    reason: String
}

impl FromStr for Instruction {
    type Err = InstructionParseError;
    fn from_str(s: &str) -> Result<Self, InstructionParseError> {
        lazy_static!{
            static ref INSTRUCTION_RE: Regex = Regex::new(r"^(\w+) (inc|dec) (-?\d+) if (\w+) (<|>|<=|>=|==|!=) (-?\d+)").unwrap();
        }
        match INSTRUCTION_RE.captures(s) {
            Some(caps) => Ok(Instruction{
                target_register: caps[1].to_string(),
                action: caps[2].to_string(),
                value: caps[3].parse().unwrap(),
                condition_register: caps[4].to_string(),
                condition: caps[5].to_string(),
                condition_value: caps[6].parse().unwrap()
            }),
            None => Err(InstructionParseError {
                reason: format!("{} did not match regex", s)
            })
        }        
    }
}

impl Instruction {
    fn is_true(&self, memory: &HashMap<String, i32>) -> bool {
        let mem = memory.get(&self.condition_register).cloned().unwrap_or(0);
        match self.condition.as_ref() {
            "<" => mem < self.condition_value,
            ">" => mem > self.condition_value,
            "<=" => mem <= self.condition_value,
            ">=" => mem >= self.condition_value,
            "==" => mem == self.condition_value,
            "!=" => mem != self.condition_value,
            _ => panic!("Unknown condition: {}", self.condition)
        }
    }

    fn execute(&self, memory: &mut HashMap<String, i32>) {
        let modifier = if self.action == "inc" {
            self.value
        } else {
            -self.value
        };
        *memory.entry(self.target_register.clone()).or_insert(0) += modifier;
    }
}
