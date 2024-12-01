use std::{collections::HashMap, fs::read_to_string, path::Path};

use timer_macro::timer;

#[timer]
fn location_multiplier(input: &str) -> i32 {
    let (vec1, vec2): (Vec<i32>, Vec<i32>) = input
        .split("\n")
        .filter(|v| !v.is_empty())
        .map(|line| {
            let values: Vec<&str> = line.split("   ").collect();
            (
                values[0].parse::<i32>().unwrap(),
                values[1].parse::<i32>().unwrap(),
            )
        })
        .unzip();

    let count_map: HashMap<i32, usize> = vec2.iter().fold(HashMap::new(), |mut acc, &val| {
        *acc.entry(val).or_insert(0) += 1;
        acc
    });

    vec1.iter()
        .map(|&val1| val1 * count_map.get(&val1).cloned().unwrap_or(0) as i32)
        .sum()
}

fn main() {
    let input = read_to_string(Path::new("src/data.txt")).unwrap();
    let result = location_multiplier(&input);
    println!("{result}")
}

#[cfg(test)]
mod location_multiplier_tests {
    use super::*;

    #[test]
    fn demo_test() {
        let input: &str = "3   4
4   3
2   5
1   3
3   9
3   3";
        let result = location_multiplier(input);
        assert_eq!(31, result)
    }
}
