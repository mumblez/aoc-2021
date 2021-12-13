use std::fs::File;
use std::io::{prelude::*, BufReader};

fn traverse(traversed: &mut [[bool; 100]; 100], grid: &Vec<Vec<u8>>, x: usize, y: usize) -> u32 {
    let mut tally = 0_u32;

    // check current location is valid
    if traversed[y][x] {
        return 0;
    } else {
        traversed[y][x] = true;
    }

    if grid[y][x] != 9 {
        tally += 1;
    } else {
        return 0;
    }

    // check adjacent
    let top = y == 0;
    let bottom = y == (grid.len() - 1);
    let left = x == 0;
    let right = x == (grid[y].len() - 1);

    if !top {
        tally += traverse(&mut *traversed, &grid, x, y - 1);
    }
    if !bottom {
        tally += traverse(&mut *traversed, &grid, x, y + 1);
    }
    if !left {
        tally += traverse(&mut *traversed, &grid, x - 1, y);
    }
    if !right {
        tally += traverse(&mut *traversed, &grid, x + 1, y);
    }

    tally
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut grid: Vec<Vec<u8>> = Vec::with_capacity(100);

    for line in reader.lines() {
        let mut row: Vec<u8> = Vec::with_capacity(100);

        for c in line.unwrap().trim().chars() {
            row.push(c.to_digit(10).unwrap() as u8);
        }

        grid.push(row);
    }

    // part 2
    // find 3 largest basins, all surrounding numbers until we hit a wall (9 or edge)
    // recursively traverse until edge or 9 detected
    // check adjacent values before traversing
    // also check if location has already been traversed

    let mut traversed = [[false; 100]; 100];
    // let mut basins: HashSet<u32> = HashSet::new();
    let mut basins: Vec<u32> = Vec::new();

    // find low points
    // let mut lowest: Vec<u8> = Vec::new();
    // let mut lowest: Vec<(usize, usize)> = Vec::new();
    for (y, row) in grid.iter().enumerate() {
        // check if top or bottom
        let top = y == 0;
        let bottom = y == (grid.len() - 1);

        for (x, col) in row.iter().enumerate() {
            // check if left or right edge
            let left = x == 0;
            let right = x == (row.len() - 1);

            // check if adjacent values are smaller, skip if greater
            if !left && col >= &grid[y][x - 1] {
                continue;
            }

            if !top && col >= &grid[y - 1][x] {
                continue;
            }

            if !right && col >= &grid[y][x + 1] {
                continue;
            }

            if !bottom && col >= &grid[y + 1][x] {
                continue;
            }

            // if we reach here than it means current number
            // is lower than all adjacents
            // lowest.push(*col);
            // lowest.push((y, x));
            let tally = traverse(&mut traversed, &grid, x, y);
            if !basins.contains(&tally) {
                basins.push(tally);
            }
        }
    }

    basins.sort();
    basins.drain(..(basins.len() - 3));
    let total = basins.iter().fold(1, |acc, x| acc * x);

    println!("Answer: {}", total);

    // part 1
    // println!(
    //     "tally: {:?}",
    //     lowest
    //         .iter()
    //         .fold(0, |acc, x| *x as u32 + 1_u32 + acc as u32)
    // );
}
