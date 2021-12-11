use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    // day 2 - we need a hashmap!
    let mut fishes: HashMap<u8, u64> = HashMap::with_capacity(9);

    // fill map from input.txt
    input
        .trim_end()
        .split(',')
        .map(|x| x.parse::<u8>().unwrap())
        .for_each(|x| {
            let value = fishes.entry(x).or_insert(0);
            *value += 1;
        });

    // process for 256 days
    let day_limit = 256_u16;
    let mut day = 0_u16;

    while day < day_limit {
        let new_fish = if fishes.contains_key(&0) {
            fishes[&0]
        } else {
            0
        };

        // slide values down by one day
        for i in 0..9 {
            // if last day add new fish
            if i == 8 {
                if new_fish > 0 {
                    let tmp = fishes.entry(i).or_insert(0);
                    *tmp = new_fish;
                } else {
                    fishes.remove(&i);
                }
            // ensure next day key exists before sliding down
            } else if fishes.contains_key(&(i + 1)) {
                let new_value = fishes[&(i + 1)];
                let value = fishes.entry(i).or_insert(0);
                *value = new_value;
            // remove key of next one didn't exist
            } else {
                fishes.remove(&i);
            }
        }
        if new_fish > 0 {
            let new_value = fishes.entry(6).or_insert(0);
            *new_value += new_fish;
        }
        day += 1;
    }

    let total = fishes.iter().map(|(_, v)| v).fold(0, |acc, x| acc + x);
    println!("Total: {}", total);
}
