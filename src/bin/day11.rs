use std::collections::BTreeSet;
const MAX: usize = 1000;

fn main() {
    let mut mirptall: BTreeSet<usize> = BTreeSet::new();
    let is_primtall = |n| !(2..1+(n as f64).sqrt().ceil() as usize).any(|m| n % m == 0);
    for n in 10..1+MAX {
        if mirptall.contains(&n) { continue }
        if is_primtall(n) {
            let as_str = format!("{}", n);
            let rev: String = as_str.chars().rev().collect();
            if as_str == rev { continue }
            let mirp: usize = rev.parse().unwrap();
            if is_primtall(mirp) {
                mirptall.insert(n);
                mirptall.insert(mirp);
            }
        }
    }
    println!("{}", mirptall.len());
}
