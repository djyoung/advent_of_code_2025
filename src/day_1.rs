use std::fs;

const PUZZLE_INPUT_PATH: &str = "input/day_1";

pub fn run() {
    println!("day 1:");
    println!("part 1: {}", part_1());
    println!("part 2: {}", part_2());
}

fn part_1() -> i32 {
    let mut position = 50;
    let mut password = 0;

    let input = fs::read_to_string(PUZZLE_INPUT_PATH).unwrap();

    for line in input.lines() {
        let direction = &line[0..1];
        let clicks = &line[1..].parse().unwrap();

        if direction == "L" {
            position = (position - clicks + 100) % 100;
        } else {
            position = (position + clicks) % 100;
        }

        if position == 0 {
            password += 1;
        }
    }

    password
}

fn part_2() -> i32 {
    let mut position = 50;
    let mut password = 0;

    let input = fs::read_to_string(PUZZLE_INPUT_PATH).unwrap();

    for line in input.lines() {
        let direction = &line[0..1];
        let clicks = line[1..].parse().unwrap();

        for _ in 0..clicks {
            if direction == "L" {
                position -= 1;
            } else {
                position += 1;
            }

            position = (position + 100) % 100;

            if position == 0 {
                password += 1;
            }
        }
    }

    password
}
