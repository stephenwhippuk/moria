use std::option::Option;
pub enum Direction{
    North = 0,
    East = 1,
    South = 2,
    West = 3
}

impl Direction{
    fn from_usize(direction: usize) -> Option<Direction> {
        match direction{
            0 => Some(Direction::North),
            1 => Some(Direction::East),
            2 => Some(Direction::South),
            3 => Some(Direction::West),
            _ => None,
        }
    }
    fn to_string(&self) -> String{
        match *self{
            Direction::North => "North".to_string(),
            Direction::East => "East".to_string(),
            Direction::South => "South".to_string(),
            Direction::West => "West".to_string(),
        }
    }
    fn from_string(direction: &str) -> Option<Direction>{
        match direction{
            "North" => Some(Direction::North),
            "East" => Some(Direction::East),
            "South" => Some(Direction::South),
            "West" => Some(Direction::West),
            _ => None,
        }
    }
}

struct Edge{
    destination: usize,
    is_closed: bool,
    is_locked: bool,
    target_id: i32,
}

impl Edge{
    fn new(destination: usize, is_closed: bool, is_locked: bool, target_id: i32) -> Edge{
        Edge{
            destination,
            is_closed,
            is_locked,
            target_id,
        }
    }
}

struct Location {
    name: String,
    description: String,
    exits: [Option<Edge>; 4],
}

impl Location {
    fn new(name: &str, description: &str, exits: [Option<Edge>; 4]) -> Location {
        Location {
            name: name.to_string(),
            description: description.to_string(),
            exits
        }
    }
}

pub struct Location_Dto {
    pub name: String,
    pub description: String,
    pub exits: String,
}

pub struct Map {
    locations: Vec<Location>,
}

impl Map{
    pub fn new () -> Map{
            let locations = vec! [
                Location::new(
                    "The Shire",
                    "You are in the Shire. It is a peaceful place.",
                    [
                        Some(Edge::new(1, false, false,0)),
                        None,
                        None,
                        None,
                    ]
                ),
                Location::new(
                    "Moria",
                    "You are in Moria. It is dark and scary.",
                    [
                        None,
                        None,
                        Some(Edge::new(0, false, false, 0)),
                        None
                    ]
                )
            ];
            Map{locations}
    }
    pub fn get_location(&self, location_id: usize) -> Option<Location_Dto>{
        if location_id >= self.locations.len(){
            return None;
        }
        
        let location = &self.locations[location_id];
        let mut exits = vec![];
        for (i, edge) in location.exits.iter().enumerate(){
            match edge{
                None => continue,
                _=> exits.push(Direction::from_usize(i).unwrap().to_string()),
            }
        }
        Some(Location_Dto{
            name: location.name.clone(),
            description: location.description.clone(),
            exits: exits.join(", "),
        })
    }
    pub fn move_to(&self, location_id: usize, direction: Option<Direction>) -> Option<usize> {
        match direction{
            None => Some(location_id),
            Some(direction) => {
                let location = &self.locations[location_id];
                let exit = &location.exits[direction as usize];
                match exit{
                    Some(edge) => {
                        if edge.is_closed || edge.is_locked{
                            None
                        } else {
                            Some(edge.destination)
                        }
                    },
                    None => None,
                }
            }
        }
    }
}
