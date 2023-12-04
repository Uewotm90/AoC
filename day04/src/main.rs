use std::fs::read_to_string;
// use std::env;

fn main() {
    let input = read_to_string(r"src/input.txt")
        .unwrap()
        .split("\n")
        .map(|str| str.into())
        .collect::<Vec<String>>();

    let cleaned = input
        .into_iter()
        .map(|line| line.split_at(2+line.find(":").unwrap()).1.into())
        .collect::<Vec<String>>();
    // dbg!(cleaned);
}

fn calc_score(left: &[i32],right: &[i32]) -> i32 {
    todo!()
}