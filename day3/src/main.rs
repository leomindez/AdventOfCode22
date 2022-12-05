use std::collections::HashSet;

fn main() {
    part_one();
    part_two();
}

fn part_one() {
    let result: i16 = include_bytes!("input")
        .split(|b| *b == b'\n')
        .map(|line| line.split_at(line.len() / 2))
        .map(|group| {
            let group1: HashSet<_> = group.0.iter().cloned().collect();
            let group2: HashSet<_> = group.1.iter().cloned().collect();

            return group1.intersection(&group2).fold(0, |mut acc, value| {
                if *value >= b'a' {
                    acc += (value - b'a') as i16 + 1;
                    acc
                } else {
                    acc += (value - b'A') as i16 + 27;
                    acc
                }
            });
        })
        .sum();

    println!("{}", result);
}

fn part_two() {
    let result: i16 = include_bytes!("input")
        .split(|b| *b == b'\n')
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|group| {
            let group1: HashSet<_> = group[0].iter().cloned().collect();
            let group2: HashSet<_> = group[1].iter().cloned().collect();
            let group3: HashSet<_> = group[2].iter().cloned().collect();

            return group1
                .intersection(&group3.intersection(&group2).cloned().collect())
                .fold(0, |mut acc, value| {
                    if *value >= b'a' {
                        acc += (value - b'a') as i16 + 1;
                        acc
                    } else {
                        acc += (value - b'A') as i16 + 27;
                        acc
                    }
                });
        })
        .sum();

    println!("{:?}", result)
}
