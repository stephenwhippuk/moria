use crate::map::Direction;
//use crate::map::Location_Dto;
use crate::map::Map;
use crate::gameobj::GameObject;

pub mod map;
pub mod gameobj;

fn main() {
    let map = Map::new();
    let current_location: usize = 0;
    let mut loc = map.get_location(current_location);
    match(loc){
        Some(loc) => {
            println!("{}", loc.name);
            println!("{}", loc.description);
            println!("{}", loc.exits);
        },
        None => {
            println!("Location not found");
        }
    }
    // attempt to move to the north
    let next_location = map.move_to(current_location, Some(Direction::North));
    match(next_location){
        Some(next_location) => {
            loc = map.get_location(next_location);
            match loc {
                Some(loc) => {
                    println!("{}", loc.name);
                    println!("{}", loc.description);
                    println!("{}", loc.exits);
                },
                None => {
                    println!("Location not found");
                }
            }
        },
        None => {
            let is_locked = map.is_locked(current_location, Direction::North);
            let is_closed = map.is_closed(current_location, Direction::North);
            if is_locked {
                println!("The door is locked");
            } else if is_closed {
                println!("The door is closed");
            } else {
                println!("You can't go that way!");
            }
        }
    }
    let mut loc = map.get_location(0);

    // let mut loc = map.get_location(0);
    // println!("{}", loc.name);
    // println!("{}", loc.description);
    // println!("{}", loc.exits);
    // let next_location = map.move_to(0, Direction::South);
    // if next_location.0 == -1 {
    //     println!("You can't go that way!");
    // } else {
    //     loc = map.get_location(next_location.1);
    //     println!("{}", loc.name);
    //     println!("{}", loc.description);
    //     println!("{}", loc.exits);
    // }
    // let next_location = map.move_to(next_location.1, Direction::South);
    // if next_location.0 == -1 {
    //     println!("You can't go that way!");
    // } else {
    //     loc = map.get_location(next_location.1);
    //     println!("{}", loc.name);
    //     println!("{}", loc.description);
    //     println!("{}", loc.exits);
    // }
}
