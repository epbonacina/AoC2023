use std::fs;

const INPUT_FILE_PATH: &str = "input.txt";

fn read_input_file() -> Vec<String> {
    fs::read_to_string(INPUT_FILE_PATH)
        .expect("Couldn't read input file")
        .lines()
        .map(String::from)
        .collect()
}

fn build_map(lines: &Vec<String>) -> Vec<Vec<i32>> {
    let mut id = 1;
    let mut map = Vec::new();
    let mut line_ids = Vec::new();

    for line in lines {
        for ch in line.chars() {
            match ch {
                _ if ch.is_digit(10) => line_ids.push(id),
                _ if ch == '.' => {
                    line_ids.push(-1);
                    id += 1;
                }
                _ => {
                    line_ids.push(0);
                    id += 1;
                }
            }
        }

        map.push(line_ids.clone());
        line_ids.clear();
    }
    map
}

fn get_numbers_adjacent_to_symbols(lines: &Vec<String>, map: &Vec<Vec<i32>>) -> Vec<(Vec<u32>, char)> {
    let mut numbers_adjacent_to_symbols = Vec::new();
    for (y, line) in map.iter().enumerate() {
        for (x, id) in line.into_iter().enumerate() {
            match id {
                0 => {
                    let numbers = get_numbers_adjacent_to_symbol(x, y, lines, map);
                    let symbol = lines[y].chars().nth(x).unwrap();
                    numbers_adjacent_to_symbols.push((numbers, symbol));
                }
                _ => {}
            }
        }
    }
    numbers_adjacent_to_symbols
}

fn get_numbers_adjacent_to_symbol(x: usize, y: usize, lines: &Vec<String>, map: &Vec<Vec<i32>>) -> Vec<u32> {
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

    let mut numbers_adjacent_to_symbol = Vec::new();
    let mut used_ids = Vec::new();
    for (candidate_y, candidate_x) in candidates_coords {
        if let Some(candidate_line) = map.get(candidate_y) {
            let digit_id = candidate_line[candidate_x];
            if used_ids.contains(&digit_id) || digit_id == 0 || digit_id == -1 {
                continue;
            }
            let number = get_number_by_id(digit_id, lines, map);
            used_ids.push(digit_id);
            numbers_adjacent_to_symbol.push(number);
        }
    }
    numbers_adjacent_to_symbol
}

fn get_number_by_id(target_id: i32, lines: &Vec<String>, map: &Vec<Vec<i32>>) -> u32 {
    let mut number = String::new();

    for (y, line) in lines.iter().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            let line_id = map[y][x];
            if line_id == target_id {
                number.push(ch);
            }
        }
    }
    number.parse().unwrap()
}

fn main() {
    let lines = read_input_file();
    let map = build_map(&lines);
    let adjacent_numbers = get_numbers_adjacent_to_symbols(&lines, &map);

    let mut gear_ratios_summed_up = 0;
    for (numbers, symbol) in adjacent_numbers {
        if symbol == '*' && numbers.len() == 2 {
            gear_ratios_summed_up += numbers[0]*numbers[1];
        }
    }
    println!("Gear ratios summed up: {}", gear_ratios_summed_up);
}






