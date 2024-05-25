use std::cmp;
use std::collections::HashMap;
use std::fs;

const FILE_PATH: &str = "input.txt";
// const FILE_PATH: &str = "smaller_input.txt";

const SEED_SECTION_NAME: &str = "seeds";
const SECTION_NAMES: [&str; 8] = [
    SEED_SECTION_NAME,
    "seed-to-soil map",
    "soil-to-fertilizer map",
    "fertilizer-to-water map",
    "water-to-light map",
    "light-to-temperature map",
    "temperature-to-humidity map",
    "humidity-to-location map",
];

fn read_input_file() -> Vec<(String, String)> {
    fs::read_to_string(FILE_PATH)
        .unwrap()
        .split("\n\n")
        .zip(SECTION_NAMES)
        .map(|(s, n)| (s.to_string(), n.to_string()))
        .collect()
}

#[derive(Clone, Debug)]
struct RangeMap {
    src_start: u64,
    dst_start: u64,
    length: u64,
}

impl RangeMap {
    fn dst_end(&self) -> u64 {
        self.dst_start + self.length
    }

    fn src_end(&self) -> u64 {
        self.src_start + self.length
    }

    fn overlaps_with(&self, other: &RangeMap) -> bool {
        self.src_end() > other.src_start && self.src_start < other.src_end()
    }

    fn is_inside(&self, other: &RangeMap) -> bool {
        other.src_end() > self.src_end() && other.src_start <= self.src_start
    }

    fn merge(&self, other: &RangeMap) -> RangeMap {
        RangeMap {
            src_start: self.src_start,
            dst_start: other.dst_start,
            length: self.length,
        }
    }

    fn intersection(&self, other: &RangeMap) -> Option<(RangeMap, RangeMap)> {
        let dst_start = cmp::max(self.dst_start, other.src_start);
        let src_start = self.src_start + (dst_start - self.dst_start);
        let dst_end = cmp::min(self.dst_end(), other.src_end());

        let other_src_start = dst_start;
        let other_dst_start = other.dst_start + (other_src_start - other.src_start);

        if dst_end <= dst_start {
            return None;
        }

        let length = dst_end - dst_start;

        let left_range_map = RangeMap {
            src_start,
            dst_start,
            length,
        };

        let right_range_map = RangeMap {
            src_start: other_src_start,
            dst_start: other_dst_start,
            length,
        };

        Some((left_range_map, right_range_map))
    }

    fn difference(&self, other: &RangeMap) -> Vec<RangeMap> {
        if !self.overlaps_with(other) {
            return vec![self.clone()];
        }

        let mut result = Vec::new();

        if self.src_start < other.src_start {
            result.push(RangeMap {
                src_start: self.src_start,
                dst_start: self.dst_start,
                length: other.src_start - self.src_start,
            });
        }

        if self.src_end() > other.src_end() {
            result.push(RangeMap {
                src_start: other.src_end(),
                dst_start: self.dst_start + (other.src_end() - self.src_start),
                length: self.src_end() - other.src_end(),
            });
        }

        result
    }
}

#[derive(Clone, Debug)]
struct RangeSet {
    range_maps: Vec<RangeMap>,
}

impl RangeSet {
    fn new() -> Self {
        RangeSet {
            range_maps: Vec::new(),
        }
    }

    fn insert(&self, new_range_map: &RangeMap) -> RangeSet {
        let mut overlaps = false;
        let mut new_ranges = Vec::new();
        for range_map in &self.range_maps {
            if new_range_map.is_inside(range_map) {
                return self.clone();
            }
            if range_map.overlaps_with(new_range_map) {
                overlaps = true;
                new_ranges.extend(range_map.difference(new_range_map));
            }
        }

        let mut new_range_set = self.clone();
        if overlaps {
            for new_range in new_ranges {
                new_range_set = new_range_set.insert(&new_range);
            }
        } else {
            new_range_set.range_maps.push(new_range_map.clone());
        }

        new_range_set
    }

    fn intersection(&self, other: &RangeSet) -> (RangeSet, RangeSet) {
        let mut intersection = RangeSet::new();
        let mut other_intersection = RangeSet::new();

        for range_map in &self.range_maps {
            for other_range_map in &other.range_maps {
                match range_map.intersection(&other_range_map) {
                    None => {}
                    Some((range, other_range)) => {
                        intersection = intersection.insert(&range);
                        other_intersection = other_intersection.insert(&other_range);
                    }
                }
            }
        }

        (intersection, other_intersection)
    }

