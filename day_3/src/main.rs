// first time using regex in rust. it was kind of painful but now i understand it
use regex::Regex;
use std::fs::read_to_string;

fn main() {
    let contents: String =
        read_to_string("./input/day_3").expect("Could not access contents of the file.");

    part_1(&contents);
    part_2(&contents);
}

fn part_1(contents: &str) -> () {
    let re: Regex = Regex::new(r"(mul\((?P<nums>\d+,\d+)\))").unwrap();

    let mut acc: u64 = 0;
    for capture in re.captures_iter(&contents) {
        acc += capture
            .name("nums")
            .map(|x| x.as_str())
            .unwrap()
            .split(",")
            .map(|x| x.parse::<u64>().unwrap())
            .product::<u64>()
    }

    println!("{}", acc)
}

fn part_2(contents: &str) -> () {
    let re: Regex = Regex::new(r"(mul\((?P<nums>\d+,\d+)\))|(do\(\))|(don't\(\))").unwrap();

    let mut acc: u64 = 0;
    let mut is_do: bool = true;
    for capture in re.captures_iter(&contents) {
        match &capture[0] {
            "do()" => {
                is_do = true;
                continue;
            }
            "don't()" => {
                is_do = false;
                continue;
            }
            &_ => {}
        }
        if is_do {
            acc += capture
                .name("nums")
                .map(|x| x.as_str())
                .unwrap()
                .split(",")
                .map(|x| x.parse::<u64>().unwrap())
                .product::<u64>()
        }
    }

    println!("{}", acc)
}
