extern crate advent_of_code_2017;
use advent_of_code_2017::*;

fn main() {
    let args = AdventArgs::init();

    let directions: Vec<&str> = args.input[0].split(",").collect();

    let mut x = 0.0;
    let mut y = 0.0;
    
    let mut max_away = 0.0;
    
    for dir in directions {
        y += match dir {
            "ne" => 0.5,
            "n" => 1.0,
            "nw" => 0.5,
            "se" => -0.5,
            "s" => -1.0,
            "sw" => -0.5,
            _ => panic!("Unexpected direction {}", dir)
        };

        x += match dir {
            "ne" => -0.5,
            "n" => 0.0,
            "nw" => 0.5,
            "se" => -0.5,
            "s" => 0.0,
            "sw" => 0.5,
            _ => panic!("Unexpected direction {}", dir)
        };

        let current_distance = tile_distance(x, y);
        if current_distance > max_away {
            max_away = current_distance;
        }
    }

    if args.part == 1 {
        println!("Child process is {} away", tile_distance(x, y));
    } else {
        println!("At most, child process was {} away", max_away);
    }
    
}

fn tile_distance(x: f32, y: f32) -> f32 {
    let tiles_x = x.abs()*2.0;
    let tiles_y = if y.abs() < tiles_x {
        0.0
    } else {
        y.abs() - tiles_x
    };
    tiles_x + tiles_y
}
