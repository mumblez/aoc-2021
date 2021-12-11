use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    // generate 1000 * 1000 grid (array)
    let mut grid = [[0_u16; 1000]; 1000];

    // parse input.txt
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        // x1,y1 -> x2,y2
        let l: Vec<String> = line.unwrap().split(' ').map(|x| x.into()).collect();
        let start: Vec<u16> = l[0].split(',').map(|x| x.parse().unwrap()).collect();
        let end: Vec<u16> = l[2].split(',').map(|x| x.parse().unwrap()).collect();
        println!("Start: {:?}, End: {:?}", start, end);

        // day-2, also detect diagonal, distances between x1, x2 and y1, y2 abs should be the same
        let x_diff = start[0] as i32 - end[0] as i32;
        let y_diff = start[1] as i32 - end[1] as i32;
        let diagonal = x_diff.abs() == y_diff.abs();

        // process diagonal
        if diagonal {
            let (mut x, mut y) = (start[0], start[1]);
            loop {
                grid[y as usize][x as usize] += 1;

                // we've reached the end
                if x == end[0] && y == end[1] {
                    break;
                }

                // move closer to end
                if x < end[0] {
                    x += 1;
                } else {
                    x -= 1;
                }
                if y < end[1] {
                    y += 1;
                } else {
                    y -= 1;
                }
            }

        // only process where either x1 == x2 or y1 == y2 (straight lines)
        } else if start[0] == end[0] || start[1] == end[1] {
            let horizontal = start[1] == end[1];
            let vertical = start[0] == end[0];

            // skip line if neither horizontal or vertical match
            if !vertical && !horizontal {
                continue;
            }

            if vertical {
                // find lowest and highest
                let (mut low, high): (u16, u16) = if start[1] > end[1] {
                    (end[1], start[1])
                } else {
                    (start[1], end[1])
                };
                while low <= high {
                    grid[low as usize][start[0] as usize] += 1;
                    low += 1;
                }
            } else {
                // find lowest and highest
                let (mut low, high): (u16, u16) = if start[0] > end[0] {
                    (end[0], start[0])
                } else {
                    (start[0], end[0])
                };
                while low <= high {
                    grid[start[1] as usize][low as usize] += 1;
                    low += 1;
                }
            }
        }
    }

    // draw lines on grid (increment point in grid)
    // count number of elements on grid greater than 1
    println!(
        "answer: {}",
        grid.iter().flat_map(|x| x).filter(|x| *x > &1).count()
    );
}
