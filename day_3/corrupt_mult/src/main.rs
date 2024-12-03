use regex::Regex;
use std::{fs::read_to_string, path::Path};

use timer_macro::timer;

#[timer]
fn corrupt_mult(input: &str) -> i32 {
    let mul_rule = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    mul_rule
        .captures_iter(input)
        .map(|caps| {
            let a: i32 = caps[1].parse().unwrap();
            let b: i32 = caps[2].parse().unwrap();
            a * b
        })
        .sum()
}

fn main() {
    let input = read_to_string(Path::new("src/data.txt")).unwrap();
    let result = corrupt_mult(&input);
    println!("{result}")
}

#[cfg(test)]
mod corrupt_mult_tests {
    use super::*;

    #[test]
    fn demo_test() {
        let input: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

        let result = corrupt_mult(input);
        assert_eq!(161, result)
    }
}
