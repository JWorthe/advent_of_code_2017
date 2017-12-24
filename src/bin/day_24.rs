extern crate advent_of_code_2017;
use advent_of_code_2017::*;

fn main() {
    let args = AdventArgs::init();
    let components: Vec<Component> = args.input.iter()
        .map(|line| {
            let mut split = line.split('/');
            Component {
                a: split.next().unwrap().parse().unwrap(),
                b: split.next().unwrap().parse().unwrap()
            }
        })
        .collect();

    if args.part == 1 {
        let strongest = build_strongest(0, components);
        println!("{}", strongest);
    } else {
        let (strongest, longest) = build_longest(0, components);
        println!("length: {}, strength: {}", longest, strongest);
    }
}

fn build_strongest(start: u32, components: Vec<Component>) -> u32 {
    components.iter().enumerate()
        .filter(|&(_, c)| c.a == start || c.b == start)
        .map(|(i, c)| {
            let end = if c.a == start { c.b } else { c.a };
            let mut subset = components.clone();
            subset.remove(i);
            c.strength() + build_strongest(end, subset)
        }).max().unwrap_or(0)
}

fn build_longest(start: u32, components: Vec<Component>) -> (u32, u32) {
    components.iter().enumerate()
        .filter(|&(_, c)| c.a == start || c.b == start)
        .map(|(i, c)| {
            let end = if c.a == start { c.b } else { c.a };
            let mut subset = components.clone();
            subset.remove(i);
            let (s, l) = build_longest(end, subset);
            (c.strength() + s, 1 + l)
        }).max_by(|&(s1, l1), &(s2, l2)| {
            l1.cmp(&l2).then(s1.cmp(&s2))
        }).unwrap_or((0, 0))
}

#[derive(Debug, Clone)]
struct Component {
    a: u32,
    b: u32
}

impl Component {
    fn strength(&self) -> u32 {
        self.a + self.b
    }
}
