use std::fs;

const PUZZLE_INPUT_PATH: &str = "input/day_2";

pub fn run() {
    println!("day 2:");
    println!("part 1: {}", part_1());
}

fn part_1() -> i64 {
    let mut sum = 0;

    let input = fs::read_to_string(PUZZLE_INPUT_PATH).unwrap();

    for range in input.split(',') {
        let (start, end) = range.split_once('-').unwrap();
        let (start, end): (i64, i64) = (start.parse().unwrap(), end.parse().unwrap());

        for product_id in start..=end {
            let product_id_str = product_id.to_string();

            if product_id_str.len().is_multiple_of(2) {
                let (first, second) = product_id_str.split_at(product_id_str.len() / 2);

                if first == second {
                    sum += product_id;
                }
            }
        }
    }

    sum
}
