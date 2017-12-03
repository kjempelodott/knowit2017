#[macro_use]
extern crate lazy_static;
use std::collections::{HashMap,BinaryHeap};

lazy_static! {
    static ref BITS: Vec<u64> = (0..33).map(|s| 1 << s).collect();
}

fn is_wall(x: u64, y: u64) -> bool {
    let z = x*x*x + 12*x*y + 5*x*y*y;
    BITS.iter()
        .filter(|&b| b & z != 0)
        .count() % 2 != 0
}

fn solve(size: u64) -> usize {
    let mut graph: HashMap<(u64, u64), bool> = HashMap::new();
    let p = (1, 1);
    let mut queue: BinaryHeap<(u64, u64)> = BinaryHeap::new();
    
    queue.push(p);
    while let Some(cur) = queue.pop() {
        let (x, y) = cur;
        let mut adjacent = vec![];
        if x != 1 { adjacent.push((x-1, y)) }
        if y != 1 { adjacent.push((x, y-1)) }
        if x != size { adjacent.push((x+1, y)) }
        if y != size { adjacent.push((x, y+1)) }
        for &newp in adjacent.iter() {
            if !graph.contains_key(&newp) {
                let (x, y) = newp;
                if is_wall(x, y) {
                    graph.insert(newp, false);
                    continue;
                }
                graph.insert(newp, true);
                queue.push(newp);
            }
        }
    }
    (1..size+1)
        .fold(0, |s, y| s + (1..size+1)
              .filter(|&x| !graph.contains_key(&(x, y)) && !is_wall(x, y))
              .count())
}

fn main() {
    println!("Unreachable points: {}", solve(1000));
}
