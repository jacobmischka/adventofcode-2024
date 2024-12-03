use std::io;

use regex::Regex;

pub fn main() {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)").unwrap();

    let mut mul_enabled = true;
    let mut part1 = 0;
    let mut part2 = 0;
    for line in io::stdin().lines() {
        for c in re.captures_iter(&line.unwrap()) {
            let s = c.get(0).unwrap();
            match s.as_str() {
                "do()" => {
                    mul_enabled = true;
                }
                "don't()" => {
                    mul_enabled = false;
                }
                _ => {
                    let lhs: i32 = c.get(1).unwrap().as_str().parse().unwrap();
                    let rhs: i32 = c.get(2).unwrap().as_str().parse().unwrap();
                    let product = lhs * rhs;
                    part1 += product;
                    if mul_enabled {
                        part2 += product;
                    }
                }
            }
        }
    }
    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}
