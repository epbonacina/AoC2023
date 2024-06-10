use std::fs;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Galaxy {
    x: usize,
    y: usize,
}

impl Galaxy {
    fn get_distance_to(&self, other: &Galaxy) -> u64 {
        let y_distance = (self.y as i64 - other.y as i64).abs();
        let x_distance = (self.x as i64 - other.x as i64).abs();
        (x_distance + y_distance) as u64
    }
}

#[derive(Clone, Debug)]
struct Universe {
    galaxies: Vec<Galaxy>,
    age: u64,
}

impl Universe {
    fn new(age: u64) -> Universe {
        Universe {
            galaxies: Vec::new(),
            age,
        }
    }

    fn insert(&self, galaxy: Galaxy) -> Universe {
        let mut new_universe = self.clone();
        new_universe.galaxies.push(galaxy);
        new_universe
    }

    fn expand(&self) -> Universe {
        let expanded_universe = self.expand_rows();
        expanded_universe.expand_columns()
    }

    fn expand_rows(&self) -> Universe {
        let mut expanded_universe = Universe::new(self.age);

        let mut expansion = 0;
        let mut previous_y = 0;
        for galaxy in self.galaxies.iter() {
            expansion += (galaxy.y - previous_y).saturating_sub(1);
            let new_galaxy = Galaxy {
                y: galaxy.y + expansion * self.age as usize,
                x: galaxy.x,
            };
            expanded_universe.galaxies.push(new_galaxy);
            previous_y = galaxy.y;
        }
        expanded_universe
    }

    fn expand_columns(&self) -> Universe {
        let mut universe = self.clone();
        universe.galaxies.sort_by_key(|galaxy| galaxy.x);

        let mut expanded_universe = Universe::new(self.age);

        let mut expansion = 0;
        let mut previous_x = 0;
        for galaxy in universe.galaxies.iter() {
            expansion += (galaxy.x - previous_x).saturating_sub(1);
            let new_galaxy = Galaxy {
                y: galaxy.y,
                x: galaxy.x + expansion * self.age as usize,
            };
            expanded_universe.galaxies.push(new_galaxy);
            previous_x = galaxy.x;
        }
        expanded_universe
    }
}

fn main() {
    const FILE_PATH: &str = "input.txt";
    // const FILE_PATH: &str = "smaller_input.txt";

    let universe_age_multiplier = 1;
    let mut universe = read_input(FILE_PATH, universe_age_multiplier);
    universe = universe.expand();
    println!(
        "Sum of relative distances for part one: {}",
        add_all_relative_distances(&universe)
    );

    let universe_age_multiplier = 999_999;
    let mut universe = read_input(FILE_PATH, universe_age_multiplier);
    universe = universe.expand();
    println!(
        "Sum of relative distances for part two: {}",
        add_all_relative_distances(&universe)
    );
}

fn read_input(file_path: &str, age: u64) -> Universe {
    let content = fs::read_to_string(file_path).expect("Couldn't read input file");

    let mut result = Universe::new(age);
    for (y, line) in content.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if ch == '#' {
                result = result.insert(Galaxy { x, y });
            }
        }
    }
    result
}

fn add_all_relative_distances(universe: &Universe) -> u64 {
    let mut result = 0;

    let mut i = 0;
    while i < universe.galaxies.len() - 1 {
        for j in (i + 1)..universe.galaxies.len() {
            result += universe.galaxies[i].get_distance_to(&universe.galaxies[j]);
        }
        i += 1;
    }
    result
}
