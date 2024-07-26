const UNBLOCKED: i32 = 0;
const BLOCKED: i32 = -1;
const LOCKED: i32 = -2;
const CLOSED: i32 = -3;

#[derive(PartialEq, Copy, Clone)]
pub enum Direction {
    Blocked = -1,
    North = 0,
    East = 1,
    South = 2,
    West = 3,
}

fn direction_to_string(direction: &Direction) -> String {
    match *direction {
        Direction::Blocked => "Blocked".to_string(),
        Direction::North => "North".to_string(),
        Direction::East => "East".to_string(),
        Direction::South => "South".to_string(),
        Direction::West => "West".to_string(),
    }
}

struct Exit {
    direction: Direction,
    name: String,
    edge: i32,
}

impl Exit {
    fn new(direction: Direction, edge: i32) -> Exit {
        Exit {
            name: direction_to_string(&direction),
            direction,
            edge,
        }
    }
}

struct Location {
    name: String,
    description: String,
    exits: [Exit; 4],
}

impl Location {
    fn new(name: &str, description: &str, exits: [i32; 4]) -> Location {
        let exits = [
            if exits[0] != -1 {
                Exit::new(Direction::North, exits[0])
            } else {
                Exit::new(Direction::Blocked, -1)
            },
            if exits[1] != -1 {
                Exit::new(Direction::East, exits[1])
            } else {
                Exit::new(Direction::Blocked, -1)
            },
            if exits[2] != -1 {
                Exit::new(Direction::South, exits[2])
            } else {
                Exit::new(Direction::Blocked, -1)
            },
            if exits[3] != -1 {
                Exit::new(Direction::West, exits[3])
            } else {
                Exit::new(Direction::Blocked, -1)
            },
        ];
        Location {
            name: name.to_string(),
            description: description.to_string(),
            exits,
        }
    }
}

struct Edge {
    destination: usize,
    is_locked: bool,
    is_closed: bool,
    target_id: i32,
}

impl Edge {
    fn new(destination: usize, is_locked: bool, is_closed: bool, target_id: i32) -> Edge {
        Edge {
            destination,
            is_locked,
            is_closed,
            target_id,
        }
    }
}

pub struct Location_Dto {
    pub name: String,
    pub description: String,
    pub exits: String,
}

pub struct Map {
    // Each Location has a vector of Exits referencing Edges
    // Each Edge has a destination Location and is each Edge is unidirectional
    // so just because you can go from A to B doesn't mean you can go from B to A
    locations: Vec<Location>,
    edges: Vec<Edge>,
}

impl Map {
    pub fn new() -> Map {
        let locations = vec![
            Location::new("Start Room", "You are in the Start Room", [0, 2, 4, 6]),
            Location::new(
                "Dark Room",
                "You are in a dark room.",
                [BLOCKED, BLOCKED, 1, BLOCKED],
            ),
            Location::new(
                "Bright Room",
                "You are in a bright room.",
                [BLOCKED, BLOCKED, BLOCKED, 3],
            ),
            Location::new(
                "Room with a door",
                "You are in a room with a door.",
                [5, BLOCKED, BLOCKED, BLOCKED],
            ),
            Location::new(
                "Kitchen",
                "You are in the kitchen",
                [BLOCKED, 7, BLOCKED, BLOCKED],
            ),
        ];
        let edges = vec![
            Edge::new(1, false, false, 0),
            Edge::new(0, false, false, 0),
            Edge::new(2, false, false, 0),
            Edge::new(0, false, false, 0),
            Edge::new(3, false, false, 0),
            Edge::new(0, false, false, 0),
            Edge::new(4, false, false, 0),
            Edge::new(0, false, false, 0),
        ];
        Map { locations, edges }
    }

    fn get_exits_string(&self, location: usize) -> String {
        assert!(location < self.locations.len());

        let mut exits = String::new();
        for (i, exit) in self.locations[location].exits.iter().enumerate() {
            if exit.edge != BLOCKED {
                if i == &self.locations[location].exits.len() - 1 {
                    exits.push_str(&format!("{}", exit.name));
                } else {
                    exits.push_str(&format!("{}, ", exit.name));
                }
            }
        }
        exits
    }

    pub fn get_location(&self, location: usize) -> Location_Dto {
        assert!(location < self.locations.len());

        Location_Dto {
            name: self.locations[location].name.clone(),
            description: self.locations[location].description.clone(),
            exits: self.get_exits_string(location),
        }
    }

    pub fn move_to(&self, current_location: usize, direction: Direction) -> (i32, usize) {
        assert!(current_location < self.locations.len());
        assert!(direction != Direction::Blocked);

        let selected_edge = self.locations[current_location].exits[direction as usize].edge;
        match selected_edge {
            BLOCKED => return (BLOCKED, 0),
            _ => {
                let edge = &self.edges[selected_edge as usize];
                match edge {
                    Edge {is_locked: true, ..} => return (LOCKED, 0),
                    Edge {is_closed: true, ..} => return (CLOSED, 0),
                    _  => return (UNBLOCKED, edge.destination),
                }
            }
        }
    }
}
