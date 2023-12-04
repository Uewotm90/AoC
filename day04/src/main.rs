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
        .map(|line| line.split_at(2 + line.find(":").unwrap()).1.into())
        .collect::<Vec<String>>();
    // dbg!(cleaned);
    let left: Vec<String> = cleaned
        .iter()
        .map(|line| line.split_at(line.find("|").unwrap()).0.into())
        .collect();
    let right: Vec<String> = cleaned
        .iter()
        .map(|line| line.split_at(1 + line.find("|").unwrap()).1.into())
        .collect();
    let winning_nums: Vec<Vec<i32>> = left.into_iter().map(|line| {
        line.split_whitespace().map(|element| element.trim().parse::<i32>().unwrap()).collect::<Vec<i32>>()
    }).collect();
    let player_nums: Vec<Vec<i32>> = right.into_iter().map(|line| {
        line.split_whitespace().map(|element| element.trim().parse::<i32>().unwrap()).collect::<Vec<i32>>()
    }).collect();
    // dbg!((winning_nums.len(),player_nums.len()));
    let zipped: Vec<_> = winning_nums.into_iter().zip(player_nums.into_iter()).collect();

}

fn calc_score(left: &[i32], right: &[i32]) -> i32 {
    let mut amount: i32;
    right.into_iter().for_each(|elem|left.)
}
