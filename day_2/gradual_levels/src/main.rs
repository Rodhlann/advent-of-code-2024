use std::{fs::read_to_string, path::Path};

use timer_macro::timer;

#[timer]
fn gradual_levels(input: &str) -> usize {
    let lines = input.lines();
    let mut count = 0;
    for line in lines {
        let mut dir = 0; // 0 undef - 1 asc - 2 desc
        let chars: Vec<i16> = line
            .split(" ")
            .map(|char| char.parse::<i16>().unwrap())
            .collect();
        for (idx, &char) in chars.iter().enumerate() {
            if chars.len() == idx + 1 {
                count += 1;
                break;
            }
            let next = chars[idx + 1];
            match dir {
                0 => {
                    dir = if char < next {
                        if next - char > 3 || next - char < 1 {
                            break;
                        }
                        1
                    } else {
                        if char - next > 3 || char - next < 1 {
                            break;
                        }
                        2
                    };
                }
                1 => {
                    if char > next || next - char > 3 || next - char < 1 {
                        break;
                    }
                }
                2 => {
                    if char < next || char - next > 3 || char - next < 1 {
                        break;
                    }
                }
                _ => panic!("Unknown state"),
            }
        }
    }
    count
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
