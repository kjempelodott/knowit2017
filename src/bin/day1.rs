use std::collections::HashSet;

static WORDLIST: &'static str = include_str!("../../res/day1.txt");

fn make_ngram(n: usize, word: &str) -> Vec<u8> {
    let mut ngram = String::new();
    (0..(word.len() - n + 1)).for_each(|i| ngram.push_str(&word[i..i+n]));
    let mut lowercase = ngram.to_lowercase().into_bytes();
    lowercase.sort();
    lowercase
}

fn main() {
    let ngram = String::from("aeteesasrsssstaesersrrsse").to_lowercase();
    let letters: HashSet<u8> = ngram.bytes().collect();

    let mut sorted = ngram.clone().into_bytes();
    sorted.sort();
    
    let (n, word) = WORDLIST.lines()
        .map(|l| String::from(l).to_lowercase())
        .filter(|l| l.bytes().collect::<HashSet<u8>>() == letters)
        .filter_map(|l| {
            if let Some(n) = (2..l.len()-1).find(|&n| make_ngram(n, &l) == sorted) {
                return Some((n, l))
            }
            None
        }).next().unwrap();
    println!("{}-{}", n, word);
}
