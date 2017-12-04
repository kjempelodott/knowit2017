use std::collections::HashMap;

fn solve(wordlist: &str) -> usize {
    wordlist.lines()
        .filter(|l| l.to_string() != l.chars().rev().collect::<String>())
        .filter(|l| {
            let mut set = HashMap::new();
            l.chars().for_each(|c| *set.entry(c).or_insert(0) += 1);
            set.values().filter(|&n| n % 2 == 1).count() < 2
        })
        .count()
}

fn main() {
    let wordlist = include_str!("../../res/day4.txt");
    println!("{}", solve(wordlist));
}
