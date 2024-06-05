use std::collections::HashMap;
use std::fs;


#[derive(Debug, Clone, PartialEq)]
enum Direction {
    Above,
    Right,
    Below,
    Left,
    Unknown,
}

#[derive(Clone, Debug, PartialEq)]
enum State {
    NotChecked,
    BeingChecked,
    IsPartOfLoop,
    Surrounded,
    Free,
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

enum ConnectionError {
    IncompatiblePipes,
    NoAvailableConnections,
}

#[derive(Clone, Debug, PartialEq)]
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

    fn insert(&mut self, pipe: Pipe) {
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

    fn set_state_of(&mut self, pipe: &Pipe, new_state: State) {
        let key = Plumping::make_key_for(pipe.x, pipe.y);
        if let Some(pipe) = self.pipes.get(&key) {
            let new_pipe = pipe.with_state(new_state);
            self.insert(new_pipe)
        }
    }
}

struct PlumpingNavigator<'a> {
    plumping: &'a Plumping,
    starting_pipe: &'a Pipe,
    current_pipe: &'a Pipe,
    previous_pipe: &'a Pipe,
}

impl<'a> PlumpingNavigator<'a> {
    fn new(plumping: &'a Plumping, starting_pipe: &'a Pipe) -> PlumpingNavigator<'a> {
        PlumpingNavigator {
            plumping,
            starting_pipe,
            current_pipe: starting_pipe,
            previous_pipe: starting_pipe,
        }
    }
}

impl<'a> Iterator for PlumpingNavigator<'a> {
    type Item = &'a Pipe;

    fn next(&mut self) -> Option<Self::Item> {
        let neighbors = self.plumping.get_connected_neighbors_of(self.current_pipe);
        for neighbor in neighbors {
            if neighbor != self.starting_pipe && neighbor != self.previous_pipe {
                self.previous_pipe = self.current_pipe;
                self.current_pipe = neighbor;
                return Some(self.current_pipe);
            }
        }
        None
    }
}

fn main() {
    const FILE_PATH: &str = "input.txt";
    // const FILE_PATH: &str = "smaller_input.txt";
    // const FILE_PATH: &str = "smaller_input2.txt";
    // const FILE_PATH: &str = "smaller_input3.txt";
    // const FILE_PATH: &str = "smaller_input4.txt";

    let plumping = read_input_file(FILE_PATH);
    let starting_pipe = find_starting_pipe(&plumping);
    let steps_to_farthest_pipe = count_steps_to_farthest_pipe(plumping.clone(), starting_pipe);
    let tiles_inside_loop = count_pipes_surrounded_by_loop(plumping.clone(), starting_pipe);
    println!("Starting position: {:?}", starting_pipe);
    println!("Steps to farthest pipe: {:?}", steps_to_farthest_pipe);
    println!("Tiles inside loop: {:?}", tiles_inside_loop);
}

fn read_input_file(file_path: &str) -> Plumping {
    let contents = fs::read_to_string(file_path).expect("Couldn't read input file");
    let lines = contents.lines().rev();

    let mut plumping = Plumping::new();
    for (y, line) in lines.enumerate() {
        for (x, ch) in line.chars().enumerate() {
            let pipe_type = PipeType::from(ch);
            let pipe = Pipe::new(pipe_type, x as i16, y as i16);
            plumping.insert(pipe);
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

fn count_steps_to_farthest_pipe(plumping: Plumping, starting_pipe: &Pipe) -> u32 {
    let pipes = PlumpingNavigator::new(&plumping, starting_pipe);

    let mut steps = 0;
    for _pipe in pipes {
        steps += 1;
    }
    steps / 2 + 1
}

fn count_pipes_surrounded_by_loop(mut plumping: Plumping, starting_pipe: &Pipe) -> u32 {
    // After an extensive analysis, I discovered that the loop 
    // bifurcates the map into two distinct sections: free and 
    // surrounded. If a pipe contacts the loop on either side, 
    // it can be classified as either free or surrounded. The 
    // primary challenge lies in determining on which side of 
    // the loop the pipe resides.
    
    let mut surrounded_side = Direction::Right; 
    let loop_pipes = PlumpingNavigator::new(&plumping, starting_pipe).peekable();

}






