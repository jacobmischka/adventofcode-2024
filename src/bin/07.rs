use std::io;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Operation {
    Add,
    Mul,
    Concat,
}

impl Operation {
    fn perform(&self, lhs: u64, rhs: u64) -> u64 {
        match self {
            Operation::Add => lhs + rhs,
            Operation::Mul => lhs * rhs,
            Operation::Concat => lhs * 10u64.pow(rhs.to_string().len() as u32) + rhs,
        }
    }
}

pub fn main() {
    let (part1, part2) = io::stdin()
        .lines()
        .fold((0, 0), |(mut part1, mut part2), line| {
            let line = line.unwrap();
            let s = line.trim();

            if s.is_empty() {
                return (part1, part2);
            }

            let mut sides = s.split(": ");
            let test_value: u64 = sides.next().unwrap().parse().unwrap();
            let nums: Vec<u64> = sides
                .next()
                .unwrap()
                .split_ascii_whitespace()
                .map(|v| v.parse().unwrap())
                .collect();

            if can_make_test_value(test_value, &nums, false) {
                part1 += test_value;
            }

            if can_make_test_value(test_value, &nums, true) {
                part2 += test_value;
            }

            (part1, part2)
        });

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}

fn can_make_test_value(test_value: u64, nums: &Vec<u64>, include_concat: bool) -> bool {
    let mut ops = vec![Operation::Add; nums.len() - 1];
    loop {
        let mut result = nums[0];
        for i in 1..nums.len() {
            result = ops[i - 1].perform(result, nums[i]);
        }

        if result == test_value {
            return true;
        }

        if inc_vec(&mut ops, include_concat).is_none() {
            return false;
        }
    }
}

fn inc_vec(v: &mut Vec<Operation>, include_concat: bool) -> Option<()> {
    for i in 0..v.len() {
        if v[i] == Operation::Add {
            v[i] = Operation::Mul;
            return Some(());
        } else if include_concat && v[i] == Operation::Mul {
            v[i] = Operation::Concat;
            return Some(());
        } else {
            v[i] = Operation::Add;
        }
    }

    None
}
