use std::io;

pub fn part_1() {
    let mut position = 50;
    let mut password = 0;

    for line in io::stdin().lines() {
        let line = line.unwrap();
        let direction = line.chars().next().unwrap();
        let rotations: i32 = line[1..].parse().unwrap();

        if direction == 'L' {
            position = (position - rotations + 100) % 100;
        } else {
            position = (position + rotations) % 100;
        }

        println!("The dial is rotated {line} to point at {position}.");

        if position == 0 {
            password += 1;
        }
    }

    println!("The password is {password}");
}
