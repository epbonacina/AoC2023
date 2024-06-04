use std::cmp;
use std::collections::HashMap;
use std::fs;

const FILE_PATH: &str = "input.txt";
// const FILE_PATH: &str = "smaller_input.txt";
// const FILE_PATH: &str = "smaller_input2.txt";
// const FILE_PATH: &str = "smaller_input3.txt";
// const FILE_PATH: &str = "smaller_input4.txt";

const NOT_SURROUNDED: i32 = 0;
const SURROUNDED: i32 = 1;
const CHECKED: i32 = 2;

#[derive(Debug, Clone, PartialEq)]
enum Direction {
    Above,
    Right,
    Below,
    Left,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
    fn get_neighbors<'a>(&'a self, route_matrix: &'a [Vec<Route>]) -> Vec<(Direction, &Route)> {
        let candidates_pos = [
            (self.x, cmp::min(self.y + 1, route_matrix.len() - 1)),
            (cmp::min(self.x + 1, route_matrix[0].len() - 1), self.y),
            (self.x, self.y.saturating_sub(1)),
            (self.x.saturating_sub(1), self.y),
        ];

        let mut directions = Vec::new();
        for (i, (candidate_x, candidate_y)) in candidates_pos.into_iter().enumerate() {
            if (candidate_x, candidate_y) != (self.x, self.y) {
                let neighbor = &route_matrix[candidate_y][candidate_x];
                let direction = match i {
                    0 => (Direction::Above, neighbor),
                    1 => (Direction::Right, neighbor),
                    2 => (Direction::Below, neighbor),
                    _ => (Direction::Left, neighbor),
                };
                directions.push(direction);
            }
        }
        directions
    }

    fn is_connected_to(&self, other: &Route) -> bool {
        let direction = match (other.x, other.y) {
            (x, y) if x == self.x && y == self.y + 1 => Direction::Above,
            (x, y) if x == self.x + 1 && y == self.y => Direction::Right,
            (x, y) if x == self.x && y == self.y - 1 => Direction::Below,
            (x, y) if x == self.x - 1 && y == self.y => Direction::Left,
            _ => return false,
        };

        let connectable_directions = self.get_connectable_directions();
        if connectable_directions.contains(&direction) {
            let relative_direction = match direction {
                Direction::Above => Direction::Below,
                Direction::Right => Direction::Left,
                Direction::Below => Direction::Above,
                Direction::Left => Direction::Right,
            };

            if other
                .get_connectable_directions()
                .contains(&relative_direction)
            {
                return true;
            }
        }
        false
    }

    fn get_connectable_directions(&self) -> Vec<Direction> {
        let directions = HashMap::from([
            (
                Pipe::Start,
                vec![
                    Direction::Above,
                    Direction::Right,
                    Direction::Below,
                    Direction::Left,
                ],
            ),
            (Pipe::NorthSouth, vec![Direction::Above, Direction::Below]),
            (Pipe::NorthEast, vec![Direction::Above, Direction::Right]),
            (Pipe::NorthWest, vec![Direction::Above, Direction::Left]),
            (Pipe::EastWest, vec![Direction::Right, Direction::Left]),
            (Pipe::SouthEast, vec![Direction::Right, Direction::Below]),
            (Pipe::SouthWest, vec![Direction::Below, Direction::Left]),
            (Pipe::Obstructed, vec![]),
        ]);
        directions.get(&self.pipe).unwrap().to_vec()
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

    fn get_connected_neighbors(&self) -> Vec<Route> {
        let neighbors = self.current_route.get_neighbors(self.route_matrix);
        let connected_neighbors = neighbors
            .into_iter()
            .filter(|(_, neighbor)| self.current_route.is_connected_to(neighbor))
            .map(|(_, neighbor)| neighbor.clone());
        connected_neighbors.collect()
    }
}

impl<'a> Iterator for Navigator<'a> {
    type Item = &'a Route;

    fn next(&mut self) -> Option<Self::Item> {
        let neighbors = self.get_connected_neighbors();
        for neighbor in neighbors.iter() {
            if (neighbor.x, neighbor.y) != (self.initial_x, self.initial_y)
                && neighbor != self.previous_route
            {
                self.previous_route = self.current_route;
                self.current_route = &self.route_matrix[neighbor.y][neighbor.x];
                return Some(self.current_route);
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
            Pipe::NorthSouth => {
                return true;
            }
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
            Pipe::EastWest => {
                return true;
            }
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
            Pipe::NorthSouth => return true,
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

fn reset_tiles_that_have_unsurrounded_neighbors(routes_conditions: &mut Vec<Vec<i32>>) {
    let (mut x, mut y) = (-1, -1);
    for (iy, conditions) in routes_conditions.iter_mut().enumerate() {
        for (ix, condition) in conditions.iter_mut().enumerate() {
            if condition == &NOT_SURROUNDED && (x, y) == (-1, -1) {
                *condition = CHECKED;
                (x, y) = (ix as i32, iy as i32);
            }
        }
    }

    if (x, y) == (-1, -1) {
        return;
    }

    let (x, y) = (x as usize, y as usize);
    let candidates_pos = [
        (x, cmp::min(y + 1, routes_conditions.len() - 1)),
        (cmp::min(x + 1, routes_conditions[0].len() - 1), y),
        (x, y.saturating_sub(1)),
        (x.saturating_sub(1), y),
    ];

    for (candidate_x, candidate_y) in candidates_pos {
        if routes_conditions[candidate_y][candidate_x] == SURROUNDED {
            routes_conditions[candidate_y][candidate_x] = NOT_SURROUNDED;
        }
    }

    reset_tiles_that_have_unsurrounded_neighbors(routes_conditions);
}

fn get_key(x: usize, y: usize) -> String {
    format!("x={x},y={y}")
}

fn count_tiles_inside_loop(route_matrix: &[Vec<Route>], starting_route: &Route) -> u32 {
    let all_routes: HashMap<String, &Route> = route_matrix
        .iter()
        .flatten()
        .map(|elem| {
            let key = get_key(elem.x, elem.y);
            (key, elem)
        })
        .collect();

    let loop_routes: HashMap<String, &Route> = Navigator::new(route_matrix, starting_route)
        .map(|elem| {
            let key = get_key(elem.x, elem.y);
            (key, elem)
        })
        .collect();

    let non_loop_routes: HashMap<String, &Route> = all_routes
        .into_iter()
        .filter(|(key, _elem)| match loop_routes.get(key) {
            Some(_value) => return false,
            None => return true,
        })
        .collect();

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
