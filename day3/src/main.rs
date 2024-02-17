use std::fs;

const INPUT_FILE_PATH: &str = "input.txt";

fn read_input_file() -> Vec<String> {
    fs::read_to_string(INPUT_FILE_PATH)
        .expect("Couldn't read input file")
        .lines()
        .map(String::from)
        .collect()
}

fn get_numbers_adjacent_to_symbols(lines: &Vec<String>) -> Vec<u32> {
    let mut numbers_adjacent_to_symbols = Vec::new();

    for (y, line) in lines.iter().enumerate() {
        let mut tmp_number = String::new();
        let mut found_a_relevant_number = false;

        for (x, ch) in line.chars().enumerate() {
            match ch {
                digit if ch.is_digit(10) && is_adjacent_to_symbol(&lines, x, y) => {
                    tmp_number.push(digit);
                    found_a_relevant_number = true;
                }
                digit if ch.is_digit(10) => tmp_number.push(digit),
                _ if found_a_relevant_number => {
                    numbers_adjacent_to_symbols.push(tmp_number.parse().unwrap());
                    found_a_relevant_number = false;
                    tmp_number.clear();
                }
                _ => {
                    tmp_number.clear();
                }
            }
        }
        if found_a_relevant_number {
            numbers_adjacent_to_symbols.push(tmp_number.parse().unwrap());
        }
    }
    numbers_adjacent_to_symbols
}

fn is_adjacent_to_symbol(lines: &Vec<String>, x: usize, y: usize) -> bool {
    let candidates_coords = [
        (y.wrapping_sub(1), x.wrapping_sub(1)),
        (y.wrapping_sub(1), x),
        (y.wrapping_sub(1), x.wrapping_add(1)),
        (y, x.wrapping_sub(1)),
        (y, x.wrapping_add(1)),
        (y.wrapping_add(1), x.wrapping_sub(1)),
        (y.wrapping_add(1), x),
        (y.wrapping_add(1), x.wrapping_add(1)),
    ];

    for (candidate_y, candidate_x) in candidates_coords {
        if let Some(line) = lines.get(candidate_y) {
            if let Some(value) = line.chars().nth(candidate_x) {
                if !value.is_digit(10) && value != '.' {
                    return true;
                }
            }
        }
    }
    return false;
}

fn main() {
    let lines = read_input_file();
    let numbers = get_numbers_adjacent_to_symbols(&lines);
    let mut sum = 0;

    for number in numbers {
        sum += number;
    }
    println!("{:?}", sum);
}
