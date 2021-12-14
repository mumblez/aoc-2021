use std::collections::HashSet;

fn main() {
    let input_coordinates = include_str!("../input-numbers.txt");
    let input_folds = include_str!("../input-folds.txt");

    let mut coordinates: HashSet<(usize, usize)> = HashSet::new();
    let mut folds: Vec<(char, usize)> = Vec::new();

    for line in input_coordinates.lines() {
        let parts: Vec<&str> = line.trim().split(',').collect();
        let x: usize = parts[0].parse().unwrap();
        let y: usize = parts[1].parse().unwrap();

        coordinates.insert((x, y));
        // coordinates.push((x, y));
    }

    for line in input_folds.lines() {
        let parts: Vec<&str> = line.trim().split_whitespace().collect();

        let final_parts: Vec<&str> = parts[2].split('=').collect();

        let x_or_y: char = final_parts[0].chars().collect::<Vec<char>>()[0];
        let number: usize = final_parts[1].parse().unwrap();

        folds.push((x_or_y, number));
    }

    for (xy, fold) in folds {
        if xy == 'x' {
            // fold left
            let (mut left, right): (HashSet<(usize, usize)>, HashSet<(usize, usize)>) =
                coordinates.iter().partition(|(x, _y)| x < &fold);

            for dot in right {
                let (mut x, y) = dot;

                x -= (x - fold) * 2;

                if !left.contains(&(x, y)) {
                    left.insert((x, y));
                }
            }

            coordinates = left;
        } else {
            // 'y' - fold up
            let (mut top, bottom): (HashSet<(usize, usize)>, HashSet<(usize, usize)>) =
                coordinates.iter().partition(|(_x, y)| y < &fold);

            for dot in bottom {
                let (x, mut y) = dot;

                y -= (y - fold) * 2;

                if !top.contains(&(x, y)) {
                    top.insert((x, y));
                }
            }

            coordinates = top;
        }
        // part 1
        // println!("After fold, there are {} dots!", coordinates.len());
    }

    // find max x and y to know how big of a grid to create, then print

    let mut max_x = 0_usize;
    let mut max_y = 0_usize;

    for (x, y) in &coordinates {
        if x > &max_x {
            max_x = *x;
        }
        if y > &max_y {
            max_y = *y;
        }
    }

    // used as capacity show should +1 else max x|y will overflow
    max_x += 1;
    max_y += 1;

    let mut grid: Vec<Vec<char>> = Vec::with_capacity(max_y);
    for i in 0..max_y {
        let mut row: Vec<char> = vec![' '; max_x];
        coordinates
            .iter()
            .filter(|(_x, y)| *y == i)
            .for_each(|(x, _y)| {
                row[*x] = '#';
            });
        grid.push(row);
    }

    for row in grid {
        for col in row {
            print!("{}", col);
        }
        print!("\n");
    }

    // general formula
    // store coordinates, maybe hashset
    // for each fold value, if coordinate greater, then we calculate the new
    // x|y by subtracting the (x|y - fold) * 2
    // if no overlap (doesn't exist in hashset) change
    // else remove
    //
    // print the number of coordinates for part 1
    //
    // part 2 we need to print
    // print the coordinates on a 2d grid
}
