use std::fs;

type Range = (usize, usize);

const PUZZLE_INPUT_PATH: &str = "input/day_5";

pub fn run() {
    println!("day 5:");

    let input = fs::read_to_string(PUZZLE_INPUT_PATH).unwrap();
    let (fresh_ingredients, required_ingredients) = parse_ingredients(&input);

    println!(
        "part 1: {}",
        part_1(&fresh_ingredients, &required_ingredients)
    );
}

fn parse_ingredients(input: &str) -> (Vec<Range>, Vec<usize>) {
    let lines: Vec<&str> = input.lines().collect();
    let empty_index = lines.iter().position(|&line| line.is_empty()).unwrap();

    let fresh_ingredients = lines[..empty_index]
        .iter()
        .map(|range_str| parse_range(range_str))
        .collect();

    let required_ingredients = lines[empty_index + 1..]
        .iter()
        .map(|s| s.parse().unwrap())
        .collect();

    (fresh_ingredients, required_ingredients)
}

fn parse_range(range_str: &str) -> Range {
    let (start, end) = range_str.split_once('-').unwrap();
    (start.parse().unwrap(), end.parse().unwrap())
}

fn part_1(fresh_ingredients: &[Range], required_ingredients: &[usize]) -> usize {
    required_ingredients
        .iter()
        .filter(|&&ingredient| {
            fresh_ingredients
                .iter()
                .any(|&(start, end)| start <= ingredient && ingredient <= end)
        })
        .count()
}
