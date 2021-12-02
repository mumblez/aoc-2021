#![allow(dead_code)]

use std::fs;

fn read_input() -> String {
    let contents = fs::read_to_string("input.txt").unwrap();
    contents
}

// day 1
// calculate final horizontal and depth (x, y) as x * y
fn position() -> u32 {
    let file = read_input();
    let (mut x, mut y) = (0_u32, 0_u32);

    for l in file.lines() {
        let tmp: Vec<&str> = l.split_whitespace().collect();
        let direction = tmp[0];
        let value: u32 = tmp[1].parse().unwrap();

        match direction {
            "forward" => x += value,
            "down" => y += value,
            _ => {
                if value > y {
                    y = 0;
                } else {
                    y -= value
                }
            }
        }
    }

    x * y
}

// day 2
// calculate final horizontal and depth (x, y) as x * y, also aim and calculate
// depth when moving forward (depth = forward * aim)
fn position2() -> u32 {
    let file = read_input();
    let (mut horizontal, mut depth, mut aim) = (0_u32, 0_u32, 0_u32);

    for l in file.lines() {
        let tmp: Vec<&str> = l.split_whitespace().collect();
        let direction = tmp[0];
        let value: u32 = tmp[1].parse().unwrap();

        match direction {
            "forward" => {
                // move forward horizontally (x)
                horizontal += value;
                // calculate depth (y)
                if aim != 0 {
                    depth += value * aim;
                }
            }
            "down" => aim += value,
            _ => {
                if value > depth {
                    aim = 0;
                } else {
                    aim -= value
                }
            }
        }
    }

    horizontal * depth
}
fn main() {
    println!("Part 1 answer: {}", position());
    println!("Part 2 answer: {}", position2());
}
