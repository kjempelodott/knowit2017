use std::collections::{HashMap,HashSet,BinaryHeap};

static mut SIZE: u64 = 10;

#[derive(PartialEq,Eq)]
enum Reachable {
    Wall,
    Unknown,
    No,
    Yes
}

fn find_any_path(p: (u64, u64), points: &mut HashMap<(u64, u64), Reachable>) {

    if *points.get(&p).unwrap() != Reachable::Unknown {
        return
    }

    let mut visited: HashSet<(u64, u64)> = HashSet::new();
    let mut queue: BinaryHeap<(u64, u64)> = BinaryHeap::new();
    queue.push(p);

    let mut reachable = false;
    
    while let Some(cur) = queue.pop() {
        let (x, y) = cur;
        visited.insert(cur);
        
        let mut adjacent = vec![];
        if x != 1 {
            adjacent.push((x-1, y));
        }
        if y != 1 {
            adjacent.push((x, y-1));
        }
        unsafe {
            if x != SIZE {
                adjacent.push((x+1, y));
            }
            if y != SIZE {
                adjacent.push((x, y+1));
            }
        }
        for &newp in adjacent.iter() {
            if visited.contains(&newp) {
                continue
            }
            match *points.get(&newp).unwrap() {
                Reachable::Wall|Reachable::No => {}
                Reachable::Yes => {
                    reachable = true;
                }
                _ => {
                    queue.push(newp);
                }
            };
        }
    }
    if reachable {
        for &p in visited.iter() {
            points.insert(p, Reachable::Yes);
        }
    }
    else {
        for &p in visited.iter() {
            points.insert(p, Reachable::No);
        }
    }
}

unsafe fn solve() -> usize {
                     
    let mut points: HashMap<(u64, u64), Reachable> = HashMap::new();
    let bits: Vec<u64> = (0..33).map(|p| 1 << p).collect();

    for y in 1..SIZE+1 {
        for x in 1..SIZE+1 {
            let z = x*x*x + 12*x*y + 5*x*y*y;
            let even = bits.iter()
                .map(|b| if b & z == 0 { 0 } else  { 1 })
                .sum::<u64>() % 2 == 0;
            if even {
                points.insert((x, y), Reachable::Unknown);
            }
            else {
                points.insert((x, y), Reachable::Wall);
            }
        }
    }
    points.insert((1, 1), Reachable::Yes);
    
    for y in 2..SIZE+1 {
        for x in 1..y+1 {
            if x == y {
                find_any_path((x, x), &mut points);
            }
            else {
                find_any_path((x, y), &mut points);
                find_any_path((y, x), &mut points);
            }
        }
    }
    points.values().filter(|&r| *r == Reachable::No).count()
}

fn main() {
    unsafe {
        SIZE = 1000;
        println!("Unreachable points: {}", solve());
    }
}
