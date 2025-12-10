use std::fs;

const PUZZLE_INPUT_PATH: &str = "input/day_2";

type Range = (usize, usize);

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

fn part_1(ranges: &[Range]) -> usize {
    let mut sum = 0;

    for &(start, end) in ranges {
        for id in start..=end {
            if is_symmetric(id) {
                sum += id;
            }
        }
    }

    sum
}

fn is_symmetric(id: usize) -> bool {
    let id_str = id.to_string();
    let len = id_str.len();

    if !len.is_multiple_of(2) {
        return false;
    }

    let (first, second) = id_str.split_at(len / 2);

    first == second
}

fn part_2(ranges: &[Range]) -> usize {
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

fn has_repeating_pattern(id: usize) -> bool {
    let id_str = id.to_string();
    let len = id_str.len();

    for i in 1..=len / 2 {
        if len % i != 0 {
            continue;
        }

        let pattern = &id_str[..i];
        let mut matched = true;

        for j in (i..len).step_by(i) {
            let window = &id_str[j..j + i];

            if window != pattern {
                matched = false;
                break;
            }
        }

        if matched {
            return true;
        }
    }

    false
}
