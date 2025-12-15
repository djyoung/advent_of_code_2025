use std::{cmp, fs};

#[derive(Clone)]
struct Range {
    start: usize,
    end: usize,
}

impl Range {
    fn parse(range_str: &str) -> Self {
        let (start_str, end_str) = range_str.split_once('-').unwrap();

        Range {
            start: start_str.parse().unwrap(),
            end: end_str.parse().unwrap(),
        }
    }

    fn contains(&self, value: usize) -> bool {
        self.start <= value && value <= self.end
    }
}

const PUZZLE_INPUT_PATH: &str = "input/day_5";

pub fn run() {
    println!("day 5:");

    let input = fs::read_to_string(PUZZLE_INPUT_PATH).unwrap();
    let (ranges, values) = parse_input(&input);

    println!("part 1: {}", part_1(&ranges, &values));
    println!("part 2: {}", part_2(&ranges));
}

fn parse_input(input: &str) -> (Vec<Range>, Vec<usize>) {
    let sections: Vec<&str> = input.split("\n\n").collect();

    let ranges = sections[0].lines().map(Range::parse).collect();

    let values = sections[1]
        .lines()
        .map(|s| s.parse().unwrap())
        .collect::<Vec<_>>();

    (ranges, values)
}

fn part_1(ranges: &[Range], values: &[usize]) -> usize {
    values
        .iter()
        .copied()
        .filter(|&value| ranges.iter().any(|range| range.contains(value)))
        .count()
}

fn merge_ranges(ranges: &[Range]) -> Vec<Range> {
    let mut sorted_ranges = ranges.to_vec();
    sorted_ranges.sort_by_key(|r| r.start);

    let mut merged: Vec<Range> = Vec::new();

    for range in sorted_ranges {
        if let Some(last) = merged.last_mut() {
            if range.start <= last.end {
                last.end = cmp::max(last.end, range.end);
            } else {
                merged.push(range);
            }
        } else {
            merged.push(range);
        }
    }
    merged
}

fn part_2(ranges: &[Range]) -> usize {
    merge_ranges(ranges)
        .iter()
        .map(|range| range.end - range.start + 1)
        .sum()
}
