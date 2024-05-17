use std::fs;

const FILE_PATH: &str = "input.txt";
// const FILE_PATH: &str = "smaller_input.txt";


fn read_input_file() -> Vec<(u32, Vec<u32>, Vec<u32>)> {
    fs::read_to_string(FILE_PATH)
        .expect("Couldn't read input file")
        .lines()
        .map(remove_unnecessary_stuff)
        .map(as_vectors_of_numbers)
        .map(add_count)
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

fn add_count(line: (Vec<u32>, Vec<u32>)) -> (u32, Vec<u32>, Vec<u32>) {
    (1, line.0, line.1)
}

fn main() {
    let lines = read_input_file();
    let mut lines_copy = lines.clone();

    let mut total_points = 0;
    for (i, (_, winning_numbers, my_numbers)) in lines.iter().enumerate() {
        let matching_numbers: Vec<u32> = winning_numbers
            .into_iter()
            .filter(|n| my_numbers.contains(&n))
            .map(|&n| n)
            .collect();
        if matching_numbers.len() > 0 {
            total_points += 2u32.pow(matching_numbers.len() as u32 - 1);
        }


        let multiplier = lines_copy[i].0;
        for card_index in i+1..(i+1 + matching_numbers.len()) {
            lines_copy[card_index].0 += multiplier;
        }
    }
    println!("Total cards: {:?}", lines_copy.iter().fold(0, |sum, item| sum + &item.0));
    println!("Total points: {:?}", total_points);
}
