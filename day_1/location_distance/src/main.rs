use std::{fs::read_to_string, path::Path};

use timer_macro::timer;

#[timer]
fn location_distance(input: &str) -> i32 {
    let (mut vec1, mut vec2): (Vec<i32>, Vec<i32>) = input
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

    vec1.sort();
    vec2.sort();

    vec1.iter()
        .zip(vec2.iter())
        .map(|(val1, val2)| (val1 - val2).abs())
        .sum()
}

fn main() {
    let input = read_to_string(Path::new("src/data.txt")).unwrap();
    let result = location_distance(&input);
    println!("{result}")
}

#[cfg(test)]
mod location_distance_tests {
    use super::*;

    #[test]
    fn demo_test() {
        let input: &str = "3   4
4   3
2   5
1   3
3   9
3   3";
        let result = location_distance(input);
        assert_eq!(11, result)
    }
}
