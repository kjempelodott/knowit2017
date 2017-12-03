extern crate itertools;
extern crate image;

use itertools::Itertools;
use image::DynamicImage::ImageRgb8;

fn main() {
    if let ImageRgb8(img) = image::open("res/day3.png").unwrap() {
        println!("{}", 
                 img.pixels()
                 .map(|&p| p[0] & 1)
                 .chunks(8)
                 .into_iter()
                 .map(|byte| (0..8).zip(byte).map(|(s, b)| b << s).sum())
                 .take_while(|c: &u8| *c != 0 && *c < 128)
                 .map(|c| c as char)
                 .collect::<String>());
    }
}
