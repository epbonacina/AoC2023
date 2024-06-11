use std::fs;

mod tree;

fn read_input_file(file_path: &str) -> Vec<(tree::SpringConditions, tree::GroupLenghts)> {
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
    let total_trees = springs.len();
    let mut count = 0;
    for (i, (spring_conditions, group_lengths)) in springs.into_iter().enumerate() {
        print!("Analysing tree number {} of {}. ({}%)\r", i+1, total_trees, (i+1)*100/total_trees);
        let mut root = tree::Node::new();

        for spring_condition in spring_conditions {
            root.insert(spring_condition);
        }

        for leave in root.get_leaves() {
            if leave.group_lengths.eq(&group_lengths) {
                count += 1;
            }
        }

    }
    println!("\nSum of all possible arrangement counts: {}", count);
}
