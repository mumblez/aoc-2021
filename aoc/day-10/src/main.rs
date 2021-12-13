use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let file = File::open("input.txt").unwrap();
    // let file = File::open("test.txt").unwrap();
    let reader = BufReader::new(file);

    // let wrong_points_map: HashMap<char, u16> =
    //     HashMap::from([(')', 3), (']', 57), ('}', 1197), ('>', 25137)]);

    let missing_points_map: HashMap<char, u8> =
        HashMap::from([(')', 1), (']', 2), ('}', 3), ('>', 4)]);

    let opener: HashMap<char, bool> = HashMap::from([
        ('(', true),
        ('[', true),
        ('{', true),
        ('<', true),
        (')', false),
        (']', false),
        ('}', false),
        ('>', false),
    ]);

    let inverse: HashMap<char, char> = HashMap::from([
        ('(', ')'),
        (')', '('),
        ('[', ']'),
        (']', '['),
        ('{', '}'),
        ('}', '{'),
        ('<', '>'),
        ('>', '<'),
    ]);

    // let mut points = 0_u64;

    let mut totals: Vec<u64> = Vec::new();

    // for each line
    for line in reader.lines() {
        // for each char
        let mut chunk: Vec<char> = Vec::new();
        let mut failed = false;

        'inner: for c in line.unwrap().trim().chars() {
            if opener[&c] {
                chunk.push(c);
            } else {
                // c = closer
                if let Some(current) = chunk.pop() {
                    if inverse[&current] != c {
                        failed = true;
                        break 'inner; // disregard corrupt lines
                    }
                }
            }
        }

        // if chunks is not empty we have an imcomplete line
        // collect completions for missing lines
        // reverse loop through remaining chars in chunk and push inverse (closer)
        if chunk.len() > 0 && !failed {
            chunk.reverse();
            let total = chunk
                .iter()
                .map(|x| missing_points_map[&inverse[&x]] as u64)
                .fold(0, |acc, x| (5 * acc) + x);
            totals.push(total);
        }
    }

    // println!("Points: {}", points);

    // sort
    // take middle value, divide by 2
    totals.sort();
    println!("Answer: {}", totals[totals.len() / 2]);
}
