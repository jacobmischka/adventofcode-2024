use std::{collections::HashSet, io};

use adventofcode_2024::*;

pub fn main() {
    let mut pos: Option<Pos> = None;
    let grid = Grid::new(
        io::stdin()
            .lines()
            .enumerate()
            .filter_map(|(y, line)| {
                let line = line.unwrap();
                if line.is_empty() {
                    None
                } else {
                    Some(
                        line.chars()
                            .enumerate()
                            .map(|(x, c)| {
                                if c == '^' {
                                    pos = Some(Pos(x, y));
                                }

                                c
                            })
                            .collect(),
                    )
                }
            })
            .collect(),
    );

    let pos = pos.unwrap();

    let (visited_positions, cycle_obstacle_locations) =
        patrol_for_cycles(&grid, pos, Direction::Up);

    println!("Part 1: {}", visited_positions.len());
    println!("Part 2: {}", cycle_obstacle_locations.len());
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum EndReason {
    Cycle,
    Edge,
}

fn patrol_for_cycles(
    grid: &Grid<char>,
    start_pos: Pos,
    start_dir: Direction,
) -> (HashSet<Pos>, HashSet<Pos>) {
    let mut visited: HashSet<(Pos, Direction)> = HashSet::new();

    let mut cycle_obstacle_locations: HashSet<Pos> = HashSet::new();

    perform_patrol(grid, start_pos, start_dir, &mut visited, None);

    let visited_positions: HashSet<_> = visited.into_iter().map(|(pos, _)| pos).collect();
    for pos in &visited_positions {
        let mut visited = HashSet::new();
        if perform_patrol(grid, start_pos, start_dir, &mut visited, Some(*pos)) == EndReason::Cycle
        {
            cycle_obstacle_locations.insert(*pos);
        }
    }

    (visited_positions, cycle_obstacle_locations)
}

fn perform_patrol(
    grid: &Grid<char>,
    mut pos: Pos,
    mut dir: Direction,
    visited: &mut HashSet<(Pos, Direction)>,
    additional_obstacle_pos: Option<Pos>,
) -> EndReason {
    while !visited.contains(&(pos, dir)) {
        visited.insert((pos, dir));

        if let Some(in_front) = pos + dir.unit_direction() {
            if let Some(obj) = grid.get(in_front) {
                if *obj == '#' || additional_obstacle_pos == Some(in_front) {
                    dir = dir.turned_right();
                } else {
                    pos = in_front;
                }
            } else {
                return EndReason::Edge;
            }
        } else {
            return EndReason::Edge;
        }
    }

    EndReason::Cycle
}
