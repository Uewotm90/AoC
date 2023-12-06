use std::collections::HashMap;
use std::fs::read_to_string;
use std::path::Path;

struct Almanac {
    seeds: Vec<u32>,
    seed_to_soil: HashMap<u32, u32>,
    soil_to_fert: HashMap<u32, u32>,
    fert_to_water: HashMap<u32, u32>,
    water_to_light: HashMap<u32, u32>,
    light_to_temp: HashMap<u32, u32>,
    temp_to_hum: HashMap<u32, u32>,
    hum_to_loc: HashMap<u32, u32>,
}

fn main() -> std::io::Result<()> {
    let input = file_to_vec(r"C:\Users\ander\Documents\GitHub\AoC\AoC\day05\src\input.txt")?;
    let almanac = parse_almanac(input);
    Ok(())
}

fn file_to_vec(path: impl AsRef<Path>) -> std::io::Result<Vec<String>> {
    let res = read_to_string(path)?
        .split("\n")
        .map(|str| str.into())
        .collect::<Vec<_>>();
    Ok(dbg!(res))
}
fn parse_almanac(input: Vec<String>) -> Almanac {
    let seeds = input[0]
        .split_at(input[0].find(":").unwrap())
        .1
        .split(" ")
        .filter_map(|elem| elem.trim().parse::<u32>().ok())
        .collect::<Vec<_>>();
    let mut iterator = input.iter();
    let seed_to_soil_delim_begin = iterator
    .position(|elem| elem.is_empty()).unwrap();
    let seed_to_soil_delim_end = iterator
    .position(|elem| dbg!(dbg!(elem).is_empty())).unwrap();
    dbg!(&input[seed_to_soil_delim_begin],&input[seed_to_soil_delim_end]);
    // dbg!(input);
    todo!()
}
// [day05\src\main.rs:14] res = [
//     "seeds: 79 14 55 13",
//     "",
//     "seed-to-soil map:",
//     "50 98 2",
//     "52 50 48",
//     "",
//     "soil-to-fertilizer map:",
//     "0 15 37",
//     "37 52 2",
//     "39 0 15",
//     "",
//     "fertilizer-to-water map:",
//     "49 53 8",
//     "0 11 42",
//     "42 0 7",
//     "57 7 4",
//     "",
//     "water-to-light map:",
//     "88 18 7",
//     "18 25 70",
//     "",
//     "light-to-temperature map:",
//     "45 77 23",
//     "81 45 19",
//     "68 64 13",
//     "",
//     "temperature-to-humidity map:",
//     "0 69 1",
//     "1 0 69",
//     "",
//     "humidity-to-location map:",
//     "60 56 37",
//     "56 93 4",
//     "",
// ]
