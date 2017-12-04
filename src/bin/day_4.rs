extern crate advent_of_code_2017;
use advent_of_code_2017::*;

fn main() {
    let args = AdventArgs::init();

    let valid_count = args.input.iter()
        .map(|line| {
            let words = line.split_whitespace().map(|x| x.to_string()).collect::<Vec<String>>();
            if args.part == 1 {
                let mut deduped_words = words.clone();
                deduped_words.sort();
                deduped_words.dedup();
                words.len() == deduped_words.len()
            } else {
                !words.iter().enumerate().any(|(i, word1)| {
                    words.iter().enumerate().any(|(j, word2)| {
                        i != j && is_anagram(word1, word2)
                    })
                })
            }
        })
        .filter(|&valid| valid)
        .count();

    println!("Valid count: {}", valid_count);
}

fn is_anagram(word1: &str, word2: &str) -> bool {
    let mut chars1 = word1.chars().collect::<Vec<_>>();
    chars1.sort();
    let mut chars2 = word2.chars().collect::<Vec<_>>();
    chars2.sort();

    chars1.len() == chars2.len() &&
        chars1.iter().zip(chars2.iter()).all(|(c1, c2)| c1 == c2)
}
