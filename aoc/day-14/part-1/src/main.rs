use std::collections::{HashMap, LinkedList};

fn split_check(
    previous: char,
    template: &mut LinkedList<char>,
    pair_map: &HashMap<&str, char>,
    tally: &mut HashMap<char, u32>,
) -> LinkedList<char> {
    let mut idx = 1_usize;

    let pair = format!("{}{}", previous, *template.front().unwrap());

    // tally previous
    let value = tally.entry(previous).or_insert(0);
    *value += 1;

    if pair_map.contains_key(pair.as_str()) {
        template.push_front(pair_map[pair.as_str()]);
        if template.len() == 2 {
            return template.to_owned();
        }
        idx += 1;
    }

    // final
    if template.len() == 1 {
        return template.to_owned();
    }

    if idx < template.len() {
        let mut right = template.split_off(idx);

        template.append(&mut split_check(
            *template.back().unwrap(),
            &mut right,
            &pair_map,
            tally,
        ));
    }

    template.to_owned()
}

fn main() {
    let template_string = "NNCB".to_string();
    let input = include_str!("../test-input.txt");
    //
    // let template_string = "SNPVPFCPPKSBNSPSPSOF";
    // let input = include_str!("../input.txt");

    // create pair mapping
    let mut pair_map: HashMap<&str, char> = HashMap::new();
    for line in input.lines() {
        let parts = line.trim().split(" -> ").collect::<Vec<&str>>();
        pair_map.insert(parts[0], parts[1].chars().collect::<Vec<char>>()[0]);
    }

    // part 2
    // create linked list
    let mut template = LinkedList::new();

    for c in template_string.chars() {
        template.push_back(c);
    }

    // part 1
    // let mut steps = 10;
    let mut steps = 6;
    // part 2
    // let mut steps = 40; // takes a very long time when shifting chars in strings
    // lots of allocations, they probably want us to use a linked list

    let mut tally: HashMap<char, u32> = HashMap::new();
    while steps > 0 {
        let mut right = template.split_off(1);

        template.append(&mut split_check(
            *template.back().unwrap(),
            &mut right,
            &pair_map,
            &mut tally,
        ));

        steps -= 1;
        print!("-");
        template.iter().for_each(|x| {
            print!("{}", x);
        });
        print!("\n");
        println!("step: {}, template len: {}", (6 - steps), template.len());
    }

    // still too slow, it has to traverse the whole list every time
    // while steps > 0 {
    //     let mut idx = 1_usize;
    //
    //     while idx < template.len() {
    //         let mut right = template.split_off(idx);
    //         // let mut pair = String::new();
    //         // pair.push(*template.back().unwrap());
    //         // pair.push(*right.front().unwrap());
    //
    //         let pair = format!("{}{}", *template.back().unwrap(), *right.front().unwrap());
    //
    //         if pair_map.contains_key(pair.as_str()) {
    //             template.push_back(pair_map[pair.as_str()]);
    //             template.append(&mut right);
    //             idx += 2;
    //         } else {
    //             idx += 1;
    //         }
    //     }
    //
    //     steps -= 1;
    // }

    // while steps > 0 {
    //     let mut idx = 2_usize;
    //     while idx < template.len() + 1 {
    //         let pair = &template[(idx - 2)..idx].to_owned();
    //
    //         if pair_map.contains_key(pair.as_str()) {
    //             template.insert(idx - 1, pair_map[pair.as_str()]);
    //             idx += 2;
    //         } else {
    //             idx += 1;
    //         }
    //     }
    //     steps -= 1;
    // }
    //
    // for c in template.chars() {

    let (mut min, mut max) = (u32::MAX, u32::MIN);
    for (_, count) in tally {
        if count > max {
            max = count;
        }
        if count < min {
            min = count;
        }
    }

    println!("Answer: {}", max - min);
}
