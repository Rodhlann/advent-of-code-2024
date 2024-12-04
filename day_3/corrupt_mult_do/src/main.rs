use regex::Regex;
use std::{fs::read_to_string, path::Path};

use timer_macro::timer;

enum Event {
    Do,
    Dont,
    Mul(i32),
}

#[timer]
fn corrupt_mult(input: &str) -> i32 {
    let mul_rule = Regex::new(r"mul\(\d+,\d+\)").unwrap();
    let dig_rule = Regex::new(r"(\d+),(\d+)").unwrap();

    let mut events: Vec<(usize, Event)> = Vec::new();

    events.extend(
        input
            .match_indices(r"do()")
            .map(|(idx, _)| (idx, Event::Do)),
    );

    events.extend(
        input
            .match_indices(r"don't()")
            .map(|(idx, _)| (idx, Event::Dont)),
    );

    events.extend(mul_rule.find_iter(input).map(|chunk| {
        let s = chunk.as_str();
        let idx = input.find(s).unwrap();
        let val: i32 = dig_rule
            .captures_iter(s)
            .map(|caps| {
                let a: i32 = caps[1].parse().unwrap();
                let b: i32 = caps[2].parse().unwrap();
                a * b
            })
            .sum();
        (idx, Event::Mul(val))
    }));

    events.sort_by_key(|&(idx, _)| idx);

    let mut can_add = true;
    let mut total = 0;

    for (_, val) in events {
        match val {
            Event::Do => can_add = true,
            Event::Dont => can_add = false,
            Event::Mul(val) => {
                if can_add {
                    total += val
                }
            }
        }
    }

    total
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
        let input: &str =
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

        let result = corrupt_mult(input);
        assert_eq!(48, result)
    }
}
