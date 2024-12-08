use std::{
    collections::{HashMap, HashSet},
    io,
};

use adventofcode_2024::*;

pub fn main() {
    let mut grid_size = (0, 0);
    let mut antennas: HashMap<char, Vec<Pos>> = HashMap::new();
    for (y, line) in io::stdin().lines().enumerate() {
        let line = line.unwrap();
        let s = line.trim();
        if s.is_empty() {
            continue;
        }
        grid_size.1 = y;

        for (x, c) in s.char_indices() {
            grid_size.0 = grid_size.0.max(x);
            if !c.is_alphanumeric() {
                continue;
            }

            antennas.entry(c).or_default().push(Pos(x, y));
        }
    }

    let mut antinodes: HashSet<Pos> = HashSet::new();
    let mut resonant_antinodes: HashSet<Pos> = HashSet::new();

    for freq_antennas in antennas.values() {
        for i in 1..freq_antennas.len() {
            for j in 0..i {
                let dist = freq_antennas[i].dist(freq_antennas[j]);
                if let Some(antinode) = freq_antennas[i] - dist {
                    if in_grid(grid_size, antinode) {
                        antinodes.insert(antinode);
                    }
                }

                if let Some(antinode) = freq_antennas[j] + dist {
                    if in_grid(grid_size, antinode) {
                        antinodes.insert(antinode);
                    }
                }

                resonant_antinodes.insert(freq_antennas[i]);
                let mut pos = freq_antennas[i];
                while in_grid(grid_size, pos) {
                    resonant_antinodes.insert(pos);
                    if let Some(new_pos) = pos - dist {
                        pos = new_pos;
                    } else {
                        break;
                    }
                }

                pos = freq_antennas[i];
                while in_grid(grid_size, pos) {
                    resonant_antinodes.insert(pos);
                    if let Some(new_pos) = pos + dist {
                        pos = new_pos;
                    } else {
                        break;
                    }
                }
            }
        }
    }

    println!("Part 1: {}", antinodes.len());
    println!("Part 2: {}", resonant_antinodes.len());
}

fn in_grid(inclusive_grid_size: (usize, usize), pos: Pos) -> bool {
    pos.0 <= inclusive_grid_size.0 && pos.1 <= inclusive_grid_size.1
}
