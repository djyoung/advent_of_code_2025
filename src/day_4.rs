use std::fs;

const PUZZLE_INPUT_PATH: &str = "input/day_4";

pub fn run() {
    println!("day 4:");

    let input = fs::read_to_string(PUZZLE_INPUT_PATH).unwrap();
    let layout = parse_layout(&input);

    println!("part 1: {}", part_1(&layout));
    println!("part 2: {}", part_2(&layout));
}

fn parse_layout(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|line| line.chars().map(|c| if c == '.' { 0 } else { 1 }).collect())
        .collect()
}

fn part_1(layout: &[Vec<usize>]) -> usize {
    layout
        .iter()
        .enumerate()
        .map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter(|&(j, &cell)| cell != 0 && count_neighbors(layout, i, j) < 4)
                .count()
        })
        .sum()
}

fn part_2(layout: &[Vec<usize>]) -> usize {
    let mut layout = layout.to_vec();
    let mut res = 0;

    loop {
        let mut changed = false;

        for i in 0..layout.len() {
            for j in 0..layout[i].len() {
                if layout[i][j] != 0 && count_neighbors(&layout, i, j) < 4 {
                    res += 1;
                    layout[i][j] = 0;
                    changed = true;
                }
            }
        }

        if !changed {
            break;
        }
    }

    res
}

fn count_neighbors(layout: &[Vec<usize>], i: usize, j: usize) -> usize {
    const DIRECTIONS: [(isize, isize); 8] = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    DIRECTIONS
        .iter()
        .filter_map(|&(di, dj)| {
            layout
                .get(i.checked_add_signed(di)?)?
                .get(j.checked_add_signed(dj)?)
        })
        .sum()
}
