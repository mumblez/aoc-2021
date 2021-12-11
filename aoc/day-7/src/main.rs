fn main() {
    let input = std::fs::read_to_string("input.txt")
        .unwrap()
        .trim()
        .split(',')
        .map(|x| x.parse::<u16>().unwrap())
        .collect::<Vec<_>>();

    let mut distance = u32::MAX;
    let min: u16 = *input.iter().min().unwrap();
    let max: u16 = *input.iter().max().unwrap();

    // naive day 1
    // for i in min..=max {
    //     let fuel: u32 = input
    //         .iter()
    //         .fold(0, |acc, x| acc + ((*x as i32 - i as i32).abs() as u32));
    //     if fuel < distance {
    //         distance = fuel;
    //     }
    // }

    // day 2, at each step, it uses +1 more fuel than the previous step
    // algorithm where n = distance from target, = n(n+1) / 2
    for i in min..=max {
        let fuel: u32 = input.iter().fold(0, |acc, x| {
            let dis = (*x as i32 - i as i32).abs() as u32;
            acc + ((dis * (dis + 1)) / 2)
        });
        if fuel < distance {
            distance = fuel;
        }
    }

    println!("answer {}", distance);

    // println!("min {:?}", input.iter().min());
    // println!("max {:?}", input.iter().max());
}
