use std::io;

pub fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    let mut rocks: Vec<u64> = s
        .split_ascii_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    for i in 0..75 {
        println!("{i} {}", rocks.len());
        if i == 25 {
            println!("Part 1: {}", rocks.len());
        }
        blink(&mut rocks);
    }

    println!("Part 2: {}", rocks.len());
}

fn blink(rocks: &mut Vec<u64>) {
    let len = rocks.len();
    for i in 0..len {
        if rocks[i] == 0 {
            rocks[i] = 1;
        } else {
            let s = rocks[i].to_string();
            if s.len() % 2 == 0 {
                let mid = s.len() / 2;
                let lhs = &s[0..mid];
                let rhs = &s[mid..s.len()];
                rocks[i] = lhs.parse().unwrap();
                rocks.push(rhs.parse().unwrap())
            } else {
                rocks[i] *= 2024;
            }
        }
    }
}
