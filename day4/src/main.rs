use std::fs;

const FILE_PATH: &str = "input.txt";
// const FILE_PATH: &str = "smaller_input.txt";


fn read_input_file() -> Vec<(Vec<u32>, Vec<u32>)> {
    fs::read_to_string(FILE_PATH)
        .expect("Couldn't read input file")
        .lines()
        .map(remove_unnecessary_stuff)
        .map(as_vectors_of_numbers)
        .collect()
}

fn remove_unnecessary_stuff(line: &str) -> &str {
    line.split_once(":").unwrap().1
}

fn as_vectors_of_numbers(line: &str) -> (Vec<u32>, Vec<u32>) {
    let (left_numbers, right_numbers) = line.split_once("|").unwrap();
    let left_numbers = left_numbers
        .trim()
        .split_whitespace()
        .map(|num| num.parse().unwrap())
        .collect();

    let right_numbers = right_numbers
        .trim()
        .split_whitespace()
        .map(|num| num.parse().unwrap())
        .collect();

    (left_numbers, right_numbers)
}

fn main() {
    let lines = read_input_file();

    let mut total_points = 0;
    for (winning_numbers, my_numbers) in lines.iter() {
        let matching_numbers: Vec<u32> = winning_numbers
            .into_iter()
            .filter(|n| my_numbers.contains(&n))
            .map(|&n| n)
            .collect();
        if matching_numbers.len() > 0 {
            total_points += 2u32.pow(matching_numbers.len() as u32 - 1);
        }
        println!("{:?}", matching_numbers);
    }
    println!("{:?}", total_points);
}
