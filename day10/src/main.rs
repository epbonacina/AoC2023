use std::collections::HashMap;
use std::cmp;
use std::fs;

const FILE_PATH: &str = "input.txt";
// const FILE_PATH: &str = "smaller_input.txt";

const ABOVE: usize = 0;
const RIGHT: usize = 1;
const BELOW: usize = 2;
const LEFT: usize = 3;
const NOT_SURROUNDED: i32 = 0;
const SURROUNDED: i32 = 1;

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

#[derive(Clone, Debug, PartialEq)]
struct Route {
    pipe: Pipe,
    x: usize,
    y: usize,
}

impl Route {
    fn get_neighbors<'a>(&'a self, route_matrix: &'a [Vec<Route>]) -> Vec<&'a Route> {
        let candidates_pos = [
            (self.x, cmp::min(self.y + 1, route_matrix.len() - 1)),
            (cmp::min(self.x + 1, route_matrix[0].len() - 1), self.y),
            (self.x, self.y.saturating_sub(1)),
            (self.x.saturating_sub(1), self.y),
        ];

        let mut neighbors = Vec::new();
        for (candidate_x, candidate_y) in candidates_pos {
            if (candidate_x, candidate_y) != (self.x, self.y) {
                neighbors.push(&route_matrix[candidate_y][candidate_x]);
            }
        }
        neighbors
    }
}

struct Navigator<'a> {
    route_matrix: &'a [Vec<Route>],
    current_route: &'a Route,
    previous_route: &'a Route,
    initial_x: usize,
    initial_y: usize,
}

impl<'a> Navigator<'a> {
    fn new(route_matrix: &'a [Vec<Route>], current_route: &'a Route) -> Navigator<'a> {
        Navigator {
            route_matrix,
            current_route,
            previous_route: current_route,
            initial_x: current_route.x,
            initial_y: current_route.y,
        }
    }

    fn get_connected_neighbors(&self) -> [&'a Route; 4] {
        let neighbors = self.current_route.get_neighbors(self.route_matrix);

        let pipe_indexes = match self.current_route.pipe {
            Pipe::Start => vec![ABOVE, RIGHT, BELOW, LEFT],
            Pipe::NorthSouth => vec![ABOVE, BELOW],
            Pipe::NorthEast => vec![ABOVE, RIGHT],
            Pipe::NorthWest => vec![ABOVE, LEFT],
            Pipe::SouthEast => vec![RIGHT, BELOW],
            Pipe::SouthWest => vec![BELOW, LEFT],
            Pipe::EastWest => vec![RIGHT, LEFT],
            Pipe::Obstructed => Vec::new(),
        };

        let mut connected_neighbors = [&Route {
            pipe: Pipe::Obstructed,
            x: 0,
            y: 0,
        }; 4];

        for i in pipe_indexes {
            match neighbors.get(i) {
                Some(neighbor) => {
                    if (neighbor.x, neighbor.y) != (self.previous_route.x, self.previous_route.y) {
                        connected_neighbors[i] = neighbor;
                    }
                }
                None => {}
            }
        }
        connected_neighbors
    }
}

impl<'a> Iterator for Navigator<'a> {
    type Item = &'a Route;

