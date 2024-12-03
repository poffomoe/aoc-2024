use std::cmp::Ordering;
use std::fs::read_to_string;

fn main() {
    let contents: String =
        read_to_string("./input/day_2").expect("Could not access contents of the file.");

    part_1(&contents);
    part_2(&contents);
}

fn part_1(contents: &str) -> () {
    let lines: Vec<&str> = contents.trim().lines().collect();

    let mut acc: i32 = 0;
    for line in lines {
        let nums: Vec<i32> = line
            .split_whitespace()
            .map(|num| num.parse().unwrap())
            .collect();

        if !is_safe(&nums) {
            continue;
        }
        acc += 1
    }

    println!("{}", acc);
}

fn part_2(contents: &str) {
    let lines: Vec<&str> = contents.trim().lines().collect();

    let mut acc: i32 = 0;
    for line in lines {
        let nums: Vec<i32> = line
            .split_whitespace()
            .map(|num| num.parse().unwrap())
            .collect();

        if !is_safe_with_dampener(&nums) {
            continue;
        }
        acc += 1
    }

    println!("{}", acc);
}

fn is_safe(nums: &Vec<i32>) -> bool {
    let order: Ordering = nums[0].cmp(&nums[1]);
    if order == Ordering::Equal {
        return false;
    }

    for idx in 0..(nums.len() - 1) {
        let inner_order: Ordering = nums[idx].cmp(&nums[idx + 1]);
        if (nums[idx] - nums[idx + 1]).abs() > 3 || inner_order != order {
            return false;
        }
    }

    true
}

fn is_safe_with_dampener(nums: &Vec<i32>) -> bool {
    if is_safe(&nums) {
        return true;
    }

    for idx in 0..nums.len() {
        let with_removed: Vec<i32> = [&nums[..idx], &nums[(idx + 1)..]].concat();
        if is_safe(&with_removed) {
            return true;
        }
    }

    false
}
