use std::{fs::read_to_string, path::Path};

use timer_macro::timer;

#[timer]
fn gradual_levels(input: &str) -> usize {
    input.lines().filter(|line| is_gradual_level(line)).count()
}

fn is_gradual_level(line: &str) -> bool {
    let numbers: Result<Vec<i16>, _> = line
        .split_whitespace()
        .map(|char| char.parse::<i16>())
        .collect();

    match numbers {
        Ok(nums) => verify_gradual_levels(&nums),
        Err(_e) => false,
    }
}

fn verify_gradual_levels(nums: &[i16]) -> bool {
    if nums.is_empty() {
        return false;
    }

    let mut sign = None;

    for window in nums.windows(2) {
        let diff = window[0] - window[1];

        if diff.abs() < 1 || diff.abs() > 3 {
            return false;
        }

        match sign {
            None => sign = Some(diff.signum()),
            Some(prev) if prev != diff.signum() => return false,
            _ => {}
        }
    }

    true
}

fn main() {
    let input = read_to_string(Path::new("src/data.txt")).unwrap();
    let result = gradual_levels(&input);
    println!("{result}")
}

#[cfg(test)]
mod gradual_levels_tests {
    use super::*;

    #[test]
    fn demo_test() {
        let input: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
72 72 70 68 67 65";

        let result = gradual_levels(input);
        assert_eq!(2, result)
    }
}
