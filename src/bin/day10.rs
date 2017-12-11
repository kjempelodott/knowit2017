fn main() {
    let mut people: Vec<bool> = vec![true; 1500];
    let mut pass_on = true;
    while people.iter().filter(|&&p| p).count() > 1 {
        (0..1500).for_each(|i| {
            if people[i] {
                if !pass_on {
                    people[i] = false;
                }
                pass_on ^= true;
            }
        });
    }
    println!("{}", people.iter()
             .enumerate()
             .filter(|&(_, &p)| p)
             .next().unwrap().0);
}
