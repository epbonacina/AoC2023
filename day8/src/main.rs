use std::collections::HashMap;
use std::fs;

const FILE_PATH: &str = "input.txt";
// const FILE_PATH: &str = "smaller_input.txt";

fn read_input_file() -> Vec<String> {
    fs::read_to_string(FILE_PATH)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

struct Direction {
    left: String,
    right: String,
}

impl Direction {
    fn from(input: (&str, &str)) -> Direction {
        Direction {
            left: input.0.to_string(),
            right: input.1.to_string(),
        }
    }
}

enum Instruction {
    Left,
    Right,
}

impl Instruction {
    fn from(instruction_char: char) -> Instruction {
        match instruction_char {
            'L' => Instruction::Left,
            'R' => Instruction::Right,
            _ => panic!("Invalid instruction: {}", instruction_char),
        }
    }
}

fn get_instructions(lines: &Vec<String>) -> Vec<Instruction> {
    let instructions_line = &lines[0];
    instructions_line.chars().map(Instruction::from).collect()
}

fn get_directions(lines: &Vec<String>) -> HashMap<String, Direction> {
    let directions_section = &lines[2..];
    let mut result = HashMap::new();
    for line in directions_section {
        let (map_name, mapped_direction) = line.split_once(" = ").unwrap();
        let (left, right) = mapped_direction[1..mapped_direction.len() - 1]
            .split_once(", ")
            .unwrap();
        let direction = Direction::from((left, right));
        result.insert(map_name.to_string(), direction);
    }
    result
}

fn count_steps_until_zzz(
    instructions: Vec<Instruction>,
    directions: HashMap<String, Direction>,
) -> u32 {
    let mut count = 0;
    let mut current_map = "AAA".to_string();
    for instruction in instructions.iter().cycle() {
        let direction = directions.get(&current_map).unwrap();
        current_map = match instruction {
            Instruction::Left => direction.left.clone(),
            Instruction::Right => direction.right.clone(),
        };
        count += 1;

        if current_map == "ZZZ" {
            break;
        }
    }
    count
}

fn main() {
    let lines = read_input_file();
    let instructions = get_instructions(&lines);
    let directions = get_directions(&lines);
    let steps_count = count_steps_until_zzz(instructions, directions);
    println!("Part one solution: {}", steps_count);
}
