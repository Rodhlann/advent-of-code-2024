use std::{fs::read_to_string, path::Path};

use timer_macro::timer;

#[timer]
fn gradual_levels_tolerate(input: &str) -> usize {
    input.lines().filter(|line| is_gradual_level(line)).count()
}

fn is_gradual_level(line: &str) -> bool {
    let numbers: Result<Vec<i16>, _> = line
        .split_whitespace()
        .map(|char| char.parse::<i16>())
        .collect();

    match numbers {
        Ok(nums) => {
            if let Some([x, y]) = verify_gradual_levels_tolerate(&nums) {
                let mut new_nums_x = nums.clone();
                new_nums_x.drain(x..x + 1);
                if verify_gradual_levels_tolerate(&new_nums_x).is_some() {
                    let mut new_nums_y = nums.clone();
                    new_nums_y.drain(y..y + 1);
                    if verify_gradual_levels_tolerate(&new_nums_y).is_some() {
                        let mut new_nums_z = nums.clone();
                        new_nums_z.drain(0..0 + 1);
                        if verify_gradual_levels_tolerate(&new_nums_z).is_some() {
                            return false;
                        }
                    }
                }
            }
            true
        }
        Err(_e) => false,
    }
}

fn verify_gradual_levels_tolerate(nums: &[i16]) -> Option<[usize; 2]> {
    if nums.is_empty() {
        return None;
    }

    let mut sign = None;

    for (idx, window) in nums.windows(2).enumerate() {
        let diff = window[0] - window[1];

        if diff.abs() < 1 || diff.abs() > 3 {
            return Some([idx, idx + 1]);
        }

        match sign {
            None => sign = Some(diff.signum()),
            Some(prev) if prev != diff.signum() => {
                return Some([idx, idx + 1]);
            }
            _ => {}
        }
    }

    None
}

fn main() {
    let input = read_to_string(Path::new("src/data.txt")).unwrap();
    let result = gradual_levels_tolerate(&input);
    println!("{result}")
}

#[cfg(test)]
mod gradual_level_tolerate_tests {
    use super::*;

    #[test]
    fn demo_test() {
        let input: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

        let result = gradual_levels_tolerate(input);
        assert_eq!(4, result)
    }
}