    fn update_intersection_with(&self, other: &RangeSet) -> RangeSet {
        let mut result = RangeSet::new();
        let (intersection, other_intersection) = self.intersection(other);

        for (range_map, other_range_map) in intersection
            .range_maps
            .iter()
            .zip(other_intersection.range_maps)
        {
            let new_range_map = range_map.merge(&other_range_map);
            result = result.insert(&new_range_map);
        }

        for range_map in &self.range_maps {
            result = result.insert(&range_map);
        }

        result
    }
}

fn get_seed_range_set_pt1(sections: &Vec<(String, String)>) -> RangeSet {
    let mut result = RangeSet::new();
    let sections: Vec<String> = sections.iter().map(|(s, _)| s.to_string()).collect();
    let (_, line) = sections[0].split_once(":").unwrap();
    let values: Vec<u64> = line
        .split_whitespace()
        .map(|i| i.parse().unwrap())
        .collect();
    for value in values {
        let new_range_map = RangeMap {
            dst_start: value,
            src_start: value,
            length: 1,
        };
        result = result.insert(&new_range_map);
    }
    result
}

fn get_seed_range_set(sections: &Vec<(String, String)>) -> RangeSet {
    let mut result = RangeSet::new();
    let sections: Vec<String> = sections.iter().map(|(s, _)| s.to_string()).collect();
    let (_, line) = sections[0].split_once(":").unwrap();
    let values: Vec<u64> = line
        .split_whitespace()
        .map(|i| i.parse().unwrap())
        .collect();
    for (i, value) in values.clone().into_iter().enumerate().step_by(2) {
        let new_range_map = RangeMap {
            dst_start: value,
            src_start: value,
            length: values[i + 1],
        };
        result = result.insert(&new_range_map);
    }
    result
}

fn get_range_set(section: &str) -> RangeSet {
    let mut result = RangeSet::new();
    let lines = section.split("\n").filter(|s| !s.is_empty()).skip(1);
    for line in lines {
        let values: Vec<u64> = line
            .split_whitespace()
            .map(|i| i.parse().unwrap())
            .collect();
        let new_range_map = RangeMap {
            dst_start: values[0],
            src_start: values[1],
            length: values[2],
        };
        result = result.insert(&new_range_map);
    }
    result
}

fn get_range_sets_for_pt1(sections: &Vec<(String, String)>) -> HashMap<String, RangeSet> {
    let mut range_sets = HashMap::new();

    let seed_range_set = get_seed_range_set_pt1(&sections);
    range_sets.insert(SEED_SECTION_NAME.to_string(), seed_range_set);

    for (section, section_name) in sections.into_iter().skip(1) {
        let range_set = get_range_set(&section);
        range_sets.insert(section_name.to_string(), range_set);
    }

    range_sets
}

fn get_range_sets_for_pt2(sections: &Vec<(String, String)>) -> HashMap<String, RangeSet> {
    let mut range_sets = HashMap::new();

    let seed_range_set = get_seed_range_set(&sections);
    range_sets.insert(SEED_SECTION_NAME.to_string(), seed_range_set);

    for (section, section_name) in sections.into_iter().skip(1) {
        let range_set = get_range_set(&section);
        range_sets.insert(section_name.to_string(), range_set);
    }

    range_sets
}

fn get_lowest_location(range_sets: HashMap<String, RangeSet>) -> u64 {
    let mut base_range_set = range_sets.get(SECTION_NAMES[0]).unwrap().clone();
    for section_name in SECTION_NAMES.iter().skip(1) {
        let next_range_set = range_sets.get(section_name.to_owned()).unwrap();
        base_range_set = base_range_set.update_intersection_with(next_range_set);
    }

    let mut lowest_location = u64::MAX;
    for range_map in base_range_set.range_maps {
        let candidate = range_map.dst_start;
        if candidate < lowest_location {
            lowest_location = candidate;
        }
    }
    lowest_location
}

fn main() {
    let sections = read_input_file();
    let range_sets_pt1 = get_range_sets_for_pt1(&sections);
    let range_sets_pt2 = get_range_sets_for_pt2(&sections);

    let lowest_location_pt1 = get_lowest_location(range_sets_pt1);
    let lowest_location_pt2 = get_lowest_location(range_sets_pt2);
    println!("Lowest location for part one: {}", lowest_location_pt1);
    println!("Lowest location for part two: {}", lowest_location_pt2);
}






