use std::io;

use adventofcode_2024::*;

pub fn main() {
    let grid = Grid::new(
        io::stdin()
            .lines()
            .filter_map(|line| {
                let line = line.unwrap();
                if line.is_empty() {
                    None
                } else {
                    Some(line.chars().collect())
                }
            })
            .collect(),
    );

    let mut part1 = 0;
    let mut part2 = 0;
    for (y, line) in grid.inner.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            match *c {
                'X' => {
                    for dir in DIRECTIONS {
                        if word_exists_at(&grid, "XMAS", Pos(x, y), dir) {
                            part1 += 1;
                        }
                    }
                }
                'A' => {
                    let mut crosses = 0;
                    for dir in CROSS_DIRECTIONS {
                        if let Some(corner) = Pos(x, y) + dir {
                            if word_exists_at(&grid, "MAS", corner, dir * -1) {
                                crosses += 1;
                                if crosses == 2 {
                                    break;
                                }
                            }
                        }
                    }

                    if crosses == 2 {
                        part2 += 1;
                    }
                }
                _ => {}
            }
        }
    }

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}

fn word_exists_at(grid: &Grid<char>, word: &str, pos: Pos, vector: Vec2D) -> bool {
    let mut cur = pos;
    for (i, c) in word.chars().enumerate() {
        match grid.get(cur) {
            Some(x) if *x == c => {}
            _ => {
                return false;
            }
        }

        if i < word.len() - 1 {
            if let Some(new_pos) = cur + vector {
                cur = new_pos;
            } else {
                return false;
            }
        }
    }

    true
}
