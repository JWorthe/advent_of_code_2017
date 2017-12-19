extern crate structopt;
#[macro_use]
extern crate structopt_derive;
use structopt::StructOpt;

use std::path::PathBuf;
use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;
use std::process;

#[derive(StructOpt, Debug)]
#[structopt(name = "AOC2017", about = "An Advent of Code CLI arguments object.")]
struct AdventCli {
    #[structopt(help = "Which part of the puzzle you are solving")]
    part: u32,

    #[structopt(help = "Input file", parse(from_os_str))]
    input: PathBuf
}

pub struct AdventArgs {
    pub part: u32,
    pub input: Vec<String>
}

impl AdventArgs {
    pub fn init() -> AdventArgs {
        let opt = AdventCli::from_args();
        let input = match AdventArgs::read_file(&opt.input) {
            Ok(input) => input,
            Err(error) => {
                // Typically I would think of exiting the program like
                // this to be bad form, but in this case I'm matching the
                // interface of StructOpt: if the input parameters were
                // invalid, just quit now with a nice message.
                eprintln!("Error reading file: {}", error);
                process::exit(1);
            }
        };
        AdventArgs {
            part: opt.part,
            input: input
        }
    }

    fn read_file(file: &PathBuf) -> Result<Vec<String>, std::io::Error> {
        let file = File::open(file)?;
        let file_reader = BufReader::new(file);
        file_reader.lines()
            .collect::<Result<Vec<_>, _>>()
            .map(AdventArgs::preprocess_file_lines)
    }

    fn preprocess_file_lines(lines: Vec<String>) -> Vec<String> {
        lines.iter()
            .filter(|line| line.len() > 0)
            .map(|line| line.trim_right().to_string())
            .collect()
    }

    pub fn one_number_input(&self) -> Result<i32, std::num::ParseIntError> {
        self.input[0].parse()
    }
    pub fn number_per_line_input(&self) -> Result<Vec<i32>, std::num::ParseIntError> {
        self.input.iter().map(|line| line.parse()).collect()
    }
}

pub fn parse_space_separated_ints(line: &String) -> Result<Vec<i32>, std::num::ParseIntError> {
    line.split_whitespace()
        .map(|x| x.parse::<i32>())
        .collect()
}


#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
pub struct Point {
    pub x: i32,
    pub y: i32
}

impl Point {
    pub fn up(&self) -> Point {
        Point {
            y: self.y-1,
            ..*self
        }
    }

    pub fn down(&self) -> Point {
        Point {
            y: self.y+1,
            ..*self
        }
    }

    pub fn left(&self) -> Point {
        Point {
            x: self.x-1,
            ..*self
        }
    }

    pub fn right(&self) -> Point {
        Point {
            x: self.x+1,
            ..*self
        }
    }

    pub fn shift(&self, dir: &Direction) -> Point {
        use Direction::*;
        
        match *dir {
            Right => self.right(),
            Left => self.left(),
            Up => self.up(),
            Down => self.down()
        }
    }
}

#[derive(Debug)]
pub enum Direction {
    Left,
    Up,
    Down,
    Right
}

impl Direction {
    pub fn rotate_left(&self) -> Direction {
        use Direction::*;
        match *self {
            Right => Up,
            Up => Left,
            Left => Down,
            Down => Right
        }
    }

    pub fn rotate_right(&self) -> Direction {
        use Direction::*;
        match *self {
            Right => Down,
            Up => Right,
            Left => Up,
            Down => Left
        }
    }
}


pub fn knot_hash(input: &String) -> String {
    let suffix: [usize; 5] = [17, 31, 73, 47, 23];
    let lengths: Vec<usize> = input.as_bytes()
        .iter().map(|&x| x as usize)
        .chain(suffix.iter().cloned())
        .collect();

    let mut position = 0;
    let mut list: Vec<u32> = (0..256).collect();

    for i in 0..64 {
        let skip = lengths.len() * i;
        knot_hash_round(&mut list, &lengths, &mut position, skip);            
    }

    let mut current_char = 0;
    let mut result = String::new();
    for (i, l) in list.iter().enumerate() {
        current_char = current_char ^ l;
        if i % 16 == 15 {
            result.push_str(&format!("{:02x}", current_char));
            current_char = 0;
        }
    }
    result
}

fn knot_hash_round(list: &mut Vec<u32>, lengths: &Vec<usize>, position: &mut usize, skip: usize) {
    for (inner_skip, &length) in lengths.iter().enumerate() {
        knot_hash_reverse_segment(list, *position, length);
        *position = (*position + length + skip + inner_skip) % list.len();
    }
}

fn knot_hash_reverse_segment(list: &mut Vec<u32>, position: usize, length: usize) {
    let mut a = position;
    let mut b = position + length - 1;
    let len = list.len();
    while a < b {
        list.swap(a%len, b%len);
        
        a += 1;
        b -= 1;
    }
}
    
