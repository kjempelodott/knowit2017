use std::collections::HashSet;

fn make_ngram(n: usize, word: &String) -> Vec<u8> {
    let mut ngram = String::new();
    (0..(word.len() - n + 1)).for_each(|i| ngram.push_str(&word[i..i+n]));
    let mut sorted = ngram.into_bytes();
    sorted.sort();
    sorted
}

fn solve(ngram: &str) -> (usize, String) {
    let ngram = String::from(ngram);
    let letters: HashSet<u8> = ngram.bytes().collect();
    let mut sorted = ngram.clone().into_bytes();
    sorted.sort();

    let wordlist = include_str!("../../res/day1.txt");
    let lines = &mut wordlist.lines();
    
    (2..8)
        .filter(|n| ngram.len() % n == 0)
        .map(|n| (n, n - 1 + ngram.len()/n))
        .filter_map(|(n, wlen)| {
            lines
                .map(|line| String::from(line).to_lowercase())
                .filter(|lc| lc.bytes().all(|c| letters.contains(&c)))
                .filter_map(|lc| {
                    if make_ngram(n, &lc) == sorted {
                        return Some((n, lc));
                    }
                    None
                }).next()
        }).next().unwrap()
}

fn main() {
    let (n, word) = solve("aeteesasrsssstaesersrrsse");
    println!("{}-{}", n, word);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn snowflake() {
        assert_eq!(solve("snnoowwffllaakke"), (2, "snowflake".into()));
    }
        #[test]
    fn mistletoe() {
        assert_eq!(solve("misiststltleletetotoe"), (3, "mistletoe".into()));
    }
}
