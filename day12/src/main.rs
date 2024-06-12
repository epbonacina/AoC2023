use std::fs;

mod tree;

fn read_input_file(file_path: &str) -> Vec<(Vec<tree::SpringCondition>, tree::GroupLenghts)> {
    fs::read_to_string(file_path)
        .expect("Couldn't read input file")
        .lines()
        .map(|line| {
            let (conditions, lengths) = line.split_once(" ").unwrap();
            let conditions = conditions
                .chars()
                .map(|ch| tree::SpringCondition::from(ch))
                .collect();
            let lengths = lengths
                .split(",")
                .map(|elem| elem.parse().unwrap())
                .collect();
            (conditions, lengths)
        })
        .collect()
}

fn main() {
    const FILE_PATH: &str = "input.txt";
    // const FILE_PATH: &str = "smaller_input.txt";

    let springs = read_input_file(FILE_PATH);
    let total_records = springs.len();

    let mut count = 0;
    let mut root = tree::Node::new();
    for (record_id, (spring_conditions, group_lengths)) in springs.into_iter().enumerate() {
        print!(
            "Analysing spring record number {} of {}. ({}%)\r",
            record_id + 1,
            total_records,
            (record_id + 1) * 100 / total_records
        );
        count += root.insert(&spring_conditions, group_lengths);
    }
    println!("\nSum of all possible arrangement counts: {}", count);
}
