#[macro_use]
extern crate lazy_static;

static ALPHA: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
lazy_static!{
    static ref CIPHER: String = {
        ALPHA.bytes()
            .map(|c| {
                let rot = (2*c - 64) % 26;
                if c + rot > 90 { 65 + ((c + rot) % 91) } else { c + rot }
            })
            .map(|c| c as char)
            .collect::<String>()
    };
}

fn solve(msg: &str) -> String {
    msg.chars()
        .map(|c| ALPHA.chars().skip(CIPHER.find(c).unwrap()).next().unwrap())
        .collect::<String>()
}

fn main() {
    println!("{}", solve("OTUJNMQTYOQOVVNEOXQVAOXJEYA"))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(solve("PWVAYOBB"), "JULEMANN")
    }
}
