use std::collections::HashMap;
use std::iter::FromIterator;

// display numbers = segments
// 0,6,9 = 6
// 1     = 2
// 2,3,5 = 5
// 4     = 4
// 7     = 3
// 8     = 7 (all)
//
// deducing segments
// unique: 1, 4, 7, 8 - find these first
// 9 = contains all of 1, also contains all of 3, remainder is top left
// 6 = remainder, also contains all of 5, remainder is bottom left
// 0 = which ever is missing middle
// so far we know 0, 1, .., 4, .. 6, 7, 8, 9
// 3 = contains all of 1
// 5 = contains top left
// 2 = remainder
// top: 7 contains 1, remaining char is top
// middle: 2,3,5 contains top middle bottom, which ever matches 4 = middle, remaining = top
// top left: remainder from above, or if we have all 5 chars x3, top left is missing on all,
// whilst right top and right bottom is common in at least 2, can deduce 5, then deduce 2 if 2
// chars mismatch, and 3 when only 1 mismatch, from 3 we can deduce 1, from 5 and 3 we can
// deduce 4
// we can also deduce 0 middle segment from one of the 5 chars

fn contains(big: &str, small: &str) -> bool {
    let mut tally = 0_u8;

    for c in big.chars() {
        for x in small.chars() {
            if c == x {
                tally += 1;
                break;
            }
        }
    }

    let mut contains = false;

    if tally == small.len() as u8 {
        contains = true;
    }

    contains
}

