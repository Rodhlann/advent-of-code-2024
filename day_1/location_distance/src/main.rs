use std::{fs::read_to_string, path::Path};

fn location_distance(input: &str) -> i32 {
    let (mut vec1, mut vec2): (Vec<&str>, Vec<&str>) = input
        .split("\n")
        .filter(|v| !v.is_empty())
        .fold((Vec::new(), Vec::new()), |mut acc, next| {
            let values: Vec<&str> = next.split("   ").collect();
            acc.0.push(values[0]);
            acc.1.push(values[1]);
            acc
        });

    vec1.sort();
    vec2.sort();

    vec1.iter().enumerate().fold(0, |mut acc, (idx, val)| {
        let val = val.parse::<i32>().unwrap() - vec2[idx].parse::<i32>().unwrap();
        acc += val.abs();
        acc
    })
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
