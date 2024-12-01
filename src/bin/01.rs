use std::{collections::HashMap, io};

pub fn main() {
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();
    for (l, r) in io::stdin().lines().filter_map(|line| {
        let line = line.unwrap();
        let line = line.trim();
        if line.is_empty() {
            None
        } else {
            let mut pieces = line
                .split_ascii_whitespace()
                .map(|num| num.parse::<i32>().unwrap());
            Some((pieces.next().unwrap(), pieces.next().unwrap()))
        }
    }) {
        left.push(l);
        right.push(r);
    }

    left.sort();
    right.sort();

    let mut counts: HashMap<i32, u32> = HashMap::new();
    for num in &right {
        *counts.entry(*num).or_default() += 1;
    }

    let mut diff_sum = 0;
    let mut mult_sum = 0u64;
    for (l, r) in left.iter().zip(&right) {
        diff_sum += l.abs_diff(*r);
        mult_sum += *l as u64 * counts.get(l).copied().unwrap_or_default() as u64;
    }

    println!("Part 1: {diff_sum}");
    println!("Part 2: {mult_sum}");
}
