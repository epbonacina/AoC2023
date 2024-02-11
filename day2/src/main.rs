use regex::Regex;
use std::fs;

const INPUT_FILEPATH: &str = "input.txt";
const MAX_RED_PERMITTED: u8 = 12;
const MAX_BLUE_PERMITTED: u8 = 14;
const MAX_GREEN_PERMITTED: u8 = 13;

fn read_input_file() -> Vec<String> {
    fs::read_to_string(INPUT_FILEPATH)
        .expect("Couldn't read input file")
        .lines()
        .map(String::from)
        .collect()
}

fn get_matches(text: &str, pattern: &str) -> Vec<String> {
    let re = Regex::new(pattern).unwrap();
    re.captures_iter(text)
        .flat_map(|c| c.iter().skip(1).map(|m| m.unwrap().as_str().to_string()).collect::<Vec<String>>())
        .collect()
}

fn get_game_records(line: &str) -> (u8, Vec<u8>, Vec<u8>, Vec<u8>) {
    let game_id_matches = get_matches(line, r"^Game (.+):");
    let red_matches = get_matches(line, r" ([0-9]+) red");
    let blue_matches = get_matches(line, r" ([0-9]+) blue");
    let green_matches = get_matches(line, r" ([0-9]+) green");

    (
        game_id_matches.iter().map(|c| c.parse().unwrap()).next().unwrap(),
        red_matches.iter().map(|c| c.parse().unwrap()).collect(),
        blue_matches.iter().map(|c| c.parse().unwrap()).collect(),
        green_matches.iter().map(|c| c.parse().unwrap()).collect(),
    )
}

fn get_max_value(values: Vec<u8>) -> u8 {
    let mut max = 0;
    for value in values {
        if value > max {
            max = value;
        }
    }
    max
}

fn main() {
    let lines = read_input_file();
    let mut sum = 0;
    for line in lines {
        let (id, red, blue, green) = get_game_records(&line);
        let max_red_found = get_max_value(red);
        let max_blue_found = get_max_value(blue);
        let max_green_found = get_max_value(green);

        if max_red_found > MAX_RED_PERMITTED || max_blue_found > MAX_BLUE_PERMITTED || max_green_found > MAX_GREEN_PERMITTED {
            continue;
        }
        sum += id as u16;
    }
    println!("{}", sum);
}
