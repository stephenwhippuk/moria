use std::{option::Option, str::FromStr};
use crate::gameobj::GameObject;

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

    fn unlock(&mut self){
        self.is_locked = false;
        self.is_closed = true;
    }

    fn open(&mut self){
        if(!self.is_locked){
            self.is_closed = false;
        }
    }

    fn close(&mut self){
        self.is_closed = true;
    }

    fn lock(&mut self){
        if(!self.is_closed){
            self.is_locked = true;
        }
    }

}

struct Location {
    name: String,
    description: String,
    exits: [Option<Edge>; 4],
    objects: Vec<usize>
}

impl Location {
    fn new(name: &str, description: &str, exits: [Option<Edge>; 4]) -> Location {
        Location {
            name: name.to_string(),
            description: description.to_string(),
            exits,
            objects: vec![]
        }
    }

    fn add_object(&mut self, object_id: usize){
        self.objects.push(object_id);
    }
}

pub struct LocationDto {
    pub name: String,
    pub description: String,
    pub exits: String,
    pub objects: String,
}

pub struct Map {
    locations: Vec<Location>,
    objects: Vec<GameObject>,
}

impl Map{
    pub fn new () -> Map{
            let mut locations = vec! [
                Location::new(
                    "The Shire",
                    "You are in the Shire. It is a peaceful place.",
                    [
                        Some(Edge::new(1, true, false,0)),
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
                        Some(Edge::new(0, true, false, 0)),
                        None
                    ]
                )
            ];
            
            // load object instances
            let objects = vec![
                GameObject::add_key("Shire Key".to_string(), 1)
            ];

            // add objects to locations
            locations[0].add_object(0);
            
            Map{locations, objects}
    }
    pub fn get_location(&self, location_id: usize) -> Option<LocationDto>{
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

        let mut objects = vec![];
        for object_id in &location.objects{
            let object = &self.objects[*object_id];
            objects.push(object.get_name());
        }

        Some(LocationDto{
            name: location.name.clone(),
            description: location.description.clone(),
            exits: exits.join(", "),
            objects: objects.join(", ")
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
    pub fn  is_closed(&self, location_id: usize, direction: Direction) -> bool{
        let location = &self.locations[location_id];
        let exit = &location.exits[direction as usize];
        match exit{
            Some(edge) => edge.is_closed || edge.is_locked,
            None => false,
        }
    }

    pub fn is_locked(&self, location_id: usize, direction: Direction) -> bool{
        let location = &self.locations[location_id];
        let exit = &location.exits[direction as usize];
        match exit{
            Some(edge) => edge.is_locked,
            None => false,
        }
    }

    pub fn unlock(&mut self, location_id: usize, direction: Direction){
        let location = &mut self.locations[location_id];
        let exit = &mut location.exits[direction as usize];
        match exit{
            Some(edge) => edge.unlock(),
            None => (),
        }
    }

    pub fn open(&mut self, location_id: usize, direction: Direction){
        let location = &mut self.locations[location_id];
        let exit = &mut location.exits[direction as usize];
        match exit{
            Some(edge) => edge.open(),
            None => (),
        }
    }

    pub fn close(&mut self, location_id: usize, direction: Direction){
        let location = &mut self.locations[location_id];
        let exit = &mut location.exits[direction as usize];
        match exit{
            Some(edge) => edge.close(),
            None => (),
        }
    }

    pub fn lock(&mut self, location_id: usize, direction: Direction){
        let location = &mut self.locations[location_id];
        let exit = &mut location.exits[direction as usize];
        match exit{
            Some(edge) => edge.lock(),
            None => (),
        }
    }

    pub fn edge_target_matched(&self, location_id: usize, direction: Direction, target_id: i32) -> bool{
        let location = &self.locations[location_id];
        let exit = &location.exits[direction as usize];
        match exit{
            Some(edge) => edge.target_id == target_id,
            None => false,
        }
    }
}
