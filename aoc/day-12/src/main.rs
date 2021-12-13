use std::collections::HashMap;

// part 1
// fn traverse(
//     node: &HashMap<&str, Vec<&str>>,
//     valid_paths: &mut Vec<Vec<String>>,
//     position: &str,
//     journey: &mut Vec<String>,
// ) {
//     // if position is lowercase (small cave) and we've already visited, then return
//     if position.to_lowercase() == position.to_string() && journey.contains(&position.to_string()) {
//         return;
//     }
//
//     // that leaves capitals and non-visited small caves
//     // add current position
//     journey.push(position.to_string());
//
//     // if position is 'end' then journey complete, add to valid path
//     if position == "end" {
//         valid_paths.push(journey.to_vec());
//         return;
//     }
//
//     // for all connections, clone journey and append to their own list
//     for cave in &node[position] {
//         traverse(node, valid_paths, &cave, &mut journey.clone());
//     }
// }

// part 2,
// - start and end can only be visited once
// - ONE small cave can be visited twice!

fn traverse(
    node: &HashMap<&str, Vec<&str>>,
    valid_paths: &mut Vec<Vec<String>>,
    position: &str,
    journey: &mut Vec<String>,
) {
    // if position is lowercase (small cave) and we've already visited, ensure we only
    // visit ONE small cave twice
    // println!("b-");
    if position.to_lowercase() == position.to_string() && position != "start" {
        let mut small_caves: HashMap<&str, u8> = HashMap::new();

        // tally all small caves
        journey
            .iter()
            .filter(|x| x.to_lowercase() == x.to_string())
            .for_each(|x| {
                let tally = small_caves.entry(x).or_insert(0);
                *tally += 1;
            });

        // println!("small_caves {:?}", small_caves);
        // println!("c- {:?}", journey);

        // if find any small cave that we've visited twice
        // then return (invalid journey)
        let mut two_times: HashMap<&str, u8> = HashMap::new();
        small_caves
            .iter()
            .filter(|(_k, v)| **v > 1)
            .for_each(|(k, v)| {
                two_times.insert(k, *v);
            });

        // println!("d- two_times: {:?}", two_times);
        // multiple 2x small caves
        if two_times.len() > 1 {
            // println!("found multiple caves more than two_times");
            return;
        }
        // println!("e-");
        // small cave visited over 2x
        for (_k, v) in two_times {
            if v > 2 {
                // println!("found cave visited more than 2 times");
                return;
            }
        }

        // if already 2 for current position
        if small_caves.contains_key(position) {
            if small_caves[position] == 2 {
                return;
            }
        }
    }

    // println!("f-");
    // can only visit 'start' once
    if position == "start" && journey.len() != 0 {
        return;
    }

    // println!("g-");
    // that leaves capitals and non-visited small caves (or visited but only once)
    // add current position
    journey.push(position.to_string());

    // println!("h-");
    // if position is 'end' then journey complete, add to valid path
    if position == "end" {
        valid_paths.push(journey.to_vec());
        // println!("Valid paths: {:?}", valid_paths);
        return;
    }

    // println!("i-");
    // for all connections, clone journey and append to their own list
    for cave in &node[position] {
        // println!("a-");
        traverse(node, valid_paths, &cave, &mut journey.clone());
    }
}

fn main() {
    let input = include_str!("../input.txt");

    // list of nodes, with collection of connecting nodes
    let mut node: HashMap<&str, Vec<&str>> = HashMap::new();

    for line in input.lines() {
        let parts: Vec<&str> = line.trim().split('-').collect();
        let left = parts[0];
        let right = parts[1];

        let left_collection = node.entry(left).or_insert_with(|| Vec::new());
        if !left_collection.contains(&right) {
            left_collection.push(right);
        }

        let right_collection = node.entry(right).or_insert_with(|| Vec::new());
        if !right_collection.contains(&left) {
            right_collection.push(left);
        }
    }

    let mut valid_paths: Vec<Vec<String>> = Vec::new();
    let mut journey: Vec<String> = Vec::new();
    traverse(&node, &mut valid_paths, "start", &mut journey);

    println!("valid_paths {:?}", valid_paths.iter().count());
    // println!("paths {:?}", valid_paths);
    // valid_paths.iter().for_each(|x| {
    //     for cave in x {
    //         print!("{},", cave);
    //     }
    //     print!("\n");
    // });

    // println!("node: {:?}", node);
    // // println!("paths: {:?}", valid_paths);
    // println!(
    //     "is uppercase {:?}",
    //     node.keys()
    //         .filter(|x| x.to_uppercase() == x.to_string())
    //         .map(|x| *x)
    //         .collect::<Vec<&str>>()
    // );
    // println!(
    //     "is lowercase {:?}",
    //     node.keys()
    //         .filter(|x| x.to_lowercase() == x.to_string())
    //         .map(|x| *x)
    //         .collect::<Vec<&str>>()
    // );
}
