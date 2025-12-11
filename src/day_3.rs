use std::fs;

const PUZZLE_INPUT_PATH: &str = "input/day_3";

pub fn run() {
    println!("day 3:");

    let input = fs::read_to_string(PUZZLE_INPUT_PATH).unwrap();
    let banks = parse_banks(&input);

    println!("part 1: {}", part_1(&banks));
    println!("part 2: {}", part_2(&banks));
}

fn parse_banks(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| line.chars().filter_map(|c| c.to_digit(10)).collect())
        .collect()
}

fn part_1(banks: &Vec<Vec<u32>>) -> u64 {
    banks.iter().map(|bank| max_joltage(&bank, 2)).sum()
}

fn part_2(banks: &Vec<Vec<u32>>) -> u64 {
    banks.iter().map(|bank| max_joltage(&bank, 12)).sum()
}

fn max_joltage(batteries: &Vec<u32>, capacity: usize) -> u64 {
    let mut res = 0;
    let mut start = 0;

    for len in (0..capacity).rev() {
        let end = batteries.len() - len;
        let slice = &batteries[start..end];
        let (max_idx, max) = find_max_in_slice(slice);

        res = res * 10 + max as u64;
        start += max_idx + 1;
    }

    res
}

fn find_max_in_slice(slice: &[u32]) -> (usize, u32) {
    slice
        .iter()
        .copied()
        .enumerate()
        .rev()
        .max_by_key(|&(_, v)| v)
        .unwrap()
}
