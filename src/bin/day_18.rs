extern crate advent_of_code_2017;
use advent_of_code_2017::*;

use std::str::FromStr;
use std::collections::HashMap;
use std::sync::mpsc::*;

fn main() {
    let args = AdventArgs::init();

    let instructions: Vec<Instruction> = args.input.iter()
        .map(|line| line.parse().unwrap())
        .collect();

    let (sender0, receiver0) = channel();
    let (sender1, receiver1) = channel();

    let mut program0 = Program::new(0, instructions.clone(), sender0, receiver1, args.part == 1);

    if args.part == 1 {
        program0.run();
        let mut answer = 0;
        while let Ok(x) = receiver0.try_recv() {
            answer = x;
        }
        
        println!("Last sent value: {}", answer);
    } else {
        let mut program1 = Program::new(1, instructions.clone(), sender1, receiver0, args.part == 1);

        while !(program0.terminated && program1.terminated) && (program0.run() || program1.run()) {
        }
        

        println!("Program 0 sent {} messages", program0.sent_count);
        println!("Program 1 sent {} messages", program1.sent_count);
    }

    
}

struct Program {
    instructions: Vec<Instruction>,
    registers: HashMap<char, i64>,
    pc: i64,
    terminated: bool,
    sender: Sender<i64>,
    sent_count: u64,
    receiver: Receiver<i64>,
    part1: bool
}

impl Program {
    fn new(process_id: i64, instructions: Vec<Instruction>, sender: Sender<i64>, receiver: Receiver<i64>, part1: bool) -> Program {
        let mut reg = HashMap::new();
        if !part1 {
            reg.insert('p', process_id);
        }
        Program {
            instructions: instructions,
            registers: reg,
            pc: 0,
            terminated: false,
            sender: sender,
            sent_count: 0,
            receiver: receiver,
            part1: part1
        }
    }
    fn run(&mut self) -> bool {
        use Instruction::*;
        
        let mut blocked = false;
        let mut did_something = false;
        
        while !blocked && !self.terminated {
            if self.pc < 0 || self.pc as usize >= self.instructions.len() {
                self.terminated = true;
            }
            else {
                let ins = self.instructions[self.pc as usize].clone();
                
                match ins {
                    Snd(x) => {
                        self.sent_count += 1;
                        self.sender.send(self.get(x)).ok();
                    },
                    Set(x, y) => {
                        let y_val = self.get(y);
                        self.set(x, y_val);
                    },
                    Add(x, y) => {
                        let x_val = self.get(x);
                        let y_val = self.get(y);
                        self.set(x, x_val + y_val);
                    },
                    Mul(x, y) => {
                        let x_val = self.get(x);
                        let y_val = self.get(y);
                        self.set(x, x_val * y_val);
                    },
                    Mod(x, y) => {
                        let x_val = self.get(x);
                        let y_val = self.get(y);
                        self.set(x, x_val % y_val);
                    },
                    Rcv(x) => {
                        if self.part1 {
                            blocked = self.get(x) != 0;
                        } else {
                            match self.receiver.try_recv() {
                                Ok(y) => {
                                    self.set(x, y);
                                },
                                Err(_) => {
                                    blocked = true;
                                    return did_something;
                                }
                            }
                        }
                    },
                    Jgz(x, y) => {
                        if self.get(x) > 0 {
                            self.pc = self.pc + self.get(y) - 1;
                        }
                    },
                }
                self.pc += 1;
                did_something = true;
            }
        }
        true
    }

    fn get(&self, register: Data) -> i64 {
        use Data::*;
        match register {
            Register(c) => self.registers.get(&c).cloned().unwrap_or(0),
            Literal(i) => i
        }
    }

    fn set(&mut self, register: Data, value: i64) {
        use Data::*;
        match register {
            Register(c) => {
                self.registers.insert(c, value);
            },
            _ => {}
        }
    }
}

#[derive(Debug, Clone)]
enum Instruction {
    Snd(Data),
    Set(Data, Data),
    Add(Data, Data),
    Mul(Data, Data),
    Mod(Data, Data),
    Rcv(Data),
    Jgz(Data, Data)
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
            (Some("snd"), Some(Ok(x)), _) => Ok(Snd(x)),
            (Some("set"), Some(Ok(x)), Some(Ok(y))) => Ok(Set(x, y)),
            (Some("add"), Some(Ok(x)), Some(Ok(y))) => Ok(Add(x, y)),
            (Some("mul"), Some(Ok(x)), Some(Ok(y))) => Ok(Mul(x, y)),
            (Some("mod"), Some(Ok(x)), Some(Ok(y))) => Ok(Mod(x, y)),
            (Some("rcv"), Some(Ok(x)), _) => Ok(Rcv(x)),
            (Some("jgz"), Some(Ok(x)), Some(Ok(y))) => Ok(Jgz(x, y)),
            (_, _, _) => Err(format!("Unknown instruction {}", s))
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Data {
    Literal(i64),
    Register(char)
}

impl FromStr for Data {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Data::*;
        
        match (s.parse(), s.chars().next()) {
            (Ok(num), _) => Ok(Literal(num)),
            (Err(_), Some(c)) => Ok(Register(c)),
            (_, _) => Err(format!("Invalid data {}", s))
        }
    }
}
