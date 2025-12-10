use std::fs;

const PUZZLE_INPUT_PATH: &str = "input/day_2";

type Range = (i64, i64);

pub fn run() {
    println!("day 2:");

    let input = fs::read_to_string(PUZZLE_INPUT_PATH).unwrap();
    let ranges = parse_ranges(&input);

    println!("part 1: {}", part_1(&ranges));
    println!("part 2: {}", part_2(&ranges));
}

fn parse_ranges(input: &str) -> Vec<Range> {
    input
        .split(',')
        .map(|range_str| {
            let (start, end) = range_str.split_once('-').unwrap();
            (start.parse().unwrap(), end.parse().unwrap())
        })
        .collect()
}

fn part_1(ranges: &[Range]) -> i64 {
    let mut sum = 0;

    for &(start, end) in ranges {
        for id in start..=end {
            if is_symmetric_id(id) {
                sum += id;
            }
        }
    }

    sum
}

fn is_symmetric_id(id: i64) -> bool {
    let id_str = id.to_string();
    let len = id_str.len();

    if !len.is_multiple_of(2) {
        return false;
    }

    let (first, second) = id_str.split_at(len / 2);

    first == second
}

fn part_2(ranges: &[Range]) -> i64 {
    let mut sum = 0;

    for &(start, end) in ranges {
        for id in start..=end {
            if has_repeating_pattern(id) {
                sum += id;
            }
        }
    }

    sum
}

fn has_repeating_pattern(id: i64) -> bool {
    let id_str = id.to_string();
    let len = id_str.len();

    for i in 0..len / 2 {
        let prefix = &id_str[0..=i];
        let prefix_len = prefix.len();

        let mut index = i + 1;
        let mut found_repeating_pattern = true;

        while index + prefix_len <= len {
            let window = &id_str[index..index + prefix_len];
            let window_len = window.len();

            if window_len.is_multiple_of(len) || prefix != window {
                found_repeating_pattern = false;
                break;
            }

            index += prefix_len;
        }

        if found_repeating_pattern && len.is_multiple_of(prefix_len) {
            return true;
        }
    }

    false
}
