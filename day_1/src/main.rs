use std::fs::read_to_string;

fn main() {
    let contents: String =
        read_to_string("./input/day_1").expect("Could not access contents of the file.");

    part_1(&contents);
    part_2(&contents);
}

fn part_1(contents: &str) -> () {
    let lines: Vec<&str> = contents.trim().lines().collect();

    let mut vec_1: Vec<i32> = vec![];
    let mut vec_2: Vec<i32> = vec![];
    for line in lines {
        let nums: Vec<i32> = line
            .split_whitespace()
            .map(|num| num.parse().unwrap())
            .collect();
        vec_1.push(nums[0]);
        vec_2.push(nums[1]);
    }
    vec_1.sort();
    vec_2.sort();

    let mut acc: i32 = 0;
    for index in 0..vec_1.len() {
        acc += (vec_1[index] - vec_2[index]).abs();
    }
    println!("Part 1: {}", acc)
}

fn part_2(contents: &str) -> () {
    let lines: Vec<&str> = contents.trim().lines().collect();

    let mut vec_1: Vec<u32> = vec![];
    let mut vec_2: Vec<u32> = vec![];
    for line in lines {
        let nums: Vec<u32> = line
            .split_whitespace()
            .map(|num| num.parse().unwrap())
            .collect();
        vec_1.push(nums[0]);
        vec_2.push(nums[1]);
    }

    let mut acc: u32 = 0;
    for num in vec_1 {
        acc += num * vec_2.iter().filter(|&x| x == &num).count() as u32;
    }
    println!("Part 2: {}", acc)
}
