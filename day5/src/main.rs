use std::str;

fn main() {
    part_one();
}

fn part_one() {
    let input = include_bytes!("input.txt");
    let (stacks, instructions) =
        input.split_at(input.windows(2).position(|byte| byte == b"\n\n").unwrap() + 2);

    let mut crates: Vec<Vec<char>> = stacks
        .split(|byte| byte.is_ascii_control())
        .map(|bytes| {
            return bytes
                .chunks(4)
                .map(|chunk| chunk.to_vec())
                .enumerate()
                .map(|(index, bytes)| {
                    (
                        index,
                        bytes
                            .into_iter()
                            .filter(|byte| byte.is_ascii_alphabetic())
                            .map(|byte| byte.clone() as char)
                            .into_iter()                            
                            .collect::<Vec<char>>(),
                    )
                })
                .collect::<Vec<(usize, Vec<char>)>>();
        })
        .flatten()
        .fold(vec![vec![]; 9], |mut acc, (index, crates)| {
            if crates.len() > 0 {
                acc[index].push(crates[0]);
            }
            acc
        })
        .into_iter()
        .map(|mut crates| { crates.reverse(); return crates})
        .collect();

    let movements = instructions
    .split(|byte| byte.is_ascii_control())
    .map(|line| line.split(|byte| byte.is_ascii_whitespace()).collect::<Vec<&[u8]>>())
    .flatten()
    .filter_map(|bytes| str::from_utf8(bytes).unwrap().parse::<usize>().ok())
    .collect::<Vec<usize>>();

    let steps = movements
    .chunks_exact(3)
    .collect::<Vec<&[usize]>>();

println!("{:?}", steps);

    for step in steps {

        let mov = step[0];
        let from = step[1]-1;
        let to = step[2]-1;
   
        
        for _ in 0..mov {
            let tmp = crates[from].pop().unwrap();
            crates[to].push(tmp);
        }
    }

    let final_crate = crates.into_iter().map(|c| *c.last().unwrap()).collect::<String>();
    println!("{:?}", final_crate);

}
