use std::{fs::read_to_string, path::Path};

use timer_macro::timer;

#[derive(Debug, Clone, Copy)]
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

    const fn coords(&self) -> (i32, i32) {
        match self {
            Direction::N => (0, -1),
            Direction::NE => (1, -1),
            Direction::E => (1, 0),
            Direction::SE => (1, 1),
            Direction::S => (0, 1),
            Direction::SW => (-1, 1),
            Direction::W => (-1, 0),
            Direction::NW => (-1, -1),
        }
    }
}

struct PathFinder<'a> {
    matrix: &'a Vec<Vec<char>>,
    w: usize,
    h: usize,
}

impl<'a> PathFinder<'a> {
    fn new(matrix: &'a Vec<Vec<char>>) -> Self {
        PathFinder {
            matrix,
            w: matrix[0].len(),
            h: matrix.len(),
        }
    }

    fn in_bounds(&self, x: i32, y: i32) -> bool {
        x >= 0 && x < self.w as i32 && y >= 0 && y < self.h as i32
    }

    fn get_next_coord(&self, x: usize, y: usize, dir: Direction) -> Option<(usize, usize)> {
        let coords = dir.coords();
        let x1 = x as i32 + coords.0;
        let y1 = y as i32 + coords.1;
        if self.in_bounds(x1, y1) {
            Some((x1 as usize, y1 as usize))
        } else {
            None
        }
    }

    fn find_pattern(&self, x1: usize, y1: usize, dir: Direction) -> bool {
        const PATTERN: [char; 4] = ['X', 'M', 'A', 'S'];

        let mut cx = x1;
        let mut cy = y1;

        for &ch in &PATTERN[1..] {
            match self.get_next_coord(cx, cy, dir) {
                Some((x2, y2)) => {
                    if self.matrix[y2][x2] != ch {
                        return false;
                    }
                    cx = x2;
                    cy = y2;
                }
                None => return false,
            }
        }

        true
    }

    fn count_xmas_patterns(&self) -> i32 {
        let mut count = 0;

        for (y, line) in self.matrix.iter().enumerate() {
            for (x, &ch) in line.iter().enumerate() {
                if ch == 'X' {
                    count += Direction::VALUES
                        .iter()
                        .filter(|&&dir| self.find_pattern(x, y, dir))
                        .count() as i32
                }
            }
        }

        count
    }
}

#[timer]
fn xmas_search(input: &str) -> i32 {
    let matrix: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().filter(|ch| !ch.is_whitespace()).collect())
        .collect();

    let path_finder = PathFinder::new(&matrix);
    path_finder.count_xmas_patterns()
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
