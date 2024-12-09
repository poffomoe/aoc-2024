use std::fs::read_to_string;

fn main() {
    let contents: String =
        read_to_string("./input/day_5").expect("Could not access contents of the file.");

    part_1(&contents);
    part_2(&contents);
}

fn part_1(contents: &str) -> () {
    let lines: Vec<&str> = contents.trim().lines().collect();

    let rules: Vec<Vec<u32>>;
    let page_numbers: Vec<Vec<u32>>;
    {
        let parts: Vec<_> = lines.split(|line| line.is_empty()).collect();

        rules = parts[0]
            .iter()
            .map(|nums| {
                nums.split('|')
                    .map(|num| num.parse::<u32>().unwrap())
                    .collect()
            })
            .collect();

        page_numbers = parts[1]
            .iter()
            .map(|nums| {
                nums.split(',')
                    .map(|num| num.parse::<u32>().unwrap())
                    .collect()
            })
            .collect();
    }

    let mut res: u32 = 0;

    'outer: for pages in page_numbers {
        for idx in 0..(pages.len() - 1) {
            let num: u32 = pages[idx];
            for idx_2 in (idx + 1)..pages.len() {
                let num_2: u32 = pages[idx_2];
                if let None = rules.iter().find(|x| (x[0] == num) && (x[1] == num_2)) {
                    continue 'outer;
                }
            }
        }

        res += pages[pages.len() / 2];
    }

    println!("{}", res);
}

fn part_2(contents: &str) -> () {
    let lines: Vec<&str> = contents.trim().lines().collect();

    let rules: Vec<Vec<u32>>;
    let page_numbers: Vec<Vec<u32>>;
    {
        let parts: Vec<_> = lines.split(|line| line.is_empty()).collect();

        rules = parts[0]
            .iter()
            .map(|nums| {
                nums.split('|')
                    .map(|num| num.parse::<u32>().unwrap())
                    .collect()
            })
            .collect();

        page_numbers = parts[1]
            .iter()
            .map(|nums| {
                nums.split(',')
                    .map(|num| num.parse::<u32>().unwrap())
                    .collect()
            })
            .collect();
    }

    let mut res: u32 = 0;

    for pages in page_numbers {
        let mut incorrect: bool = false;

        'inner: for idx in 0..(pages.len() - 1) {
            let num: u32 = pages[idx];
            for idx_2 in (idx + 1)..pages.len() {
                let num_2: u32 = pages[idx_2];
                if let None = rules.iter().find(|x| (x[0] == num) && (x[1] == num_2)) {
                    incorrect = true;
                    break 'inner;
                }
            }
        }

        if incorrect {
            let mut pages: Vec<u32> = pages;
            pages.sort_by(|&a, &b| {
                if let Some(_) = rules.iter().find(|&num| *num == Vec::from([a, b])) {
                    std::cmp::Ordering::Less
                } else {
                    std::cmp::Ordering::Greater
                }
            });

            res += pages[pages.len() / 2];
        }
    }

    println!("{}", res);
}
