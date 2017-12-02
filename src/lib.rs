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
            .map(|line| line.trim().to_string())
            .collect()
    }
}

pub fn parse_space_separated_ints(line: &String) -> Result<Vec<i32>, std::num::ParseIntError> {
    line.split_whitespace()
        .map(|x| x.parse::<i32>())
        .collect()
}
