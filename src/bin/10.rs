use std::{collections::HashSet, io};

use adventofcode_2024::*;

pub fn main() {
    let mut start_points: Vec<Pos> = Vec::new();
    let grid = Grid::new(
        io::stdin()
            .lines()
            .map(|l| l.unwrap())
            .enumerate()
            .filter_map(|(y, l)| {
                let s = l.trim();
                if s.is_empty() {
                    None
                } else {
                    Some(
                        s.char_indices()
                            .map(|(x, c)| {
                                if let Some(n) = c.to_digit(10).map(|n| n as u8) {
                                    if n == 0 {
                                        start_points.push(Pos(x, y));
                                    }
                                    n
                                } else {
                                    u8::MAX
                                }
                            })
                            .collect(),
                    )
                }
            })
            .collect(),
    );

    let (part1, part2): (usize, usize) = start_points.iter().copied().fold((0, 0), |acc, pos| {
        let ends = walk_trail(&grid, pos);
        let unique_ends: HashSet<_> = ends.iter().copied().collect();

        (acc.0 + unique_ends.len(), acc.1 + ends.len())
    });
    println!("Part 1: {part1}",);
    println!("Part 2: {part2}",);
}

fn walk_trail(grid: &Grid<u8>, pos: Pos) -> Vec<Pos> {
    if let Some(val) = grid.get(pos) {
        if *val == 9 {
            return vec![pos];
        }

        CARDINAL_DIRECTIONS
            .into_iter()
            .filter_map(|dir| {
                (pos + dir).and_then(|next_pos| {
                    grid.get(next_pos).and_then(|next_val| {
                        if *next_val == *val + 1 {
                            Some(walk_trail(grid, next_pos))
                        } else {
                            None
                        }
                    })
                })
            })
            .flatten()
            .collect()
    } else {
        Vec::new()
    }
}
