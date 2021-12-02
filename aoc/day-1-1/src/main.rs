// read input.txt and count number of times a value (line) increased from previous line
//
#![allow(dead_code)]

use std::fs;

fn read_input() -> String {
    let file = fs::read_to_string("input-1.txt").unwrap();
    file
}

// day 1
fn count_increasing() -> u32 {
    let file = read_input();
    let mut prevn = 0_u32;
    let mut total = 0_u32;

    for l in file.lines() {
        let n: u32 = l.parse().unwrap();
        if prevn != 0 && n > prevn {
            total += 1;
        }
        prevn = n;
    }
    total
}

// day 2
fn count_sliding_window() -> u32 {
    let file = read_input();
    let lines: Vec<u32> = file.lines().map(|x| x.parse::<u32>().unwrap()).collect();

    let mut total = 0_u32;

    let mut previous = 0_u32;

    for l in lines.windows(3) {
        // sum 3 values at a time before comparing
        let sum = l.iter().fold(0, |acc, x| acc + x);

        if previous != 0 && sum > previous {
            total += 1;
        }

        previous = sum;
    }
    total
}

fn main() {
    // day 1
    println!("Total: {}", count_increasing());
    // day 2
    println!("Total: {}", count_sliding_window());
}
