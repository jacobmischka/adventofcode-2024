use std::io;

pub fn main() {
    let mut safe = 0;
    let mut safe_after_dampened = 0;

    for line in io::stdin().lines().filter_map(|line| {
        line.ok()
            .and_then(|line| if line.is_empty() { None } else { Some(line) })
    }) {
        let nums: Vec<_> = line
            .split_ascii_whitespace()
            .map(|n| n.parse::<i32>().unwrap())
            .collect();

        // These clones are gross but still instant and much clearer than alternatives
        if let Some(invalid_index) = check_report(&nums) {
            let mut new = nums.clone();
            new.remove(invalid_index);
            if check_report(&new).is_none() {
                safe_after_dampened += 1;
                continue;
            }

            if invalid_index < nums.len() - 1 {
                let mut new = nums.clone();
                new.remove(invalid_index + 1);
                if check_report(&new).is_none() {
                    safe_after_dampened += 1;
                    continue;
                }
            }

            if invalid_index > 0 {
                let mut new = nums.clone();
                new.remove(invalid_index - 1);
                if check_report(&new).is_none() {
                    safe_after_dampened += 1;
                    continue;
                }
            }
        } else {
            safe += 1;
            safe_after_dampened += 1;
        }
    }

    println!("Part 1: {safe}");
    println!("Part 2: {safe_after_dampened}");
}

fn check_report(nums: &[i32]) -> Option<usize> {
    let mut direction_sign: Option<i32> = None;

    for i in 0..(nums.len() - 1) {
        if !is_safe(nums[i] - nums[i + 1], direction_sign) {
            return Some(i);
        }
        if direction_sign.is_none() {
            direction_sign = Some(nums[i] - nums[i + 1]);
        }
    }

    return None;
}

fn is_safe(diff: i32, desired_sign: Option<i32>) -> bool {
    if let Some(desired_sign) = desired_sign {
        if desired_sign.signum() != diff.signum() {
            return false;
        }
    }

    return diff != 0 && diff.abs() <= 3;
}
