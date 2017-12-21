extern crate advent_of_code_2017;
use advent_of_code_2017::*;

extern crate regex;
use regex::Regex;

fn main() {
    let args = AdventArgs::init();

    let (t2, t3) = parse_transforms(&args.input);

    let mut picture = vec!(
        vec!(false, true, false),
        vec!(false, false, true),
        vec!(true, true, true)
    );

    let iterations = if args.part == 1 {
        5
    } else {
        18
    };
    for _ in 0..iterations {
        picture = expand(&picture, &t2, &t3);
    }

    let ones: usize = picture.iter().map(
        |row| row.iter().filter(|&&x| x).count()
    ).sum();
    println!("{} ones", ones);
}

fn print(picture: &Vec<Vec<bool>>) {
    for row in picture {
        for &c in row {
            print!("{}", if c {"#"} else {"."});
        }
        println!();
    }
    println!();
}

fn expand(picture: &Vec<Vec<bool>>, t2: &Vec<Transform2>, t3: &Vec<Transform3>) -> Vec<Vec<bool>> {
    let size = picture.len();
    let div = if size % 2 == 0 { 2 } else { 3 };
    let segments = size / div;
    let new_size = size + segments;

    let mut result = vec!(vec!(false; new_size); new_size);

    for i in 0..segments {
        let y = i*div;
        let v = i*(div+1);
        for j in 0..segments {
            let x = j*div;
            let u = j*(div+1);
            if div == 2 {
                let init = [
                    [picture[y][x], picture[y][x+1]],
                    [picture[y+1][x], picture[y+1][x+1]]
                ];
                let pattern = t2.iter().find(|p| p.matches(&init)).expect(&format!("No pattern matches {:?}", init));
                let to = pattern.to;

                for a in 0..div+1 {
                    for b in 0..div+1 {
                        result[v+a][u+b] = to[a][b];
                    }
                }
            } else {
                let init = [
                    [picture[y][x], picture[y][x+1], picture[y][x+2]],
                    [picture[y+1][x], picture[y+1][x+1], picture[y+1][x+2]],
                    [picture[y+2][x], picture[y+2][x+1], picture[y+2][x+2]]
                ];
                let pattern = t3.iter().find(|p| p.matches(&init)).expect(&format!("No pattern matches {:?}", init));
                let to = pattern.to;

                for a in 0..div+1 {
                    for b in 0..div+1 {
                        result[v+a][u+b] = to[a][b];
                    }
                }
            }
        }
    }
        
    result
}

fn parse_transforms(input: &Vec<String>) -> (Vec<Transform2>, Vec<Transform3>) {
    let t2_re = Regex::new(r"^(.)(.)/(.)(.) => (.)(.)(.)/(.)(.)(.)/(.)(.)(.)$").unwrap();
    let t3_re = Regex::new(r"^(.)(.)(.)/(.)(.)(.)/(.)(.)(.) => (.)(.)(.)(.)/(.)(.)(.)(.)/(.)(.)(.)(.)/(.)(.)(.)(.)$").unwrap();

    let mut t2 = Vec::new();
    let mut t3 = Vec::new();
    for line in input {
        if let Some(t2_caps) = t2_re.captures(line) {
            t2.push(Transform2 {
                from: [
                    [&t2_caps[1] == "#", &t2_caps[2] == "#"],
                    [&t2_caps[3] == "#", &t2_caps[4] == "#"]
                ],
                to: [
                    [&t2_caps[5] == "#", &t2_caps[6] == "#", &t2_caps[7] == "#"],
                    [&t2_caps[8] == "#", &t2_caps[9] == "#", &t2_caps[10] == "#"],
                    [&t2_caps[11] == "#", &t2_caps[12] == "#", &t2_caps[13] == "#"]
                ]
            });
        } else if let Some(t3_caps) = t3_re.captures(line) {
            t3.push(Transform3 {
                from: [
                    [&t3_caps[1] == "#", &t3_caps[2] == "#", &t3_caps[3] == "#"],
                    [&t3_caps[4] == "#", &t3_caps[5] == "#", &t3_caps[6] == "#"],
                    [&t3_caps[7] == "#", &t3_caps[8] == "#", &t3_caps[9] == "#"]
                ],
                to: [
                    [&t3_caps[10] == "#", &t3_caps[11] == "#", &t3_caps[12] == "#", &t3_caps[13] == "#"],
                    [&t3_caps[14] == "#", &t3_caps[15] == "#", &t3_caps[16] == "#", &t3_caps[17] == "#"],
                    [&t3_caps[18] == "#", &t3_caps[19] == "#", &t3_caps[20] == "#", &t3_caps[21] == "#"],
                    [&t3_caps[22] == "#", &t3_caps[23] == "#", &t3_caps[24] == "#", &t3_caps[25] == "#"]
                ]
            });
        }
    }
    
    (t2, t3)
}

#[derive(Debug)]
struct Transform2 {
    from: [[bool; 2]; 2],
    to: [[bool; 3]; 3]
}

impl Transform2 {    
    fn rotate(from: &[[bool;2];2]) -> [[bool;2];2] {
        [
            [from[1][0],from[0][0]],
            [from[1][1],from[0][1]]
        ]
    }

    fn flip(from: &[[bool;2];2]) -> [[bool;2];2] {
        [
            [from[0][1],from[0][0]],
            [from[1][1],from[1][0]]
        ]
    }
        
    fn matches(&self, other: &[[bool; 2]; 2]) -> bool {
        let mut any_match = false;
        let mut spinning_other = other.clone();
        for _ in 0..4 {
            any_match = any_match ||
                self.from == spinning_other ||
                self.from == Transform2::flip(&spinning_other);
            
            spinning_other = Transform2::rotate(&spinning_other);
        }
        any_match
    }
}

#[derive(Debug)]
struct Transform3 {
    from: [[bool; 3]; 3],
    to: [[bool; 4]; 4]
}

impl Transform3 {
    fn rotate(from: &[[bool;3];3]) -> [[bool;3];3] {
        [
            [from[2][0],from[1][0],from[0][0]],
            [from[2][1],from[1][1],from[0][1]],
            [from[2][2],from[1][2],from[0][2]]
        ]
    }

    fn flip(from: &[[bool;3];3]) -> [[bool;3];3] {
        [
            [from[0][2],from[0][1],from[0][0]],
            [from[1][2],from[1][1],from[1][0]],
            [from[2][2],from[2][1],from[2][0]]
        ]
    }
        
    fn matches(&self, other: &[[bool; 3]; 3]) -> bool {
        let mut any_match = false;
        let mut spinning_other = other.clone();
        for _ in 0..4 {
            any_match = any_match ||
                self.from == spinning_other ||
                self.from == Transform3::flip(&spinning_other);
            
            spinning_other = Transform3::rotate(&spinning_other);
        }
        any_match
    }
}
