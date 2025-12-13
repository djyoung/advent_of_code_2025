use std::fs;

const PUZZLE_INPUT_PATH: &str = "input/day_5";

pub fn run() {
    println!("day 5:");

    let input = fs::read_to_string(PUZZLE_INPUT_PATH).unwrap();

    println!("part 1: {}", part_1(&input));
}

fn part_1(input: &str) -> usize {
    let mut result = 0;

    let lines: Vec<&str> = input.lines().collect();

    let empty_index = lines.iter().position(|&line| line.is_empty()).unwrap();

    let fresh_ingredients = lines[..empty_index].to_vec();
    let required_ingredients = lines[empty_index + 1..].to_vec();

    println!("fresh_ingredients: {:?}", fresh_ingredients);
    println!("required_ingredients: {:?}", required_ingredients);

    result
}
