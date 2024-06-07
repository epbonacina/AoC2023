use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::fs;

#[derive(Debug, Clone, PartialEq)]
enum Direction {
    Above,
    Right,
    Below,
    Left,
    Unknown,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum State {
    NotChecked,
    ComposingLoop,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum PipeType {
    Start,
    NorthSouth,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
    EastWest,
    Obstructed,
}

impl PipeType {
    fn from(character: char) -> PipeType {
        match character {
            'S' => PipeType::Start,
            '|' => PipeType::NorthSouth,
            'L' => PipeType::NorthEast,
            'J' => PipeType::NorthWest,
            'F' => PipeType::SouthEast,
            '7' => PipeType::SouthWest,
            '-' => PipeType::EastWest,
            _ => PipeType::Obstructed,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Pipe {
    pipe_type: PipeType,
    x: i16,
    y: i16,
    state: State,
}

impl Pipe {
    fn new(pipe_type: PipeType, x: i16, y: i16) -> Pipe {
        Pipe {
            pipe_type,
            x,
            y,
            state: State::NotChecked,
        }
    }

    fn is_corner(&self) -> bool {
        match self.pipe_type {
            PipeType::NorthEast
            | PipeType::NorthWest
            | PipeType::SouthEast
            | PipeType::SouthWest => true,
            _ => false,
        }
    }

    fn is_connected_to(&self, other: &Pipe) -> bool {
        let direction = self.get_direction_towards(other);
        let connectable_directions = self.get_connectable_directions();
        let other_direction = other.get_direction_towards(self);
        let other_connectable_directions = other.get_connectable_directions();
        connectable_directions.contains(&direction)
            && other_connectable_directions.contains(&other_direction)
    }

    fn get_connectable_directions(&self) -> Vec<Direction> {
        match self.pipe_type {
            PipeType::Start => vec![
                Direction::Above,
                Direction::Right,
                Direction::Below,
                Direction::Left,
            ],
            PipeType::NorthSouth => vec![Direction::Above, Direction::Below],
            PipeType::NorthEast => vec![Direction::Above, Direction::Right],
            PipeType::NorthWest => vec![Direction::Above, Direction::Left],
            PipeType::EastWest => vec![Direction::Right, Direction::Left],
            PipeType::SouthEast => vec![Direction::Right, Direction::Below],
            PipeType::SouthWest => vec![Direction::Below, Direction::Left],
            PipeType::Obstructed => vec![],
        }
    }

    fn get_direction_towards(&self, other: &Pipe) -> Direction {
        match (other.x, other.y) {
            (x, y) if x == self.x && y == self.y + 1 => Direction::Above,
            (x, y) if x == self.x + 1 && y == self.y => Direction::Right,
            (x, y) if x == self.x && y == self.y - 1 => Direction::Below,
            (x, y) if x == self.x - 1 && y == self.y => Direction::Left,
            _ => Direction::Unknown,
        }
    }

    fn with_state(&self, new_state: State) -> Pipe {
        let mut pipe = self.clone();
        pipe.state = new_state;
        pipe
    }
}

#[derive(Clone, Debug)]
struct Plumping {
    pipes: HashMap<String, Pipe>,
}

impl Plumping {
    fn new() -> Plumping {
        Plumping {
            pipes: HashMap::new(),
        }
    }

    fn insert_or_update(&mut self, pipe: Pipe) {
        let key = Plumping::make_key_for(pipe.x, pipe.y);
        self.pipes.insert(key, pipe);
    }

    fn make_key_for(x: i16, y: i16) -> String {
        format!("x={},y={}", x, y)
    }

    fn get_neighbors_of(&self, pipe: &Pipe) -> Vec<&Pipe> {
        let (x, y) = (pipe.x, pipe.y);
        let neighbors_coords = [(x, y + 1), (x + 1, y), (x, y - 1), (x - 1, y)];

        let mut neighbors = Vec::new();
        for (ix, iy) in neighbors_coords {
            let key = Plumping::make_key_for(ix, iy);
            if let Some(pipe) = self.pipes.get(&key) {
                neighbors.push(pipe)
            }
        }
        neighbors
    }

    fn get_connected_neighbors_of(&self, pipe: &Pipe) -> Vec<&Pipe> {
        let neighbors = self.get_neighbors_of(pipe);
        neighbors
            .into_iter()
            .filter(|neighbor| pipe.is_connected_to(neighbor))
            .collect()
    }
}

#[derive(Debug, Clone)]
struct PlumpingNavigator<'a> {
    plumping: &'a Plumping,
    current_pipe: &'a Pipe,
    previous_pipe: &'a Pipe,
}

impl<'a> PlumpingNavigator<'a> {
    fn new(plumping: &'a Plumping, starting_pipe: &'a Pipe) -> PlumpingNavigator<'a> {
        PlumpingNavigator {
            plumping,
            current_pipe: starting_pipe,
            previous_pipe: starting_pipe,
        }
    }
}

impl<'a> Iterator for PlumpingNavigator<'a> {
    type Item = Pipe;

    fn next(&mut self) -> Option<Self::Item> {
        let neighbors = self.plumping.get_connected_neighbors_of(self.current_pipe);

        for neighbor in neighbors {
            if neighbor == self.previous_pipe {
                continue;
            }
            if self.current_pipe.pipe_type == PipeType::Start
                && self.previous_pipe.pipe_type != PipeType::Start
            {
                return None;
            }
            self.previous_pipe = self.current_pipe;
            self.current_pipe = neighbor;
            return Some(self.previous_pipe.clone());
        }
        None
    }
}

fn main() {
    // const FILE_PATH: &str = "input.txt";
    // const FILE_PATH: &str = "smaller_input.txt";
    // const FILE_PATH: &str = "smaller_input2.txt";
    // const FILE_PATH: &str = "smaller_input3.txt";
    const FILE_PATH: &str = "smaller_input4.txt";

    let plumping = read_input_file(FILE_PATH);
    let starting_pipe = find_starting_pipe(&plumping);
    let steps_to_farthest_pipe = count_steps_to_farthest_pipe(&plumping, starting_pipe);
    let surrounded_pipes_count = count_pipes_surrounded_by_loop(&plumping, starting_pipe);
    println!("Starting position: {:?}", starting_pipe);
    println!("Steps to farthest pipe: {:?}", steps_to_farthest_pipe);
    println!("Surrounded pipes count: {:?}", surrounded_pipes_count);
}

fn read_input_file(file_path: &str) -> Plumping {
    let contents = fs::read_to_string(file_path).expect("Couldn't read input file");
    let lines = contents.lines().rev();

    let mut plumping = Plumping::new();
    for (y, line) in lines.enumerate() {
        for (x, ch) in line.chars().enumerate() {
            let pipe_type = PipeType::from(ch);
            let pipe = Pipe::new(pipe_type, x as i16, y as i16);
            plumping.insert_or_update(pipe);
        }
    }
    plumping
}

fn find_starting_pipe(plumping: &Plumping) -> &Pipe {
    plumping
        .pipes
        .values()
        .find(|&pipe| pipe.pipe_type == PipeType::Start)
        .expect("Couldn't find starting pipe")
}

fn count_steps_to_farthest_pipe(plumping: &Plumping, starting_pipe: &Pipe) -> u32 {
    let pipes = PlumpingNavigator::new(plumping, starting_pipe);

    let mut steps = 0;
    for _pipe in pipes {
        steps += 1;
    }
    steps / 2 + 1
}

fn count_pipes_surrounded_by_loop(plumping: &Plumping, starting_pipe: &Pipe) -> (u32, u32) {
    // After an extensive analysis, I discovered that the loop
    // bifurcates the map into two distinct sections: free and
    // surrounded. If a pipe contacts the loop on either side,
    // it can be classified as either free or surrounded. The
    // primary challenge lies in determining on which side of
    // the loop the pipe resides.

    let plumping = set_loop_pipes(&plumping, starting_pipe);
    let (mut group1, mut group2) = separate_pipes_by_side(&plumping, starting_pipe);
    println!(
        "group1.len = {:?}, group2.len = {:?}",
        group1.len(),
        group2.len()
    );
    println!("{:?}", group2);
    println!();
    println!();
    println!();
    println!("{:?}", group1);
    expand_group_to_catch_orphan_pipes(&plumping, &mut group1);
    expand_group_to_catch_orphan_pipes(&plumping, &mut group2);
    (group1.len() as u32, group2.len() as u32)
}

fn set_loop_pipes(plumping: &Plumping, starting_pipe: &Pipe) -> Plumping {
    let loop_pipes: Vec<Pipe> = PlumpingNavigator::new(&plumping, starting_pipe).collect();
    let mut new_plumping = plumping.clone();
    for pipe in loop_pipes {
        new_plumping.insert_or_update(pipe.with_state(State::ComposingLoop));
    }
    new_plumping
}

fn separate_pipes_by_side<'a>(
    plumping: &'a Plumping,
    starting_pipe: &'a Pipe,
) -> (Vec<Pipe>, Vec<Pipe>) {
    let mut loop_pipes = PlumpingNavigator::new(&plumping, starting_pipe).peekable();
    let mut side_1 = HashSet::new();
    let mut side_2 = HashSet::new();

    while let Some(loop_pipe) = loop_pipes.next() {
        if let Some(next_pipe) = loop_pipes.peek() {
            let side_1_direction = determine_side_1_direction(&loop_pipe, &next_pipe);
            let neighbors = plumping.get_neighbors_of(&loop_pipe);
            let next_neighbors = plumping.get_neighbors_of(&next_pipe);
            let directions_towards_neighbors =
                get_directions_towards_neighbors(&neighbors, &loop_pipe);

            if loop_pipe.is_corner() {
                // println!("loop_pipe: {:?}", loop_pipe);
                // println!("next_pipe: {:?}", next_pipe);
                // println!("side direction: {:?}", side_1_direction);
                if directions_towards_neighbors.iter().any(|(_, direction)| direction == &side_1_direction) {
                    for (neighbor, _) in directions_towards_neighbors {
                        side_1.insert(neighbor.clone());
                        // println!("neighbor: {:?}", neighbor);
                        // println!("Inserted");
                    }
                }
                // } else {
                //     for (neighbor, _) in directions_towards_neighbors {
                //         side_2.insert(neighbor.clone());
                //         println!("neighbor: {:?}", neighbor);
                //         println!("Inserted2");
                //     }
                // }
                continue;
            }
            for (neighbor, direction) in directions_towards_neighbors {
                if loop_pipe.pipe_type != PipeType::Start {
                    // println!("loop_pipe: {:?}", loop_pipe);
                    // println!("next_pipe: {:?}", next_pipe);
                    // println!("neighbor: {:?}", neighbor);
                    // println!("side direction: {:?}", side_1_direction);
                    // println!("neighbor_direction: {:?}", direction);
                    if direction == side_1_direction {
                        if !side_2.contains(neighbor) {
                            side_1.insert(neighbor.clone());
                            // println!("Inserted");
                        }
                    } else {
                        if !side_1.contains(neighbor) {
                            side_2.insert(neighbor.clone());
                            // println!("Inserted2");
                        }
                    }
                    // println!();
                    // println!();
                }

                // let directions_towards_next_neighbors =
                //     get_directions_towards_neighbors(&next_neighbors, &next_pipe);
                // for (neighbor, direction) in directions_towards_next_neighbors {
                //     if direction == side_1_direction {
                //         side_1.insert(neighbor.clone());
                //     }
                // }
            }
        }
    }
    (Vec::from_iter(side_1), Vec::from_iter(side_2))
}

fn determine_side_1_direction(current_loop_pipe: &Pipe, next_loop_pipe: &Pipe) -> Direction {
    let side_1_direction = match current_loop_pipe.get_direction_towards(next_loop_pipe) {
        Direction::Above => Direction::Left,
        Direction::Right => Direction::Above,
        Direction::Below => Direction::Right,
        Direction::Left => Direction::Below,
        _ => panic!("Unknown direction"),
    };
    side_1_direction
}

fn get_directions_towards_neighbors<'a>(
    neighbors: &'a Vec<&Pipe>,
    pipe: &Pipe,
) -> Vec<(&'a Pipe, Direction)> {
    let directions_towards_neighbors = neighbors
        .iter()
        .filter(|neighbor| neighbor.state != State::ComposingLoop)
        .map(|&neighbor| {
            let direction = pipe.get_direction_towards(neighbor);
            (neighbor, direction)
        })
        .collect();
    directions_towards_neighbors
}

fn expand_group_to_catch_orphan_pipes<'a>(plumping: &'a Plumping, group: &mut Vec<Pipe>) {
    let mut queue: VecDeque<Pipe> = VecDeque::new();
    let mut visited: HashSet<Pipe> = HashSet::new();

    for pipe in group.iter() {
        queue.push_back(pipe.clone());
        visited.insert(pipe.clone());
    }

    while let Some(pipe) = queue.pop_front() {
        let neighbors = plumping.get_neighbors_of(&pipe);
        let neighbors = neighbors
            .iter()
            .filter(|neighbor| neighbor.state != State::ComposingLoop);

        for &neighbor in neighbors {
            if visited.insert(neighbor.clone()) {
                queue.push_back(neighbor.clone());
                group.push(neighbor.clone());
            }
        }
    }
}
