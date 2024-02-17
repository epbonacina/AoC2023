use std::fs;

const INPUT_FILE_PATH: &str = "input.txt";


fn read_input_file() -> Vec<String> {
    fs::read_to_string(INPUT_FILE_PATH)
        .expect("Couldn't read input file")
        .lines()
        .map(String::from)
        .collect()
}


fn get_numbers_adjacent_to_symbols(lines: Vec<String>) -> Vec<i32>{
    let mut numbers_adjacent_to_symbols = Vec::new();
    let mut number = String::new();
    let mut found_relevant_number=false;
    for (y, line) in lines.iter().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if ch.is_digit(10) {
                if is_adjacent_to_symbol(x, y, &lines) {
                    found_relevant_number = true;
                }
                number.push(ch);
            }
            else {
                if found_relevant_number {
                    numbers_adjacent_to_symbols.push(number.clone().parse().unwrap());
                    found_relevant_number = false;
                }
                number.clear();
            }
        }
        if found_relevant_number {
            numbers_adjacent_to_symbols.push(number.clone().parse().unwrap());
            found_relevant_number = false;
        }
        number.clear();
    }
    numbers_adjacent_to_symbols
}


fn is_adjacent_to_symbol(x: usize, y: usize, lines: &Vec<String>) -> bool {
    let number_of_lines = lines.len();
    let line_length = lines[0].len();

    let y_start;
    if y as i32 -1 >= 0 {
        y_start = y-1;
    }
    else {
        y_start = y;
    }

    let y_end;
    if y+1 < number_of_lines {
        y_end = y+1;
    }
    else {
        y_end = y;
    }
    
    let x_start;
    if x as i32 -1 >= 0 {
        x_start = x-1;
    }
    else {
        x_start = x;
    }

    let x_end;
    if x+1 < line_length {
        x_end = x+1;
    }
    else {
        x_end = x;
    }

    for line in lines[y_start..=y_end].iter() { 
        for ch in line.chars().collect::<Vec<char>>()[x_start..=x_end].iter() { 
            if !ch.is_digit(10) && ch != &'.' {
                return true;
            }
        }
    }
    return false;
}


fn main() {
    let lines = read_input_file();
    let numbers = get_numbers_adjacent_to_symbols(lines);
    let mut sum = 0;

    for number in numbers {
        sum += number;
    }
    println!("{:?}", sum);
}
