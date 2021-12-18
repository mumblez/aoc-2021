use std::collections::HashMap;

fn pairs_from_string(
    data: &str,
    tally: &mut HashMap<char, u64>,
    pair_map: &HashMap<(char, char), char>,
) -> HashMap<(char, char), u64> {
    let mut new_pairs: HashMap<(char, char), u64> = HashMap::new();

    let mut last_char = 'x';

    for (i, c) in data.chars().enumerate() {
        // tally first char and skip
        if i == 0 {
            tally.insert(c, 1);
            last_char = c;
            continue;
        }

        // tally each char in string
        let value = tally.entry(c).or_insert(0);
        *value += 1;

        let middle = pair_map[&(last_char, c)];

        // tally char from pair_map
        let value = tally.entry(middle).or_insert(0);
        *value += 1;

        // insert 2x(char, char) into new_pairs
        let value = new_pairs.entry((last_char, middle)).or_insert(0);
        *value += 1;
        let value = new_pairs.entry((middle, c)).or_insert(0);
        *value += 1;

        last_char = c;
    }

    new_pairs
}

fn create_new_pairs(
    current_pairs: HashMap<(char, char), u64>,
    tally: &mut HashMap<char, u64>,
    pair_map: &HashMap<(char, char), char>,
) -> HashMap<(char, char), u64> {
    let mut new_pairs: HashMap<(char, char), u64> = HashMap::new();

    for ((left, right), count) in current_pairs {
        let middle = pair_map[&(left, right)];

        // tally char from pair_map
        let value = tally.entry(middle).or_insert(0);
        *value += count;

        // insert 2x(char, char) into new_pairs
        let value = new_pairs.entry((left, middle)).or_insert(0);
        *value += count;
        let value = new_pairs.entry((middle, right)).or_insert(0);
        *value += count;
    }
    new_pairs
}

fn main() {
    let template_string = "SNPVPFCPPKSBNSPSPSOF";
    let input = include_str!("../input.txt");

    // create pair mapping
    let mut pair_map: HashMap<(char, char), char> = HashMap::new();
    for line in input.lines() {
        let (keys, value) = line.trim().split_once(" -> ").unwrap();
        let mut kiter = keys.chars();
        pair_map.insert(
            (kiter.next().unwrap(), kiter.next().unwrap()),
            value.chars().next().unwrap(),
        );
    }

    #[allow(unused_assignments)]
    let mut pairs: HashMap<(char, char), u64> = HashMap::new();
    let mut tally: HashMap<char, u64> = HashMap::new();

    // step 1 (from string)
    pairs = pairs_from_string(template_string, &mut tally, &pair_map);

    let steps = 40;

    // step 2 onwards
    for _step in 2..=steps {
        pairs = create_new_pairs(pairs, &mut tally, &pair_map);
    }

    // calculate max - min
    let (mut min, mut max) = (u64::MAX, u64::MIN);
    for (_, count) in &tally {
        if *count > max {
            max = *count;
        }
        if *count < min {
            min = *count;
        }
    }

    println!("Answer: {}", max - min);
}
