use std::fs;

const PUZZLE_INPUT_PATH: &str = "input/day_1";
const STARTING_POSITION: usize = 50;
const DIAL_SIZE: usize = 100;

enum Direction {
    Left,
    Right,
}

struct Rotation {
    direction: Direction,
    clicks: usize,
}

pub fn run() {
    println!("day 1:");

    let input = fs::read_to_string(PUZZLE_INPUT_PATH).unwrap();
    let rotations = parse_rotations(&input);

    println!("part 1: {}", part_1(&rotations));
    println!("part 2: {}", part_2(&rotations));
}

fn parse_rotations(input: &str) -> Vec<Rotation> {
    input
        .lines()
        .map(|line| {
            let (direction_str, clicks_str) = line.split_at(1);

            let direction = match direction_str {
                "L" => Direction::Left,
                _ => Direction::Right,
            };

            let clicks = clicks_str.parse().unwrap();

            Rotation { direction, clicks }
        })
        .collect()
}

fn part_1(rotations: &[Rotation]) -> usize {
    let mut position = STARTING_POSITION;
    let mut count = 0;

    for rotation in rotations {
        position = match rotation.direction {
            Direction::Left => (position + DIAL_SIZE - rotation.clicks) % DIAL_SIZE,
            Direction::Right => (position + rotation.clicks) % DIAL_SIZE,
        };

        if position == 0 {
            count += 1;
        }
    }

    count
}

fn part_2(rotations: &[Rotation]) -> usize {
    let mut position = STARTING_POSITION;
    let mut count = 0;

    for rotation in rotations {
        for _ in 0..rotation.clicks {
            position = match rotation.direction {
                Direction::Left => (position + DIAL_SIZE - 1) % DIAL_SIZE,
                Direction::Right => (position + 1) % DIAL_SIZE,
            };

            if position == 0 {
                count += 1;
            }
        }
    }

    count
}
