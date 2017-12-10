extern crate advent_of_code_2017;
use advent_of_code_2017::*;

fn main() {
    let args = AdventArgs::init();

    let lengths: Vec<usize> = if args.part == 1 {
        args.input[0].split(",").map(|x| x.parse().unwrap()).collect()
    } else {
        let suffix: [usize; 5] = [17, 31, 73, 47, 23];
        args.input[0].as_bytes()
            .iter().map(|&x| x as usize)
            .chain(suffix.iter().cloned())
            .collect()
    };
    
    let mut position = 0;
    let mut list: Vec<u32> = (0..256).collect();

    if args.part == 1 {
        hash_round(&mut list, &lengths, &mut position, 0);
    } else {
        for i in 0..64 {
            let skip = lengths.len() * i;
            hash_round(&mut list, &lengths, &mut position, skip);            
        }
    }


    if args.part == 1 {
        let answer = list[0]*list[1];
        println!("{}", answer);
    } else {
        let mut current_char = 0;
        for (i, l) in list.iter().enumerate() {
            current_char = current_char ^ l;
            if i % 16 == 15 {
                print!("{:02x}", current_char);
                current_char = 0;
            }
        }
        println!("");
    }
}

fn hash_round(list: &mut Vec<u32>, lengths: &Vec<usize>, position: &mut usize, skip: usize) {
    for (inner_skip, &length) in lengths.iter().enumerate() {
        reverse(list, *position, length);
        *position = (*position + length + skip + inner_skip) % list.len();
    }
}

fn reverse(list: &mut Vec<u32>, position: usize, length: usize) {
    let mut a = position;
    let mut b = position + length - 1;
    let len = list.len();
    while a < b {
        list.swap(a%len, b%len);
        
        a += 1;
        b -= 1;
    }
}
