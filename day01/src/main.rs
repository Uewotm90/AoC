use std::fs::read_to_string;

fn main() {
    let input =
        read_to_string(r"C:\Users\ander\Documents\GitHub\AoC\AoC\day01\src\input.txt").unwrap();
    let split: Vec<String> = input.split("\n").map(|str| str.into()).collect();
    let cleaned: i32 = split
        .into_iter()
        .map(|line| {
            let chars: Vec<char> = line.chars().filter(|x| x.is_ascii_digit()).collect();
            // dbg!(&chars);
            format!("{}{}", chars.first().unwrap(), chars.last().unwrap())
            // .fold("".into(), |acc, x| format!("{acc}{x}"))
        })
        // .inspect(|item| {dbg!(item);})
        .map(|clean| clean.parse::<i32>().unwrap())
        .sum();
    dbg!(cleaned);
}
