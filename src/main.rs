use crate::map::Direction;
use crate::map::Location_Dto;
use crate::map::Map;

pub mod map;

fn main() {
    let map = Map::new();
    let mut loc = map.get_location(0);
    println!("{}", loc.name);
    println!("{}", loc.description);
    println!("{}", loc.exits);
    let next_location = map.move_to(0, Direction::South);
    if next_location.0 == -1 {
        println!("You can't go that way!");
    } else {
        loc = map.get_location(next_location.1);
        println!("{}", loc.name);
        println!("{}", loc.description);
        println!("{}", loc.exits);
    }
    let next_location = map.move_to(next_location.1, Direction::South);
    if next_location.0 == -1 {
        println!("You can't go that way!");
    } else {
        loc = map.get_location(next_location.1);
        println!("{}", loc.name);
        println!("{}", loc.description);
        println!("{}", loc.exits);
    }
}
