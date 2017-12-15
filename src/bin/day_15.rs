extern crate advent_of_code_2017;
use advent_of_code_2017::*;

fn main() {
    let args = AdventArgs::init();

    let mut a: u64 = 591;
    let mut b: u64 = 393;

    let mut matches: u64 = 0;

    let comparisons = if args.part == 1 {
        40_000_000
    } else {
        5_000_000
    };
    
    for _ in 0..comparisons {
        a = (a * 16807) % 2147483647;
        b = (b * 48271) % 2147483647;

        while args.part != 1 && a % 4 != 0 {
            a = (a * 16807) % 2147483647;
        }
        while args.part != 1 && b % 8 != 0 {
            b = (b * 48271) % 2147483647;
        }
        
        if lower_16_match(a, b) {
            matches += 1;
        }
    }

    println!("There were {} matches", matches);
}

fn lower_16_match(a: u64, b: u64) -> bool {
    let mask = 65535; //2^16-1, ie 16 ones
    (a & mask) == (b & mask)
}
