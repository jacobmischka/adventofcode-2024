use std::{fmt::Display, io};

use adventofcode_2024::*;

enum ParseState {
    Grid,
    Directions,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Robot,
    Box,
    Wall,
    Empty,
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Robot => write!(f, "@"),
            Tile::Box => write!(f, "O"),
            Tile::Wall => write!(f, "#"),
            Tile::Empty => write!(f, "."),
        }
    }
}

impl TryFrom<char> for Tile {
    type Error = String;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '@' => Ok(Tile::Robot),
            '.' => Ok(Tile::Empty),
            'O' => Ok(Tile::Box),
            '#' => Ok(Tile::Wall),
            c => Err(format!("invalid tile {c}")),
        }
    }
}

pub fn main() {
    let mut grid: Grid<Tile> = Grid::new(Vec::new());
    let mut directions: Vec<Direction> = Vec::new();
    let mut state = ParseState::Grid;
    let mut pos: Option<Pos> = None;
    for line in io::stdin().lines().map(|l| l.unwrap()) {
        let s = line.trim();
        if s.is_empty() {
            state = ParseState::Directions;
            continue;
        }

        match state {
            ParseState::Grid => {
                grid.inner.push(
                    s.char_indices()
                        .map(|(i, c)| {
                            let tile = Tile::try_from(c).unwrap();
                            match tile {
                                Tile::Robot => {
                                    pos = Some(Pos(i, grid.inner.len()));
                                }
                                _ => {}
                            }
                            tile
                        })
                        .collect(),
                );
            }
            ParseState::Directions => {
                directions.extend(s.chars().map(|c| Direction::try_from(c).unwrap()));
            }
        }
    }

    let mut pos = pos.unwrap();

    for dir in &directions {
        pos = do_move(&mut grid, pos, *dir);
    }

    println!(
        "Part 1: {}",
        grid.inner.iter().enumerate().fold(0, |sum, (y, row)| {
            sum + row.iter().enumerate().fold(0, |row_sum, (x, tile)| {
                if *tile == Tile::Box {
                    row_sum + gps_coordinate(Pos(x, y))
                } else {
                    row_sum
                }
            })
        })
    )
}

fn gps_coordinate(pos: Pos) -> usize {
    pos.1 * 100 + pos.0
}

fn do_move(grid: &mut Grid<Tile>, pos: Pos, dir: Direction) -> Pos {
    let mut stack: Vec<Pos> = vec![pos];
    let mut cur = pos;
    let v = dir.unit_direction();
    while let Some(next_pos) = cur + v {
        match grid.get(next_pos) {
            Some(Tile::Robot) => {
                panic!("robot position lost");
            }
            Some(Tile::Empty) => {
                stack.push(next_pos);
                break;
            }
            Some(Tile::Box) => {
                stack.push(next_pos);
                cur = next_pos;
            }
            Some(Tile::Wall) => {
                break;
            }
            None => {
                break;
            }
        }
    }

    if let Some(mut end) = stack.pop() {
        if grid.get(end).copied() == Some(Tile::Empty) {
            while let Some(next) = stack.pop() {
                if let Some(next_tile) = grid.get(next).copied() {
                    grid.set(end, next_tile);
                    end = next;
                }
            }
            grid.set(pos, Tile::Empty).unwrap();
            (pos + v).unwrap()
        } else {
            pos
        }
    } else {
        pos
    }
}
