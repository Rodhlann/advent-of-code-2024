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

fn in_bounds(x: i32, y: i32, w: usize, h: usize) -> bool {
    x >= 0 && x < w as i32 && y >= 0 && y < h as i32
}

fn sibling_coords(x: usize, y: usize, w: usize, h: usize, dir: &Direction) -> Option<[usize; 2]> {
    let coords = dir.coords();
    let x1 = x as i32 + coords[0];
    let y1 = y as i32 + coords[1];
    if in_bounds(x1, y1, w, h) {
        Some([x1 as usize, y1 as usize])
    } else {
        None
    }
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
                for dir in Direction::VALUES {
                    if let Some([x1, y1]) = sibling_coords(x, y, w, h, &dir) {
                        if matrix[y1][x1] == 'M' {
                            if let Some([x2, y2]) = sibling_coords(x1, y1, w, h, &dir) {
                                if matrix[y2][x2] == 'A' {
                                    if let Some([x3, y3]) = sibling_coords(x2, y2, w, h, &dir) {
                                        if matrix[y3][x3] == 'S' {
                                            count += 1;
                                        }
                                    }
                                }
                            }
                        }
                    }
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
        assert_eq!(18, result)
    }
}

// M M M S X X M A S M
// M S A M X M S M S A
// A M X S X M A A M M
// M S A M A S M S M X
// X M A S A M X A M M
// X X A M M X X A M A
// S M S M S A S X S S
// S A X A M A S A A A
// M A M M M X M M M M
// M X M X A X M A S X
