use std::{fs::read_to_string, path::Path};

use timer_macro::timer;

#[derive(Debug)]
enum Direction {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
}

impl Direction {
    const VALUES: [Self; 8] = [
        Self::N,
        Self::NE,
        Self::E,
        Self::SE,
        Self::S,
        Self::SW,
        Self::W,
        Self::NW,
    ];

    fn coords(&self) -> [i32; 2] {
        match self {
            Direction::N => [0, -1],
            Direction::NE => [1, -1],
            Direction::E => [1, 0],
            Direction::SE => [1, 1],
            Direction::S => [0, 1],
            Direction::SW => [-1, 1],
            Direction::W => [-1, 0],
            Direction::NW => [-1, -1],
        }
    }
}

fn in_bounds(x: usize, y: usize, w: usize, h: usize) -> bool {
    x > 0 && x < w && y > 0 && y < h
}

fn get_valid_dir(
    ch: char,
    x: usize,
    y: usize,
    w: usize,
    h: usize,
    matrix: Vec<Vec<char>>,
) -> Option<Direction> {
    for dir in Direction::VALUES {
        let x1 = (x as i32 + dir.coords()[0]) as usize;
        let y1 = (y as i32 + dir.coords()[1]) as usize;
        if in_bounds(x1, y1, w, h) {
            let sib = matrix[y1][x1];
            if sib == ch {
                return Some(dir);
            }
        }
    }
    None
}

// TODO the reason this is currently broken is I need to continue searching if a path fails until all directions have been expended
// I should be able to figure out a way to reduce some duplication for checking down a line of characters...

#[timer]
fn xmas_search(input: &str) -> i32 {
    let matrix: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().filter(|c| !c.is_whitespace()).collect())
        .collect();

    let w = matrix[0].len();
    let h = matrix.len();

    let mut count = 0;
    for (y, line) in matrix.iter().enumerate() {
        for (x, &ch) in line.iter().enumerate() {
            if ch == 'X' {
                match get_valid_dir('M', x, y, w, h, matrix.clone()) {
                    Some(dir) => {
                        let x1 = (x as i32 + dir.coords()[0]) as usize;
                        let y1 = (y as i32 + dir.coords()[1]) as usize;
                        if in_bounds(x1, y1, w, h) {
                            println!("{x1},{y1}");
                            let sib = matrix[y1][x1];
                            println!("{sib}");
                            if 'A' == sib {
                                println!("{:?}", dir);
                                let x2 = (x as i32 + dir.coords()[0]) as usize;
                                let y2 = (y as i32 + dir.coords()[1]) as usize;
                                if in_bounds(x2, y2, w, h) {
                                    let sib = matrix[y2][x2];
                                    if 'S' == sib {
                                        count += 1;
                                    }
                                }
                            }
                        }
                    }
                    None => (),
                }
            }
        }
    }

    count
}

fn main() {
    let input = read_to_string(Path::new("src/data.txt")).unwrap();
    let result = xmas_search(&input);
    println!("{result}")
}

#[cfg(test)]
mod xmas_search_tests {
    use super::*;

    #[test]
    fn demo_test() {
        let input: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

        let result = xmas_search(input);
        assert_eq!(2, result)
    }
}
