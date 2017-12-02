extern crate advent_of_code_2017;
use advent_of_code_2017::*;

fn main() {
    let args = AdventArgs::init();
    let sum = args.input.iter().map(|line| {
        let splitline = parse_space_separated_ints(line).unwrap();
        
        if args.part == 1 {
            let max = splitline.iter().max().unwrap();
            let min = splitline.iter().min().unwrap();
            max-min
        } else {
            for i in 0..splitline.len() {
                for j in 0..splitline.len() {
                    if i != j && splitline[i] % splitline[j] == 0  {
                        return splitline[i] / splitline[j];
                    }
                }
            }
            panic!("Didn't find a dividing one! {:?}", splitline)
        }
    }).sum::<i32>();

    println!("Checksum is {}", sum);
}
