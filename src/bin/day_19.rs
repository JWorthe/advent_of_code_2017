extern crate advent_of_code_2017;
use advent_of_code_2017::*;

fn main() {
    use Direction::*;
    
    let args = AdventArgs::init();

    let input: Vec<Vec<char>> = args.input.iter().map(|line| line.chars().collect()).collect();

    let mut position = Point {
        x: input[0].iter().position(|&c| c == '|').unwrap() as i32,
        y: 0
    };
    
    let mut direction = Down;
    let mut path_ended = false;
    let mut tokens = Vec::new();

    // moving onto the map counts as one, but because of how I'm
    // counting there's also an off the map step that I shouldn't be
    // counting at the end. They cancel out.
    let mut steps_moved = 0;

    while !path_ended {
        position = position.shift(&direction);
        steps_moved += 1;
        
        match char_at(&input, &position) {
            '|' | '-' => {
                //continue as is
            },
            ' ' => {
                path_ended = true;
            },
            '+' => {
                let left_option = char_at(&input, &position.shift(&direction.rotate_left()));
                let right_option = char_at(&input, &position.shift(&direction.rotate_right()));
                match (left_option, right_option) {
                    (' ', ' ') => {
                        path_ended = true;
                    },
                    (_, ' ') => {
                        direction = direction.rotate_left();
                    },
                    (' ', _) => {
                        direction = direction.rotate_right();
                    },
                    _ => {
                        panic!("Don't know where to go from {:?}", position);
                    }
                }
            },
            token => {
                tokens.push(token);
            }
        }
        
    }

    if args.part == 1 {
        println!("{}", tokens.iter().collect::<String>());
    } else {
        println!("{}", steps_moved);
    }
}

fn char_at(input: &Vec<Vec<char>>, position: &Point) -> char {
    if position.y < 0 ||
        position.x < 0 ||
        position.y as usize >= input.len() ||
        position.x as usize >= input[position.y as usize].len() {
            ' '
        } else {
            input[position.y as usize][position.x as usize]
        }
    
}
