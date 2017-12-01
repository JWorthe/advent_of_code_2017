extern crate advent_of_code_2017;
use advent_of_code_2017::*;

fn main() {
    let args = AdventArgs::init();

    let number_chars = args.input[0].chars().collect::<Vec<_>>();

    let mut sum = 0;
    
    for i in 0..number_chars.len() {
        let next = if args.part == 1 {
            (i + 1)
        } else {
            (i + number_chars.len() / 2)
        } % number_chars.len();
        if (number_chars[i] == number_chars[next]) {
            let parsed: i32 = number_chars[i].to_string().parse().unwrap();
            sum += parsed;
        }
    }
    
    println!("Sum is {}", sum);
}
