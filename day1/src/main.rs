use std::fs;

const INPUT_FILEPATH: &str = "input.txt";

fn read_input() -> Vec<String> {
    fs::read_to_string(INPUT_FILEPATH)
        .expect("Couldn't read input file")
        .lines()
        .map(String::from)
        .collect()
}

fn get_digits_and_their_indices(line: &str) -> Vec<[u32; 2]> {
    let mut digits_and_their_indices = Vec::new();
    for (i, ch) in line.chars().enumerate() {
        if ch.is_digit(10) {
            digits_and_their_indices.push([i as u32, ch.to_digit(10).unwrap()]);
        }
    }
    digits_and_their_indices
}

fn get_word_spelled_digits_and_their_indices(line: &str) -> Vec<[u32; 2]> {
    let word_spelled_numbers = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let mut words_found_and_their_indices: Vec<(usize, &str)> = Vec::new();
    for word in word_spelled_numbers {
        let mut matches = line.match_indices(word).collect();
        words_found_and_their_indices.append(&mut matches);
    }

    let mut digits_and_their_indices: Vec<[u32; 2]> = Vec::new();
    for (index, word) in words_found_and_their_indices {
        for (digit, w) in word_spelled_numbers.iter().enumerate() {
            if w.contains(word) {
                digits_and_their_indices.push([index as u32, digit as u32 + 1]);
            }
        }
    }
    digits_and_their_indices
}

fn merge_vectors(vec1: Vec<[u32; 2]>, vec2: Vec<[u32; 2]>) -> Vec<[u32; 2]> {
    let merged_vec = vec1
        .iter()
        .chain(vec2.iter())
        .cloned()
        .collect();
    merged_vec
}

fn main() {
    let lines = read_input();
    let mut sum = 0;
    for line in lines {
        let digits_and_their_indices = get_digits_and_their_indices(&line);
        let word_spelled_digits_and_their_indices =
            get_word_spelled_digits_and_their_indices(&line);
        
        let mut digits = merge_vectors(
            digits_and_their_indices,
            word_spelled_digits_and_their_indices,
        );
        
        digits.sort_by_key(|k| k[0]);

        if digits.len() > 0 {
            let first: String = digits.first().unwrap()[1].to_string();
            let last: String = digits.last().unwrap()[1].to_string();
            let value = first + &last;
            sum += value.parse::<u32>().unwrap();
        }
    }
    println!("{}", sum);
}
