extern crate advent_of_code_2017;
use advent_of_code_2017::*;

use std::str::FromStr;

fn main() {
    let args = AdventArgs::init();

    if args.part == 1 {
        let instructions: Vec<Instruction> = args.input.iter()
            .map(|line| line.parse().unwrap())
            .collect();

        let mut program = Program::new(instructions.clone(), args.part == 1);
        let mul_called = program.run();
        println!("Mult called {} times", mul_called);
    } else {
        println!("Result is {}", run_as_rust());
    }

}

fn to_register(c: char) -> usize {
    (c as u32 - 'a' as u32) as usize
}

struct Program {
    instructions: Vec<Instruction>,
    registers: [i64; 8],
    pc: i64
}

impl Program {
    fn new(instructions: Vec<Instruction>, part1: bool) -> Program {
        let mut reg = [0; 8];
        if part1 == false {
            reg[0] = 1;
        }
        Program {
            instructions: instructions,
            registers: reg,
            pc: 0
        }
    }
    fn run(&mut self) -> u32 {
        use Instruction::*;
        
        let mut mul_called = 0;
        
        while self.pc >= 0 && (self.pc as usize) < self.instructions.len() {
            let ins = self.instructions[self.pc as usize].clone();
            
            match ins {
                Set(x, y) => {
                    let y_val = self.get(y);
                    self.set(x, y_val);
                },
                Sub(x, y) => {
                    let x_val = self.get(x);
                    let y_val = self.get(y);
                    self.set(x, x_val - y_val);
                },
                Mul(x, y) => {
                    let x_val = self.get(x);
                    let y_val = self.get(y);
                    self.set(x, x_val * y_val);
                    mul_called += 1;
                },
                Jnz(x, y) => {
                    if self.get(x) != 0 {
                        self.pc = self.pc + self.get(y) - 1;
                    }
                },
            }
            self.pc += 1;
        }

        mul_called
    }

    fn get(&self, register: Data) -> i64 {
        use Data::*;
        match register {
            Register(c) => self.registers[c],
            Literal(i) => i
        }
    }

    fn set(&mut self, register: Data, value: i64) {
        use Data::*;
        match register {
            Register(c) => {
                self.registers[c] = value;
            },
            _ => {}
        }
    }
}

#[derive(Debug, Clone)]
enum Instruction {
    Set(Data, Data),
    Sub(Data, Data),
    Mul(Data, Data),
    Jnz(Data, Data)
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Instruction::*;
        
        let mut str_iter = s.split_whitespace();
        let ins = str_iter.next();
        let x = str_iter.next().map(|x| x.parse::<Data>());
        let y = str_iter.next().map(|x| x.parse::<Data>());

        match (ins, x, y) {
            (Some("set"), Some(Ok(x)), Some(Ok(y))) => Ok(Set(x, y)),
            (Some("sub"), Some(Ok(x)), Some(Ok(y))) => Ok(Sub(x, y)),
            (Some("mul"), Some(Ok(x)), Some(Ok(y))) => Ok(Mul(x, y)),
            (Some("jnz"), Some(Ok(x)), Some(Ok(y))) => Ok(Jnz(x, y)),
            (_, _, _) => Err(format!("Unknown instruction {}", s))
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Data {
    Literal(i64),
    Register(usize)
}

impl FromStr for Data {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Data::*;
        
        match (s.parse(), s.chars().next()) {
            (Ok(num), _) => Ok(Literal(num)),
            (Err(_), Some(c)) => Ok(Register(to_register(c))),
            (_, _) => Err(format!("Invalid data {}", s))
        }
    }
}


fn run_as_rust() -> i64 {
    let mut h: i64 = 0;   
    let mut b: i64 = 99 * 100 + 100000;
    let c: i64 = b + 17000;

    while b <= c {
        let f = (2..b).any(|d| {
            b % d == 0
        });

        if f {
            h += 1;
        }

        b += 17;
    }
    
    h
}
