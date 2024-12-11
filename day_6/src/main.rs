use std::fs::read_to_string;

fn main() {
    let contents: String =
        read_to_string("./input/day_6").expect("Could not access contents of the file.");

    let marked: Vec<(usize, usize)> = part_1(&contents);
    part_2(&contents, marked);
}

fn part_1(contents: &str) -> Vec<(usize, usize)> {
    const DIRECTIONS: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

    let mut matrix: Vec<Vec<char>> = contents
        .trim()
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let (mut pos_i, mut pos_j): (usize, usize) = (0, 0);

    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            if matrix[i][j] == '^' {
                (pos_i, pos_j) = (i, j)
            }
        }
    }

    let mut current_dir: usize = 0;
    let mut marked: Vec<(usize, usize)> = Vec::new();
    loop {
        let dir = DIRECTIONS[current_dir];

        let next_pos_i: usize = ((pos_i as i32) + dir.0) as usize;
        let next_pos_j: usize = ((pos_j as i32) + dir.1) as usize;

        if (next_pos_i >= matrix.len()) || (next_pos_j >= matrix[0].len()) {
            matrix[pos_i][pos_j] = 'X';
            break;
        }

        if matrix[next_pos_i][next_pos_j] == '#' {
            current_dir = (current_dir + 1) % 4;
            continue;
        }

        matrix[pos_i][pos_j] = 'X';
        pos_i = next_pos_i;
        pos_j = next_pos_j;
    }

    let mut res: usize = 0;
    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            if matrix[i][j] == 'X' {
                marked.push((i, j));
                res += 1
            }
        }
    }

    println!("{}", res);
    marked
}

fn part_2(contents: &str, marked: Vec<(usize, usize)>) -> () {
    const DIRECTIONS: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

    let mut matrix: Vec<Vec<char>> = contents
        .trim()
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let (mut pos_i, mut pos_j): (usize, usize) = (0, 0);

    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            if matrix[i][j] == '^' {
                (pos_i, pos_j) = (i, j)
            }
        }
    }

    let start_pos: (usize, usize) = (pos_i, pos_j);

    let matrixdef = matrix.clone();
    let mut res: i32 = 0;
    for obstacle_pos in marked {
        let mut current_dir: usize = 0;
        matrix = matrixdef.clone();
        (pos_i, pos_j) = start_pos;
        let mut visited: Vec<(usize, (usize, usize))> = Vec::new();
        matrix[obstacle_pos.0][obstacle_pos.1] = '#';

        'inner: loop {
            let dir: (i32, i32) = DIRECTIONS[current_dir];

            let next_pos_i: usize = ((pos_i as i32) + dir.0) as usize;
            let next_pos_j: usize = ((pos_j as i32) + dir.1) as usize;

            if (next_pos_i >= matrix.len()) || (next_pos_j >= matrix[0].len()) {
                break 'inner;
            }

            if matrix[next_pos_i][next_pos_j] == '#' {
                if let Some(_) = visited
                    .iter()
                    .find(|&&x| x == (current_dir, (next_pos_i, next_pos_j)))
                {
                    res += 1;
                    break 'inner;
                }
                visited.push((current_dir, (next_pos_i, next_pos_j)));

                current_dir = (current_dir + 1) % 4;
                continue 'inner;
            }

            pos_i = next_pos_i;
            pos_j = next_pos_j;
        }
    }

    println!("{}", res);
}
