fn solve(max: usize) -> u64 {
    let mut nums: Vec<u64> = vec![0, 1, 2, 2];
    while nums.len() < max + 1 {
        let num = nums.last().unwrap() + 1;
        let n = *nums.get(num as usize).unwrap();
        nums.append(&mut vec![num; n as usize]);
    }
    nums.iter().take(1_000_001).sum()
}

fn main() {
    println!("{}", solve(1_000_000));
}
