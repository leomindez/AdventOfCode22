//Will be good to use std::ops::Range struct to validate if range contains another range

fn main() {
    part_one();
    part_two();
}

fn part_one() {
    let result = include_str!("input")
        .lines()
        .map(|line| {
            let (left, right) = line.split_once(",").unwrap();
            let (pair1, pair2) = (
                left.split_once("-").unwrap(),
                right.split_once("-").unwrap(),
            );
            (
                pair1.0.parse::<u8>().unwrap(),
                pair1.1.parse::<u8>().unwrap(),
                pair2.0.parse::<u8>().unwrap(),
                pair2.1.parse::<u8>().unwrap(),
            )
        })
        .filter(|(rs1, re1, rs2, re2)| (rs1 >= rs2 && re1 <= re2) || (rs1 <= rs2 && re1 >= re2))
        .count();

    println!("{}", result);
}

fn part_two() {
    let result = include_str!("input")
        .lines()
        .map(|line| {
            let (left, right) = line.split_once(",").unwrap();
            let (pair1, pair2) = (
                left.split_once("-").unwrap(),
                right.split_once("-").unwrap(),
            );
            (
                pair1.0.parse::<u8>().unwrap(),
                pair1.1.parse::<u8>().unwrap(),
                pair2.0.parse::<u8>().unwrap(),
                pair2.1.parse::<u8>().unwrap(),
            )
        })
        .filter(|(rs1, re1, rs2, re2)| (rs1 <= re2) && (rs2 <= re1))
        .count();

    println!("{}", result);
}