fn process(digits: &Vec<String>, display: &Vec<String>) -> u32 {
    // let all_chars = "abcdefg";
    // Fill in the segments when we can figure out the mapping
    // 'a' = top
    // 'd' = middle
    // 'g' = bottom
    // 'b' = top left
    // 'e' = bottom left
    // 'c' = top right
    // 'f' = bottom right
    let mut segments: HashMap<char, char> = HashMap::new();

    // Fill in numbers we recognise from the letters / length
    // sort the letters before inserting
    let mut numbers: HashMap<u8, String> = HashMap::new();
    let mut fives: Vec<String> = Vec::new();
    let mut sixes: Vec<String> = Vec::new();

    for digit in digits {
        // sort letters
        let mut tmp_digit: Vec<_> = digit.chars().collect();
        tmp_digit.sort_unstable();
        let d: String = tmp_digit.into_iter().collect();

        // Recognised numbers 1, 4, 7, 8 from length, 2, 4, 3, 7
        if !numbers.contains_key(&1) && digit.len() == 2 {
            numbers.insert(1, d);
        } else if !numbers.contains_key(&4) && digit.len() == 4 {
            numbers.insert(4, d);
        } else if !numbers.contains_key(&7) && digit.len() == 3 {
            numbers.insert(7, d);
        } else if !numbers.contains_key(&8) && digit.len() == 7 {
            numbers.insert(8, d);
        } else if digit.len() == 5 {
            fives.push(d);
        } else if digit.len() == 6 {
            sixes.push(d);
        }
    }

    // Calculate digit segments
    // [3] contains all segments of 1 from fives
    'outer: for five in &fives {
        if contains(five, &numbers[&1]) {
            for c in five.chars() {
                if !numbers[&1].contains(c) {
                    numbers.insert(3, five.clone());
                    break 'outer;
                }
            }
        }
    }

    // calculate *middle* segment (d) and *bottom* from all fives that appear 3x and which of those which match 4
    // (contains)
    let mut horizontals: Vec<char> = Vec::new();
    for c in fives[0].chars() {
        if fives[1].contains(c) && fives[2].contains(c) {
            horizontals.push(c);
        }
    }
    // calculate *top*, *middle*, *bottom* segment (a, d, g) from remaining char of 7 contains 1
    for c in &horizontals {
        if numbers[&4].contains(*c) {
            segments.insert('d', *c);
        } else if numbers[&7].contains(*c) {
            segments.insert('a', *c);
        } else {
            segments.insert('g', *c);
        }
    }

    // calculate *top left* segment (b), remove 3 (contains all of 7), [2] matches 2 from 4, [5]
    // matches 3 from 4
    let mut fives_tmp: Vec<String> = Vec::new();

    for five in &fives {
        // if five.contains(&one) {
        if contains(&five, &numbers[&1]) {
            numbers.insert(3, five.clone());
        } else {
            fives_tmp.push(five.to_string());
        }
    }

    for d in fives_tmp {
        // if contains 2 chars of 4 = 2
        // if contains 3 chars of 4 = 5
        let count = d.chars().filter(|x| numbers[&4].contains(*x)).count();

        if count == 2 {
            numbers.insert(2, d.into());
        } else {
            numbers.insert(5, d.into());
        }
    }

    // top left, segment 'b'
    let mut segments_tmp = segments.clone();
    numbers[&5]
        .chars()
        .filter(|x| *x != segments_tmp[&'a']) // top
        .filter(|x| *x != segments_tmp[&'d']) // middle
        .filter(|x| *x != segments_tmp[&'g']) // bottom
        .filter(|x| !numbers[&1].contains(*x)) // not part of 1
        .for_each(|x| {
            segments.insert('b', x);
        });

    // bottom right, segment 'f'
    segments_tmp = segments.clone();
    numbers[&5]
        .chars()
        .filter(|x| *x != segments_tmp[&'a']) // top
        .filter(|x| *x != segments_tmp[&'d']) // middle
        .filter(|x| *x != segments_tmp[&'g']) // bottom
        .filter(|x| numbers[&1].contains(*x)) // part of 1
        .for_each(|x| {
            segments.insert('f', x);
        });

    // bottom left, segment 'e', matches all of 3 except for remainder
    numbers[&2]
        .chars()
        .filter(|x| !numbers[&3].contains(*x))
        .for_each(|x| {
            segments.insert('e', x);
        });

    // what remains is top right segment 'c'
    segments_tmp = segments.clone();
    numbers[&2]
        .chars()
        .filter(|x| *x != segments_tmp[&'a']) // top
        .filter(|x| *x != segments_tmp[&'d']) // middle
        .filter(|x| *x != segments_tmp[&'g']) // bottom
        .filter(|x| *x != segments_tmp[&'e']) // bottom left
        .for_each(|x| {
            segments.insert('c', x);
        });

    // map remaining numbers 0, 6, 9 from segments
    let mut nine: Vec<char> = Vec::new();
    nine.push(segments[&'a']);
    nine.push(segments[&'b']);
    nine.push(segments[&'c']);
    nine.push(segments[&'d']);
    nine.push(segments[&'f']);
    nine.push(segments[&'g']);
    nine.sort_unstable();
    numbers.insert(9, String::from_iter(nine));

    let mut zero: Vec<char> = Vec::new();
    zero.push(segments[&'a']);
    zero.push(segments[&'b']);
    zero.push(segments[&'c']);
    zero.push(segments[&'e']);
    zero.push(segments[&'f']);
    zero.push(segments[&'g']);
    zero.sort_unstable();
    numbers.insert(0, String::from_iter(zero));

    let mut six: Vec<char> = Vec::new();
    six.push(segments[&'a']);
    six.push(segments[&'b']);
    six.push(segments[&'d']);
    six.push(segments[&'e']);
    six.push(segments[&'f']);
    six.push(segments[&'g']);
    six.sort_unstable();
    numbers.insert(6, String::from_iter(six));

    // We should have all segments and numbers and can now process display
    let mut display_number = 0_u32;
    let mut multiplier = 1000_u32;

    for digit in display {
        let mut tmp_digit: Vec<_> = digit.chars().collect();
        tmp_digit.sort_unstable();
        let d: String = tmp_digit.into_iter().collect();

        for (k, v) in &numbers {
            if v == &d {
                display_number += *k as u32 * multiplier;
                break;
            }
        }

        multiplier /= 10;
    }

    // println!("Original Numbers: {:?}", digits);
    // println!("Display number: {:?}", display_number);
    // for (k, v) in segments {
    //     println!("{}: {}", k, v);
    // }
    // println!("Numbers: {:?}", numbers);
    // println!("Fives: {:?}", fives);
    // println!("Sixes: {:?}", sixes);

    display_number
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut tally = 0_u32;
    // --- DAY 2 ---

    for line in input.lines() {
        let parts: Vec<String> = line.split('|').map(|x| x.trim().into()).collect();

        let digits: Vec<String> = parts[0].split_whitespace().map(|x| x.into()).collect();
        let display: Vec<String> = parts[1].split_whitespace().map(|x| x.into()).collect();
        tally += process(&digits, &display);
    }

    println!("Taly: {}", tally);

    // -------------------------------------------------------------------------
    // --- DAY 1 ---
    // let mut tally = 0_u32;
    // let recognised: Vec<u8> = vec![2, 3, 4, 7];
    // let mut counts: Vec<Vec<u8>> = Vec::new();
    //
    // for l in input.lines() {
    //     l.split('|')
    //         .filter(|x| x.trim().split_whitespace().count() == 4)
    //         .map(|x| {
    //             let tmp_strings: Vec<&str> = x.trim().split_whitespace().collect();
    //             let tmp_counts: Vec<u8> = tmp_strings
    //                 .iter()
    //                 .map(|x| x.chars().count() as u8)
    //                 .collect();
    //             tmp_counts
    //         })
    //         .for_each(|x| counts.push(x));
    // }

    // tally when we recognise string counts of 2, 3, 4, 7 (display: 1, 7, 4, 8)
    // for count in counts {
    //     for c in count {
    //         if recognised.contains(&(c)) {
    //             tally += 1;
    //         }
    //     }
    // }
    //

    // counts.iter().flat_map(|x| x.iter()).for_each(|c| {
    //     if recognised.contains(c) {
    //         tally += 1;
    //     }
    // });
    //
    // println!("Tally: {}", tally);
}
