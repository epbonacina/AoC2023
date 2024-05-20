use std::collections::HashMap;
use std::fs;

const FILE_PATH: &str = "input.txt";
// const FILE_PATH: &str = "smaller_input.txt";
const SECTION_NAMES: [&str; 7] = [
        "seed-to-soil map",
        "soil-to-fertilizer map",
        "fertilizer-to-water map",
        "water-to-light map",
        "light-to-temperature map",
        "temperature-to-humidity map",
        "humidity-to-location map",
    ];


fn read_input() -> Vec<String> {
    fs::read_to_string(FILE_PATH)
        .expect("Couldn't read input file")
        .split("\n\n")
        .map(String::from)
        .collect::<Vec<String>>()
}

fn get_seed_ids(lines: &Vec<String>) -> Vec<u32> {
    lines[0]
        .split_once(":")
        .unwrap()
        .1
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect()
}

fn get_section_mappings(section_name: &str, sections: &Vec<String>) -> HashMap<u32, u32> {
    let section = get_section_by_name(section_name, sections);
    let mappings = &section
        .split_once(":")
        .unwrap()
        .1
        .split("\n")
        .filter(|s| !s.is_empty())
        .collect::<Vec<&str>>();

    let mut resulting_map = HashMap::new();
    for mapping in mappings {
        let values: Vec<String> = mapping.split_whitespace().map(String::from).collect();
        let dest_range_start: u32 = values[0].parse().unwrap();
        let src_range_start: u32 = values[1].parse().unwrap();
        let range_len: u32 = values[2].parse().unwrap();

        for i in 0..range_len {
            resulting_map.insert(src_range_start + i, dest_range_start + i);
        }
    }

    resulting_map
}

fn get_section_by_name(section_name: &str, sections: &Vec<String>) -> String {
    sections
        .iter()
        .filter(|s| s.contains(section_name))
        .map(String::from)
        .collect()
}


fn find_seed_location(seed_id: u32, mappings: &HashMap<&str, HashMap<u32, u32>>) -> u32 {
    let mut key = seed_id;
    for section in SECTION_NAMES {
        let mapping = mappings.get(section).unwrap();
        match mapping.get(&key) {
            Some(k) => key = *k,
            None => {},
        }
    }
    key
}

fn main() {
    let sections = read_input();
    let seed_ids = get_seed_ids(&sections);
    let mut mappings_of_each_section = HashMap::new();

    for section_name in SECTION_NAMES {
        let mappings = get_section_mappings(&section_name, &sections);
        mappings_of_each_section.insert(section_name, mappings);
    }

    let mut lowest_location = i32::MAX;
    for seed_id in seed_ids {
        let location_number = find_seed_location(seed_id, &mappings_of_each_section);
        if (location_number as i32) < lowest_location {
            lowest_location = location_number as i32;
        }
    }
    println!("{}", lowest_location);
}








