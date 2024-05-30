use std::cmp;
use std::fs;

const FILE_PATH: &str = "input.txt";
// const FILE_PATH: &str = "smaller_input.txt";

const ABOVE: usize = 0;
const RIGHT: usize = 1;
const BELLOW: usize = 2;
const LEFT: usize = 3;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Pipe {
    Start,
    NorthSouth,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
    EastWest,
    Obstructed,
}

impl Pipe {
    fn from(character: char) -> Pipe {
        match character {
            'S' => Pipe::Start,
            '|' => Pipe::NorthSouth,
            'L' => Pipe::NorthEast,
            'J' => Pipe::NorthWest,
            'F' => Pipe::SouthEast,
            '7' => Pipe::SouthWest,
            '-' => Pipe::EastWest,
            _ => Pipe::Obstructed,
        }
    }
}

#[derive(Debug)]
struct Route {
    pipe: Pipe,
    x: usize,
    y: usize,
}

struct Navigator<'a> {
    route_matrix: &'a Vec<Vec<Route>>,
    current_route: &'a Route,
    previous_route: &'a Route,
    initial_x: usize,
    initial_y: usize,
}

impl<'a> Navigator<'a> {
    fn new(route_matrix: &'a Vec<Vec<Route>>, current_route: &'a Route) -> Navigator<'a> {
        Navigator {
            route_matrix,
            current_route,
            previous_route: current_route,
            initial_x: current_route.x,
            initial_y: current_route.y,
        }
    }

    fn get_neighbors(&self) -> [&'a Route; 4] {
        let x = self.current_route.x;
        let y = self.current_route.y;
        let candidates_pos = [
            (x, cmp::min(y + 1, self.route_matrix.len() - 1)),
            (cmp::min(x + 1, self.route_matrix[0].len() - 1), y),
            (x, y.saturating_sub(1)),
            (x.saturating_sub(1), y),
        ];

        let mut neighbors = [&Route {
            pipe: Pipe::Obstructed,
            x: 0,
            y: 0,
        }; 4];

        let indexes = match self.current_route.pipe {
            Pipe::Start => vec![ABOVE, RIGHT, BELLOW, LEFT],
            Pipe::NorthSouth => vec![ABOVE, BELLOW],
            Pipe::NorthEast => vec![ABOVE, RIGHT],
            Pipe::NorthWest => vec![ABOVE, LEFT],
            Pipe::SouthEast => vec![RIGHT, BELLOW],
            Pipe::SouthWest => vec![BELLOW, LEFT],
            Pipe::EastWest => vec![RIGHT, LEFT],
            Pipe::Obstructed => Vec::new(),
        };

        for i in 0..neighbors.len() {
            if indexes.contains(&i) {
                let (x, y) = candidates_pos[i];
                if (x, y) != (self.current_route.x, self.current_route.y)
                    && (x, y) != (self.previous_route.x, self.previous_route.y)
                {
                    neighbors[i] = &self.route_matrix[y][x];
                }
            }
        }
        neighbors
    }
}

impl<'a> Iterator for Navigator<'a> {
    type Item = &'a Route;

    fn next(&mut self) -> Option<Self::Item> {
        let neighbors = self.get_neighbors();
        let possible_connections = [
            (ABOVE, Pipe::NorthSouth),
            (ABOVE, Pipe::SouthEast),
            (ABOVE, Pipe::SouthWest),
            (RIGHT, Pipe::NorthWest),
            (RIGHT, Pipe::SouthWest),
            (RIGHT, Pipe::EastWest),
            (BELLOW, Pipe::NorthSouth),
            (BELLOW, Pipe::NorthEast),
            (BELLOW, Pipe::NorthWest),
            (LEFT, Pipe::NorthEast),
            (LEFT, Pipe::SouthEast),
            (LEFT, Pipe::EastWest),
        ];

        for (i, neighbor) in neighbors.iter().enumerate() {
            if possible_connections.contains(&(i, neighbor.pipe))
                && (neighbor.x, neighbor.y) != (self.initial_x, self.initial_y)
            {
                self.previous_route = self.current_route;
                self.current_route = neighbor;
                return Some(neighbor);
            }
        }
        None
    }
}

fn count_steps_to_farthest_pipe(route_matrix: &Vec<Vec<Route>>, starting_route: &Route) -> u32 {
    let navigator = Navigator::new(route_matrix, starting_route);

    let mut steps = 0;
    for _pipe in navigator {
        steps += 1;
    }
    steps / 2 + 1
}

fn find_starting_route(route_matrix: &Vec<Vec<Route>>) -> &Route {
    for route_line in route_matrix {
        for route in route_line {
            match route.pipe {
                Pipe::Start => return route,
                _ => {}
            }
        }
    }
    panic!("Couldn't find starting position");
}

fn read_input_file() -> Vec<Vec<Route>> {
    fs::read_to_string(FILE_PATH)
        .expect("Couldn't read input file")
        .lines()
        .rev()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, ch)| {
                    let pipe = Pipe::from(ch);
                    Route { pipe, y, x }
                })
                .collect()
        })
        .collect()
}

fn main() {
    let route_matrix = read_input_file();
    let starting_position = find_starting_route(&route_matrix);
    let steps_to_farthest_pipe = count_steps_to_farthest_pipe(&route_matrix, starting_position);
    println!("Starting position: {:?}", starting_position);
    println!("Steps to farthest pipe: {:?}", steps_to_farthest_pipe);
}
