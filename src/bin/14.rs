use std::{collections::HashMap, io, str::FromStr};

use adventofcode_2024::*;

const GRID_SIZE: (usize, usize) = (101, 103);
// const GRID_SIZE: (usize, usize) = (11, 7);

#[derive(Debug, Clone)]
struct Robot {
    pos: Pos,
    vel: Vec2D,
}

impl Robot {
    fn step(&mut self) {
        self.pos = self.pos.wrapping_add(self.vel, GRID_SIZE);
    }
}

impl FromStr for Robot {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let make_err = || format!("invalid robot str {s}");
        let map_err = |err| format!("invalid robot str {s}: {err}");
        let mut pieces = s.split_ascii_whitespace();
        let pos_str = pieces.next().ok_or_else(make_err)?;
        let mut pos_vals = pos_str[2..].split(',').map(|v| v.parse::<usize>());
        let pos = Pos(
            pos_vals.next().ok_or_else(make_err)?.map_err(map_err)?,
            pos_vals.next().ok_or_else(make_err)?.map_err(map_err)?,
        );

        let vel_str = pieces.next().ok_or_else(make_err)?;
        let mut vel_vals = vel_str[2..].split(',').map(|v| v.parse::<isize>());
        let vel = Vec2D(
            vel_vals.next().ok_or_else(make_err)?.map_err(map_err)?,
            vel_vals.next().ok_or_else(make_err)?.map_err(map_err)?,
        );

        Ok(Self { pos, vel })
    }
}

pub fn main() {
    let mut robots: Vec<Robot> = io::stdin()
        .lines()
        .filter_map(|line| {
            let line = line.unwrap();
            let s = line.trim();
            if s.is_empty() {
                None
            } else {
                Some(Robot::from_str(s).unwrap())
            }
        })
        .collect();

    for i in 0..1000000000 {
        if i == 100 {
            println!("Part 1: {}", get_safety_factor(&robots));
        }

        for robot in &mut robots {
            robot.step();
        }

        let (ul, ur, ll, lr) = get_quadrants(&robots);
        if (ul.len() as isize - ur.len() as isize).abs() < 2
            && (ll.len() as isize - lr.len() as isize).abs() < 2
        {
            let (bot_map, p) = symmetrical_percentage(&robots);
            if p > 0.5 {
                dump_robots(&bot_map);
                println!("Part 2: {i}");
            }
        }
        println!("{i}:");
        println!();
    }
}

fn symmetrical_percentage(robots: &Vec<Robot>) -> (HashMap<Pos, Vec<&Robot>>, f64) {
    let mut robot_map: HashMap<Pos, Vec<&Robot>> = HashMap::new();
    for robot in robots {
        robot_map.entry(robot.pos).or_default().push(robot);
    }

    let mut count = 0;

    for y in 0..=GRID_SIZE.1 {
        for x in 0..(GRID_SIZE.0 / 2) {
            if let (Some(lbots), Some(rbots)) = (
                robot_map.get(&Pos(x, y)),
                robot_map.get(&Pos(
                    GRID_SIZE.0.checked_add_signed(x as isize * -1).unwrap(),
                    y,
                )),
            ) {
                count += lbots.len() + rbots.len();
            }
        }
    }

    (robot_map, count as f64 / robots.len() as f64)
}

fn dump_robots(robot_map: &HashMap<Pos, Vec<&Robot>>) {
    for y in 0..=GRID_SIZE.1 {
        for x in 0..=GRID_SIZE.0 {
            if let Some(len) = robot_map.get(&Pos(x, y)).map(|r| r.len()) {
                print!("{}", len);
            } else {
                print!(".",);
            }
        }
        println!();
    }
}

fn get_safety_factor(robots: &Vec<Robot>) -> usize {
    let (ul, ur, ll, lr) = get_quadrants(robots);
    ul.len() * ur.len() * ll.len() * lr.len()
}

fn get_quadrants(robots: &Vec<Robot>) -> (Vec<&Robot>, Vec<&Robot>, Vec<&Robot>, Vec<&Robot>) {
    let mut ul = Vec::new();
    let mut ur = Vec::new();
    let mut ll = Vec::new();
    let mut lr = Vec::new();

    for robot in robots {
        match (
            robot.pos.0 < GRID_SIZE.0 / 2,
            robot.pos.1 < GRID_SIZE.1 / 2,
            robot.pos.0 > GRID_SIZE.0 / 2,
            robot.pos.1 > GRID_SIZE.1 / 2,
        ) {
            (true, true, _, _) => {
                ul.push(robot);
            }
            (true, false, _, true) => {
                ll.push(robot);
            }
            (false, true, true, _) => {
                ur.push(robot);
            }
            (false, false, true, true) => {
                lr.push(robot);
            }
            _ => {}
        }
    }

    (ul, ur, ll, lr)
}
