use std::collections::HashSet;

#[allow(dead_code)]
fn print_grid(grid: &[[u8; 10]; 10]) {
    for row in grid {
        for col in row {
            print!("{}", col);
        }
        print!("\n");
    }
    println!("-----");
}

fn increment(grid: &mut [[u8; 10]; 10]) {
    for y in 0..10 {
        for x in 0..10 {
            if grid[y][x] == 9 {
                grid[y][x] = 0;
            } else {
                grid[y][x] += 1;
            }
        }
    }
}

fn traverse_flashes(
    grid: &mut [[u8; 10]; 10],
    x: usize,
    y: usize,
    flashes_traversed: &mut HashSet<(usize, usize)>,
) -> u32 {
    let mut flashes = 0_u32;

    // flash and reset to 0
    if grid[y][x] == 9 {
        grid[y][x] = 0;
    }

    // ignore already flashed and increment normal number
    if grid[y][x] != 0 {
        grid[y][x] += 1;
        return 0;
    }

    // record visit and return if already visited
    if flashes_traversed.contains(&(x, y)) {
        return 0;
    } else {
        flashes_traversed.insert((x, y));
        flashes += 1;
    }

    // at this point it's a new flash (0) so we should
    // check and traverse surrounding cells
    let top = y == 0;
    let bottom = y == 9;
    let left = x == 0;
    let right = x == 9;

    if !top {
        flashes += traverse_flashes(grid, x, y - 1, flashes_traversed);
        if !left {
            flashes += traverse_flashes(grid, x - 1, y - 1, flashes_traversed);
        }
        if !right {
            flashes += traverse_flashes(grid, x + 1, y - 1, flashes_traversed);
        }
    }
    if !bottom {
        flashes += traverse_flashes(grid, x, y + 1, flashes_traversed);
        if !left {
            flashes += traverse_flashes(grid, x - 1, y + 1, flashes_traversed);
        }
        if !right {
            flashes += traverse_flashes(grid, x + 1, y + 1, flashes_traversed);
        }
    }
    if !left {
        flashes += traverse_flashes(grid, x - 1, y, flashes_traversed);
    }
    if !right {
        flashes += traverse_flashes(grid, x + 1, y, flashes_traversed);
    }

    flashes
}

fn check_all_flash(grid: &[[u8; 10]; 10]) -> bool {
    let all_flash = true;
    for row in grid {
        for col in row {
            if *col != 0 {
                return false;
            }
        }
    }
    all_flash
}

fn main() {
    let input = include_str!("../input.txt");

    let mut grid = [[0_u8; 10]; 10];

    for (y, l) in input.lines().enumerate() {
        for (x, c) in l.trim().chars().enumerate() {
            grid[y][x] = c.to_digit(10).unwrap() as u8;
        }
    }

    let mut step = 0;
    let mut flashes = 0_u32;
    let mut all_flashed = false;
    // while step < 100 {
    while !all_flashed {
        // increment every value by 1
        increment(&mut grid);

        // recursively traverse surrounding cells,
        // increment and check if flash
        let mut flashes_traversed: HashSet<(usize, usize)> = HashSet::new();

        for y in 0..10 {
            for x in 0..10 {
                if grid[y][x] == 0 {
                    // recursively check surrounding flashes
                    flashes += traverse_flashes(&mut grid, x, y, &mut flashes_traversed);
                }
            }
        }
        // part 2
        // what step when all flash!
        all_flashed = check_all_flash(&grid);
        if all_flashed {
            println!("All flash at step: {}", step + 1);
        }
        step += 1;
    }

    // part 1
    println!("Flashes: {}", flashes);
}
