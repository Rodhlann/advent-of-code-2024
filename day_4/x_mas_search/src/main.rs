use std::{fs::read_to_string, path::Path};

use timer_macro::timer;

#[derive(Debug, Clone, Copy)]
enum Direction {
    NE,
    SE,
    SW,
    NW,
}

impl Direction {
    const VALUES: [Self; 4] = [Self::NE, Self::SE, Self::SW, Self::NW];

    const fn coords(&self) -> (i32, i32) {
        match self {
            Direction::NE => (1, -1),
            Direction::SE => (1, 1),
            Direction::SW => (-1, 1),
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

    fn find_pattern(&self, x1: usize, y1: usize) -> bool {
        let mut ne_sw = Vec::new();
        let mut se_nw = Vec::new();

        for &dir in Direction::VALUES.iter() {
            match dir {
                Direction::NE | Direction::SW => {
                    if let Some((x2, y2)) = self.get_next_coord(x1, y1, dir) {
                        ne_sw.push(self.matrix[y2][x2]);
                    }
                }
                Direction::SE | Direction::NW => {
                    if let Some((x2, y2)) = self.get_next_coord(x1, y1, dir) {
                        se_nw.push(self.matrix[y2][x2]);
                    }
                }
            }
        }

        let check_pattern = |chars: &[char]| {
            if chars.len() != 2 {
                return false;
            }
            let mut sorted = chars.to_vec();
            sorted.sort();
            sorted[0] == 'M' && sorted[1] == 'S'
        };

        check_pattern(&ne_sw) && check_pattern(&se_nw)
    }

    fn count_xmas_patterns(&self) -> i32 {
        let mut count = 0;

        for y in 0..self.h {
            for x in 0..self.w {
                if self.matrix[y][x] == 'A' {
                    if self.find_pattern(x, y) {
                        count += 1;
                    }
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
        let input: &str = "MMASSAMSSM
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
..........";

        let result = xmas_search(input);
        assert_eq!(9, result)
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
