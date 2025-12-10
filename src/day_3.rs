use std::{cmp, fs};

const PUZZLE_INPUT_PATH: &str = "input/day_3";

pub fn run() {
    println!("day 3:");

    let input = fs::read_to_string(PUZZLE_INPUT_PATH).unwrap();
    let banks = parse_banks(&input);

    println!("part 1: {}", part_1(&banks));
}

fn parse_banks(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|bank_str| bank_str.chars().filter_map(|c| c.to_digit(10)).collect())
        .collect()
}

fn part_1(banks: &[Vec<u32>]) -> u32 {
    let mut sum = 0;

    for bank in banks {
        let mut max = 0;
        let mut left = 0;
        let mut right = 1;

        while right < bank.len() {
            let left_digit = bank[left];
            let right_digit = bank[right];

            if left_digit < right_digit {
                left = right;
            }

            max = cmp::max(max, (left_digit * 10) + right_digit);
            right += 1;
        }

        sum += max;
    }

    sum
}
