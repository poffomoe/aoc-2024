use std::fs::read_to_string;

fn main() {
    let contents: String =
        read_to_string("./input/day_4").expect("Could not access contents of the file.");

    part_1(&contents);
    part_2(&contents);
}

fn part_1(contents: &str) -> () {
    const DIRECTIONS: [(i32, i32); 8] = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
        (1, 0),
        (1, -1),
        (0, -1),
    ];

    const ORDER: [char; 3] = ['M', 'A', 'S'];

    let matrix: Vec<Vec<char>> = contents
        .trim()
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut acc: i32 = 0;
    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            let letter: char = matrix[i][j];
            if letter == 'X' {
                let mut available_dirs: Vec<usize> = vec![];

                if (i >= 3) && (j >= 3) {
                    available_dirs.push(0);
                }

                if i >= 3 && (j <= matrix[i].len() - 4) {
                    available_dirs.push(1);
                }

                if (i >= 3) && (j <= matrix[i].len() - 4) {
                    available_dirs.push(2);
                }

                if j <= matrix[i].len() - 4 {
                    available_dirs.push(3);
                }

                if (j <= matrix[i].len() - 4) && (i <= matrix.len() - 4) {
                    available_dirs.push(4);
                }

                if i <= matrix.len() - 4 {
                    available_dirs.push(5);
                }

                if (j >= 3) && (i <= matrix.len() - 4) {
                    available_dirs.push(6);
                }

                if j >= 3 {
                    available_dirs.push(7);
                }

                for dir in available_dirs {
                    let mut correct: bool = true;
                    'innermost: for a in 1..=3 {
                        if !(matrix[i + (DIRECTIONS[dir].0 as usize * a)]
                            [j + (DIRECTIONS[dir].1 as usize * a)]
                            == ORDER[a - 1])
                        {
                            correct = false;
                            break 'innermost;
                        }
                    }
                    if correct {
                        acc += 1
                    }
                }
            }
        }
    }

    println!("{}", acc);
}

fn part_2(contents: &str) -> () {
    let matrix: Vec<Vec<char>> = contents
        .trim()
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut acc: i32 = 0;
    for i in 1..(matrix.len() - 1) {
        for j in 1..(matrix[i].len() - 1) {
            let letter: char = matrix[i][j];
            if letter == 'A' {
                let corners: [char; 4] = [
                    matrix[i - 1][j - 1],
                    matrix[i - 1][j + 1],
                    matrix[i + 1][j + 1],
                    matrix[i + 1][j - 1],
                ];
                match corners {
                    ['M', 'M', 'S', 'S']
                    | ['S', 'S', 'M', 'M']
                    | ['S', 'M', 'M', 'S']
                    | ['M', 'S', 'S', 'M'] => acc += 1,
                    _ => (),
                }
            }
        }
    }
    println!("{}", acc)
}
