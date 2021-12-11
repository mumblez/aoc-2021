use std::fs::File;
use std::io::{prelude::*, BufReader};

#[derive(Debug)]
struct Number {
    number: u8,
    marked: bool,
}

fn process_grid(grid: &[Number], number: u8) {
    let total_unmarked = grid
        .iter()
        .filter(|x| !x.marked)
        .map(|x| x.number)
        .fold(0, |acc, x| acc + x as u32);

    println!(
        "Answer, number({}) * total_unmarked({}): {}",
        number,
        total_unmarked,
        total_unmarked as u32 * number as u32
    );
}

fn tally(grid: &[Number], row: bool, num: u8) -> bool {
    let mut tally = 0_u8;

    // column check
    for x in 0..5 {
        for y in 0..5 {
            #[allow(unused_assignments)]
            let mut idx = 0;

            if row {
                idx = y + (x * 5);
            } else {
                idx = x + (y * 5);
            }

            if grid[idx].marked {
                tally += 1;
            } else {
                tally = 0;
                break;
            }
        }

        if tally == 5 {
            process_grid(grid, num);
            return true;
        }
    }
    return false;
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut numbers: Vec<u8> = Vec::new();
    let mut grid: Vec<Number> = Vec::new();

    for line in reader.lines() {
        let l = line.unwrap();

        // skip blank line
        if l.len() == 0 {
            continue;
        }

        // store first line and grids
        if l.contains(',') {
            numbers = l.split(',').map(|x| x.parse::<u8>().unwrap()).collect();
            continue;
        } else {
            l.split_whitespace()
                .map(|x| x.parse::<u8>().unwrap())
                .for_each(|x| {
                    grid.push(Number {
                        number: x,
                        marked: false,
                    })
                });
        }
    }

    // day 1 - first winning grid
    // mark and check for bingo
    // let mut num_count = 0_u32;
    //
    // 'outer: for num in numbers {
    //     for n in grid.iter_mut() {
    //         if n.number == num {
    //             n.marked = true;
    //         }
    //     }
    //
    //     num_count += 1;
    //
    //     // check after 5 nums
    //     if num_count >= 5 {
    //         // check per grid (25 elements)
    //         for grid_all in grid.chunks(25) {
    //             // column check
    //             if tally(grid_all, false, num) {
    //                 break 'outer;
    //             };
    //
    //             // row check
    //             if tally(grid_all, true, num) {
    //                 break 'outer;
    //             };
    //         }
    //     }
    // }

    // day 2 - last winning grid
    let mut num_count = 0_u32;
    let mut chunks_to_ignore: Vec<usize> = Vec::new();

    'outer: for num in numbers {
        for n in grid.iter_mut() {
            if n.number == num {
                n.marked = true;
            }
        }

        num_count += 1;

        // check after 5 nums
        if num_count >= 5 {
            // check per grid (25 elements)
            for (i, grid_all) in grid.chunks(25).enumerate() {
                if chunks_to_ignore.contains(&i) {
                    continue;
                }
                // column check
                if tally(grid_all, false, num) {
                    chunks_to_ignore.push(i);
                    // day 2 - instead of jumping to the next number, we find and mark all
                    // remaining grids
                    continue;
                };

                // row check
                if tally(grid_all, true, num) {
                    chunks_to_ignore.push(i);
                    continue;
                };
            }
        }
    }

    // println!("numbers: {:?}", numbers);
    // println!("Grids: {:?}", grid.len());
}
