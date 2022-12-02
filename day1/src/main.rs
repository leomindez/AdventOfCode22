fn main() {
    part_one();
    part_two();
}

fn part_one() {
    let lines = include_str!("input")
        .split("\n\n")
        .map(|calories| {
            calories
                .lines()
                .map(|number| number.parse::<i32>().unwrap())
                .sum::<i32>()
        })
        .max()
        .unwrap();
    println!("Part One {:?}", lines);
}

fn part_two() {
    let mut three_max_cal: Vec<i32> = include_str!("input")
        .split("\n\n")
        .map(|calories| {
            calories
                .lines()
                .map(|number| number.parse::<i32>().unwrap())
                .sum()
        })
        .collect::<Vec<i32>>();

    three_max_cal.sort_unstable();

    let max_cal: i32 = three_max_cal.into_iter().rev().take(3).sum();

    println!("Part Two {:?}", max_cal);
}
