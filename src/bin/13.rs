use std::io;

use adventofcode_2024::*;

const FLOATING_POINT_BUFFER: f64 = 0.001;

#[derive(Debug, Clone, Default)]
struct Machine {
    a: Vec2D,
    b: Vec2D,
    prize: (f64, f64),
}

pub fn main() {
    let mut machines: Vec<Machine> = Vec::new();
    let mut machine = Machine::default();

    for line in io::stdin().lines() {
        let line = line.unwrap();
        let s = line.trim();
        if s.is_empty() {
            machines.push(machine);
            machine = Machine::default();
        } else if s.starts_with("Button") {
            let half = s.split(": ").skip(1).next().unwrap();
            let mut coords = half.split(", ").map(|s| {
                assert_eq!(&s[1..2], "+");
                s[2..].parse().unwrap()
            });
            if s.starts_with("Button A:") {
                machine.a = Vec2D(coords.next().unwrap(), coords.next().unwrap());
            } else {
                machine.b = Vec2D(coords.next().unwrap(), coords.next().unwrap());
            }
        } else if s.starts_with("Prize:") {
            let half = s.split(": ").skip(1).next().unwrap();
            let mut coords = half.split(", ").map(|s| s[2..].parse().unwrap());
            machine.prize = (coords.next().unwrap(), coords.next().unwrap());
        }
    }
    if machine.prize != (0.0, 0.0) {
        machines.push(machine);
    }

    println!(
        "Part 1: {}",
        machines
            .iter()
            .map(|machine| win(machine).unwrap_or(0))
            .sum::<u64>()
    );
    println!(
        "Part 2: {}",
        machines
            .into_iter()
            .map(|mut machine| {
                machine.prize.0 += 10000000000000.0;
                machine.prize.1 += 10000000000000.0;
                win(&machine).unwrap_or(0)
            })
            .sum::<u64>()
    );
}

fn win(machine: &Machine) -> Option<u64> {
    // machine.b.0 * b = machine.prize.0 - machine.a.0 * a
    // machine.b.1 * b = machine.prize.1 - machine.a.1 * a
    //
    // b = x * a + c = y * a + d

    let x = -1.0 * (machine.a.0 as f64 / machine.b.0 as f64);
    let c = machine.prize.0 / machine.b.0 as f64;

    let y = -1.0 * (machine.a.1 as f64 / machine.b.1 as f64);
    let d = machine.prize.1 / machine.b.1 as f64;

    let a = (d - c) / (x - y);
    let b = a * x + c;

    let a_rounded = a.round();
    let b_rounded = b.round();

    let a_diff = (a_rounded - a).abs();
    let b_diff = (b_rounded - b).abs();

    if a.is_sign_positive()
        && b.is_sign_positive()
        && a_diff < FLOATING_POINT_BUFFER
        && b_diff < FLOATING_POINT_BUFFER
    {
        Some(a_rounded as u64 * 3 + b_rounded as u64)
    } else {
        None
    }
}
