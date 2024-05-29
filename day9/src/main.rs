use std::fs;

const FILE_PATH: &str = "input.txt";
// const FILE_PATH: &str = "smaller_input.txt";

fn read_input_file() -> Vec<Vec<i64>> {
    fs::read_to_string(FILE_PATH)
        .expect("Couldn't read input file")
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|value| value.parse().unwrap())
                .collect()
        })
        .collect()
}

fn extrapolate(values: &Vec<i64>) -> i64 {
    if values.iter().all(|&elem| elem == 0) {
        return 0;
    }
    let differences = get_differences(&values);
    values.last().unwrap() + extrapolate(&differences)
}

fn extrapolate_backwards(values: &Vec<i64>) -> i64 {
    if values.iter().all(|&elem| elem == 0) {
        return 0;
    }
    let differences = get_differences(&values);
    values.first().unwrap() - extrapolate_backwards(&differences)
}

fn get_differences(values: &Vec<i64>) -> Vec<i64> {
    let mut new_values = Vec::new();
    let mut iter = values.iter().enumerate().peekable();
    while let Some((i, value)) = iter.next() {
        if iter.peek().is_none() {
            break;
        }
        new_values.push(values[i + 1] - value);
    }
    new_values
}

fn sum_extrapolations(lines: &Vec<Vec<i64>>) -> i64 {
    let mut result = 0;
    for line in lines {
        result += extrapolate(line);
    }
    result
}

fn sum_backward_extrapolations(lines: &Vec<Vec<i64>>) -> i64 {
    let mut result = 0;
    for line in lines {
        result += extrapolate_backwards(line);
    }
    result
}

fn main() {
    let lines = read_input_file();
    let result = sum_extrapolations(&lines);
    println!("Part one: {result:?}");
    let result = sum_backward_extrapolations(&lines);
    println!("Part two: {result:?}");
}
