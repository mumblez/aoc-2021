use std::fs;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn get_reader() -> BufReader<File> {
    let file = File::open("input.txt").unwrap();
    BufReader::new(file)
}

fn split_input(idx: usize, input: Vec<String>, prefer: char) -> Vec<String> {
    let mut ones: Vec<String> = Vec::new();
    let mut zeroes: Vec<String> = Vec::new();

    for l in input {
        let c = l.chars().nth(idx).unwrap();
        if c == '1' {
            ones.push(l.into());
        } else {
            zeroes.push(l.into());
        }
    }

    if prefer == '1' {
        if ones.len() >= zeroes.len() {
            return ones;
        } else {
            return zeroes;
        }
    } else {
        if ones.len() >= zeroes.len() {
            return zeroes;
        } else {
            return ones;
        }
    }
}

fn find_value(pref: char, data: Vec<String>, item_size: u8) -> u32 {
    let mut index = 0_u8;

    let mut data_clone = data;

    while index < item_size as u8 {
        let tmp = split_input(index as usize, data_clone, pref);

        index += 1;
        data_clone = tmp;
        if data_clone.len() == 1 {
            break;
        }
    }

    let mut decimal = 0_u32;

    for (i, n) in data_clone[0].chars().enumerate() {
        if n == '1' {
            decimal += 2_u32.pow(data_clone[0].len() as u32 - (i as u32 + 1))
        }
    }

    decimal
}

fn main() {
    let reader = get_reader();
    let item_size: usize = reader.lines().nth(0).unwrap().unwrap().len();

    let reader = get_reader();

    let mut tally = vec![0_u32; item_size];
    let mut total_lines = 0;

    for line in reader.lines() {
        let l = line.unwrap();
        for (i, c) in l.chars().enumerate() {
            if c == '1' {
                tally[i] += 1;
            }
        }
        total_lines += 1;
    }

    let mut gamma = 0_u32;
    let mut epsilon = 0_u32;
    let half = total_lines / 2;

    for (i, n) in tally.iter().enumerate() {
        if n > &half {
            gamma += 2_u32.pow(tally.len() as u32 - (i as u32 + 1))
        } else {
            epsilon += 2_u32.pow(tally.len() as u32 - (i as u32 + 1))
        }
    }

    println!("part 1 answer {:?}", gamma * epsilon);

    // day 2
    // read entire input to vec of strings
    // send to function that accepts index, returns 2 vecs for 1's and 0's
    // recurse until index is end of string and only 1 element remains
    let contents: Vec<String> = fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|x| x.into())
        .collect();

    let oxygen = find_value('1', contents.clone(), item_size as u8);
    let co2 = find_value('0', contents.clone(), item_size as u8);

    println!("oxygen: {}", oxygen);
    println!("co2: {}", co2);
    println!("Answer: {}", co2 * oxygen);
}
