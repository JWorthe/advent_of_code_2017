extern crate advent_of_code_2017;
use advent_of_code_2017::*;

fn main() {
    let args = AdventArgs::init();
    let input = args.input[0].clone();

    let mut used = 0;
    let mut grid: Vec<Vec<bool>> = vec!(vec!(false; 128); 128);
    for i in 0..128 {
        let to_hash = format!("{}-{}", input, i);
        let hash = knot_hash(&to_hash);
        for (x1,c) in hash.chars().enumerate() {
            let parsed = u32::from_str_radix(&c.to_string(), 16).unwrap();
            used += parsed.count_ones();
            for (x2,b) in format!("{:04b}",parsed).chars().enumerate() {
                grid[i][4*x1+x2] = b == '1';
            }
        }
    }

    if args.part == 1 {
        println!("{} is used", used);
    } else {
        let mut group_count = 0;
        for start_y in 0..128 {
            for start_x in 0..128 {
                if grid[start_y][start_x] {
                    group_count += 1;
                    clear_group(&mut grid, Point{
                        x: start_x as i32,
                        y: start_y as i32
                    });
                }
            }
        }
        println!("{} groups", group_count);
        
    }
}

fn clear_group(grid: &mut Vec<Vec<bool>>, point: Point) {
    if point.x >= 0 && point.x < 128 && point.y >= 0 && point.y < 128 {
        if grid[point.y as usize][point.x as usize] {
            grid[point.y as usize][point.x as usize] = false;
            clear_group(grid, point.up());
            clear_group(grid, point.down());
            clear_group(grid, point.left());
            clear_group(grid, point.right());
        }
    }
}