    fn next(&mut self) -> Option<Self::Item> {
        let neighbors = self.get_connected_neighbors();
        let possible_connections = [
            (ABOVE, Pipe::NorthSouth),
            (ABOVE, Pipe::SouthEast),
            (ABOVE, Pipe::SouthWest),
            (RIGHT, Pipe::NorthWest),
            (RIGHT, Pipe::SouthWest),
            (RIGHT, Pipe::EastWest),
            (BELOW, Pipe::NorthSouth),
            (BELOW, Pipe::NorthEast),
            (BELOW, Pipe::NorthWest),
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

fn count_steps_to_farthest_pipe(route_matrix: &[Vec<Route>], starting_route: &Route) -> u32 {
    let navigator = Navigator::new(route_matrix, starting_route);

    let mut steps = 0;
    for _pipe in navigator {
        steps += 1;
    }
    steps / 2 + 1
}

fn there_is_a_wall_above(
    route: &Route,
    loop_routes: &HashMap<String, &Route>,
    _route_matrix: &[Vec<Route>],
) -> bool {
    let ys = 0..route.y;

    let mut found_a_east_pointing_pipe = false;
    let mut found_a_west_pointing_pipe = false;
    for y in ys {
        let key = get_key(route.x, y);
        let candidate_route = match loop_routes.get(&key) {
            Some(value) => value,
            None => &&Route {
                pipe: Pipe::Obstructed,
                x: 0,
                y: 0,
            },
        };
        match candidate_route.pipe {
            Pipe::EastWest => return true,
            Pipe::NorthEast | Pipe::SouthEast => found_a_east_pointing_pipe = true,
            Pipe::NorthWest | Pipe::SouthWest => found_a_west_pointing_pipe = true,
            _ => {}
        }
    }
    found_a_east_pointing_pipe && found_a_west_pointing_pipe
}

fn there_is_a_wall_to_the_right(
    route: &Route,
    loop_routes: &HashMap<String, &Route>,
    route_matrix: &[Vec<Route>],
) -> bool {
    let xs = route.x..route_matrix[0].len();

    let mut found_a_north_pointing_pipe = false;
    let mut found_a_south_pointing_pipe = false;
    for x in xs {
        let key = get_key(x, route.y);
        let candidate_route = match loop_routes.get(&key) {
            Some(value) => value,
            None => &&Route {
                pipe: Pipe::Obstructed,
                x: 0,
                y: 0,
            },
        };
        match candidate_route.pipe {
            Pipe::NorthSouth => return true,
            Pipe::NorthEast | Pipe::NorthWest => found_a_north_pointing_pipe = true,
            Pipe::SouthEast | Pipe::SouthWest => found_a_south_pointing_pipe = true,
            _ => {}
        }
    }
    found_a_north_pointing_pipe && found_a_south_pointing_pipe
}

fn there_is_a_wall_below(
    route: &Route,
    loop_routes: &HashMap<String, &Route>,
    route_matrix: &[Vec<Route>],
) -> bool {
    let ys = route.y..route_matrix.len();

    let mut found_a_east_pointing_pipe = false;
    let mut found_a_west_pointing_pipe = false;
    for y in ys {
        let key = get_key(route.x, y);
        let candidate_route = match loop_routes.get(&key) {
            Some(value) => value,
            None => &&Route {
                pipe: Pipe::Obstructed,
                x: 0,
                y: 0,
            },
        };
        match candidate_route.pipe {
            Pipe::EastWest => return true,
            Pipe::NorthEast | Pipe::SouthEast => found_a_east_pointing_pipe = true,
            Pipe::NorthWest | Pipe::SouthWest => found_a_west_pointing_pipe = true,
            _ => {}
        }
    }
    found_a_east_pointing_pipe && found_a_west_pointing_pipe
}

fn there_is_a_wall_to_the_left(
    route: &Route,
    loop_routes: &HashMap<String, &Route>,
    _route_matrix: &[Vec<Route>],
) -> bool {
    let xs = 0..route.x;

    let mut found_a_north_pointing_pipe = false;
    let mut found_a_south_pointing_pipe = false;
    for x in xs {
        let key = get_key(x, route.y);
        let candidate_route = match loop_routes.get(&key) {
            Some(value) => value,
            None => &&Route {
                pipe: Pipe::Obstructed,
                x: 0,
                y: 0,
            },
        };
        match candidate_route.pipe {
            Pipe::EastWest => return true,
            Pipe::NorthEast | Pipe::SouthEast => found_a_north_pointing_pipe = true,
            Pipe::NorthWest | Pipe::SouthWest => found_a_south_pointing_pipe = true,
            _ => {}
        }
    }

    found_a_north_pointing_pipe && found_a_south_pointing_pipe
}
fn is_surrounded_by_walls(
    route: &Route,
    loop_routes: &HashMap<String, &Route>,
    route_matrix: &[Vec<Route>],
) -> bool {
    there_is_a_wall_above(route, loop_routes, route_matrix)
        && there_is_a_wall_to_the_right(route, loop_routes, route_matrix)
        && there_is_a_wall_below(route, loop_routes, route_matrix)
        && there_is_a_wall_to_the_left(route, loop_routes, route_matrix)
}

fn reset_tiles_that_have_unsurrounded_neighbors(
    routes_conditions: &Vec<Vec<i32>>,
) -> Vec<Vec<i32>> {
    let mut new_routes_conditions = routes_conditions.clone();
    for (y, line) in routes_conditions.iter().enumerate() {
        for (x, _condition) in line.iter().enumerate() {
            let candidates_pos = [
                (x, cmp::min(y + 1, routes_conditions.len() - 1)),
                (cmp::min(x + 1, routes_conditions[0].len() - 1), y),
                (x, y.saturating_sub(1)),
                (x.saturating_sub(1), y),
            ];
            if candidates_pos
                .into_iter()
                .any(|(ix, iy)| routes_conditions[iy][ix] == 0)
            {
                new_routes_conditions[y][x] = 0;
            }
        }
    }
    new_routes_conditions
}

fn get_key(x: usize, y: usize) -> String {
    format!("x={x},y={y}")
}

fn count_tiles_inside_loop(route_matrix: &[Vec<Route>], starting_route: &Route) -> u32 {
    let all_routes: HashMap<String, &Route> = route_matrix.iter().flatten().map(|elem| {
        let key = get_key(elem.x, elem.y);
        (key, elem)
    }).collect();

    let loop_routes: HashMap<String, &Route> = Navigator::new(route_matrix, starting_route).map(|elem| {
        let key = get_key(elem.x, elem.y);
        (key, elem)
    }).collect();

    let non_loop_routes: HashMap<String, &Route> = all_routes.into_iter().filter(
        |(key, _elem)| {
            match loop_routes.get(key) {
                Some(_value) => return false,
                None => return true,
            }
        }).collect();

    let mut routes_conditions = vec![vec![-1; route_matrix[0].len()]; route_matrix.len()];
    for (_, route) in non_loop_routes {
        if is_surrounded_by_walls(route, &loop_routes, route_matrix) {
            routes_conditions[route.y][route.x] = SURROUNDED;
        } else {
            routes_conditions[route.y][route.x] = NOT_SURROUNDED;
        }
    }

    reset_tiles_that_have_unsurrounded_neighbors(&mut routes_conditions);

    let mut count = 0;
    for line in routes_conditions {
        for condition in line {
            if condition == SURROUNDED {
                count += 1;
            }
        }
    }
    count
}

fn find_starting_route(route_matrix: &[Vec<Route>]) -> &Route {
    route_matrix
        .iter()
        .flat_map(|row| row.iter())
        .find(|&route| route.pipe == Pipe::Start)
        .expect("Couldn't find starting position")
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
                .map(|(x, ch)| Route {
                    pipe: Pipe::from(ch),
                    y,
                    x,
                })
                .collect()
        })
        .collect()
}

fn main() {
    let route_matrix = read_input_file();
    let starting_position = find_starting_route(&route_matrix);
    let steps_to_farthest_pipe = count_steps_to_farthest_pipe(&route_matrix, starting_position);
    let tiles_inside_loop = count_tiles_inside_loop(&route_matrix, starting_position);
    println!("Starting position: {:?}", starting_position);
    println!("Steps to farthest pipe: {:?}", steps_to_farthest_pipe);
    println!("Tiles inside loop: {:?}", tiles_inside_loop);
}
