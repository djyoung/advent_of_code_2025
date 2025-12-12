use std::fs;

const PUZZLE_INPUT_PATH: &str = "input/day_4";

pub fn run() {
    println!("day 4:");

    let input = fs::read_to_string(PUZZLE_INPUT_PATH).unwrap();
    let layout = parse_layout(&input);

    println!("part 1: {}", part_1(&layout));
}

fn parse_layout(input: &str) -> Vec<Vec<usize>> {
    let mut layout = Vec::new();

    for line in input.lines() {
        let mut rolls = Vec::new();

        for char in line.chars() {
            if char == '.' {
                rolls.push(0);
            } else {
                rolls.push(1);
            }
        }

        layout.push(rolls);
    }

    layout
}

fn part_1(layout: &Vec<Vec<usize>>) -> usize {
    let mut res = 0;

    for i in 0..layout.len() {
        for j in 0..layout[i].len() {
            if layout[i][j] == 0 {
                continue;
            }

            if count_neighbors(layout, i, j) < 4 {
                res += 1;
            }
        }
    }

    res
}

fn count_neighbors(layout: &Vec<Vec<usize>>, i: usize, j: usize) -> usize {
    let mut count = 0;

    let directions = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    for (di, dj) in directions {
        let di = i as isize + di;
        let dj = j as isize + dj;

        if di >= 0 && di < layout.len() as isize && dj >= 0 && dj < layout[i].len() as isize {
            count += layout[di as usize][dj as usize]
        }
    }

    count
}
