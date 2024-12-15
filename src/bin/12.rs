use std::{collections::HashSet, io};

use adventofcode_2024::*;

#[derive(Debug, Clone)]
struct Plot {
    plant: char,
    positions: HashSet<Pos>,
}

impl Plot {
    fn new(plant: char) -> Plot {
        Plot {
            plant,
            positions: HashSet::new(),
        }
    }
}

pub fn main() {
    let grid = Grid::new(
        io::stdin()
            .lines()
            .filter_map(|line| {
                let line = line.unwrap();
                let s = line.trim();
                if s.is_empty() {
                    None
                } else {
                    Some(s.chars().collect())
                }
            })
            .collect(),
    );

    let mut plots: Vec<Plot> = Vec::new();
    for y in 0..grid.inner.len() {
        for x in 0..grid.inner[y].len() {
            let pos = Pos(x, y);
            if let Some(&plant) = grid.get(pos) {
                if !plots.iter().any(|plot| plot.positions.contains(&pos)) {
                    plots.push(get_plot(&grid, Plot::new(plant), pos));
                }
            }
        }
    }

    let (part1, part2) = plots
        .iter()
        .map(|plot| determine_plot_sizes(&grid, plot))
        .fold((0, 0), |(p1, p2), sizes| {
            (
                p1 + sizes.area * sizes.perimeter,
                p2 + sizes.area * sizes.sides,
            )
        });

    println!("Part 1: {part1}",);
    println!("Part 2: {part2}",);
}

fn get_plot(grid: &Grid<char>, mut plot: Plot, pos: Pos) -> Plot {
    plot.positions.insert(pos);
    for v in CARDINAL_DIRECTIONS {
        if let Some(adj) = pos + v {
            if !plot.positions.contains(&adj) {
                if let Some(&plant) = grid.get(adj) {
                    if plant == plot.plant {
                        plot = get_plot(grid, plot, adj);
                    }
                }
            }
        }
    }

    plot
}

#[derive(Debug, Clone)]
struct PlotSizes {
    area: u32,
    perimeter: u32,
    sides: u32,
}

/// This is quite bad.
fn determine_plot_sizes(grid: &Grid<char>, plot: &Plot) -> PlotSizes {
    let min_x = plot.positions.iter().copied().map(|p| p.0).min().unwrap();
    let max_x = plot.positions.iter().copied().map(|p| p.0).max().unwrap();
    let min_y = plot.positions.iter().copied().map(|p| p.1).min().unwrap();
    let max_y = plot.positions.iter().copied().map(|p| p.1).max().unwrap();

    let mut area = 0;
    let mut perimeter = 0;
    let mut seen_wall_pairs: HashSet<(Pos, Direction)> = HashSet::new();
    let mut wall_pos: Option<Pos> = None;

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let pos = Pos(x, y);
            if plot.positions.contains(&pos) {
                area += 1;
                for dir in DIRECTIONS {
                    if let Some(adj) = pos + dir.unit_direction() {
                        if let Some(&plant) = grid.get(adj) {
                            if plant == plot.plant {
                                continue;
                            }
                        }
                    }
                    perimeter += 1;
                    if wall_pos.is_none() {
                        wall_pos = Some(pos);
                    }
                    seen_wall_pairs.insert((pos, dir));
                }
            }
        }
    }

    let mut sides = 0;
    let mut counted_pairs: HashSet<(Pos, Direction)> = HashSet::new();

    for &(wall_pos, wall_dir) in &seen_wall_pairs {
        if counted_pairs.contains(&(wall_pos, wall_dir)) {
            continue;
        }

        sides += 1;

        let mut counted_len = counted_pairs.len();
        counted_pairs.insert((wall_pos, wall_dir));
        while counted_len != counted_pairs.len() {
            counted_len = counted_pairs.len();

            for &(other_wall_pos, other_wall_dir) in &seen_wall_pairs {
                if counted_pairs.contains(&(other_wall_pos, other_wall_dir)) {
                    continue;
                }

                if other_wall_dir != wall_dir {
                    continue;
                }

                if let Some(adj_wall_pos) =
                    other_wall_pos + other_wall_dir.turned_left().unit_direction()
                {
                    if counted_pairs.contains(&(adj_wall_pos, wall_dir)) {
                        counted_pairs.insert((other_wall_pos, other_wall_dir));
                    }
                }

                if let Some(adj_wall_pos) =
                    other_wall_pos + other_wall_dir.turned_right().unit_direction()
                {
                    if counted_pairs.contains(&(adj_wall_pos, wall_dir)) {
                        counted_pairs.insert((other_wall_pos, other_wall_dir));
                    }
                }
            }
        }
    }

    return PlotSizes {
        area,
        perimeter,
        sides,
    };
}
