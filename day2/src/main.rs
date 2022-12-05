const SCORE_ONE: [usize;9 ] = [4,8,3,1,5,9,7,2,6];
const SCORE_TWO: [usize; 9] = [2,4,9,1,5,9,2,6,7];
fn main() {
    part_one(SCORE_ONE);
    part_one(SCORE_TWO);
}

fn part_one(table: [usize;9]) {
    let result: _ = include_bytes!("input")
    .split(|byte| *byte == b'\n')
    .map(|line| (3 * (line[0] - b'A') + (line[2]  - b'X')) as usize)
    .map(|val| table[val])
    .sum::<usize>();
    println!("{:?}", result);
}
