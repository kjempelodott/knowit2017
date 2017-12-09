const MAX: u32 = 130_000;

fn main() {
    println!("{}",
             (1..MAX/2+1)
             .map(|i|
                  (i+1..MAX/2+1)
                  .scan(i, |sum, j| {
                      *sum = *sum + j;
                      Some(*sum)
                  })
                  .take_while(|&sum| sum <= MAX)
                  .count()).sum::<usize>());
}
