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
    let winning_nums: Vec<Vec<i32>> = left
        .into_iter()
        .map(|line| {
            line.split_whitespace()
                .map(|element| element.trim().parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect();
    let player_nums: Vec<Vec<i32>> = right
        .into_iter()
        .map(|line| {
            line.split_whitespace()
                .map(|element| element.trim().parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect();
    // dbg!((winning_nums.len(),player_nums.len()));
    let zipped: Vec<_> = winning_nums
        .into_iter()
        .zip(player_nums.into_iter())
        .collect();
    let res: i32 = zipped
        .iter()
        .map(|pair| calc_score(pair.0.as_slice(), pair.1.as_slice()))
        // .inspect(|elem| {dbg!(elem);})
        .fold(0, |acc,x|acc+x);
    dbg!(res);
}

fn calc_score(left: &[i32], right: &[i32]) -> i32 {
    let mut amount: u32 = 0;
    right
        .into_iter()
        .for_each(|elem| match left.binary_search(elem) {
            Ok(_) => amount = amount + 1,
            Err(_) => (),
        });
        match amount {
            0 => 0,
            _ => 2_i32.pow(amount - 1)
        }
    
}
