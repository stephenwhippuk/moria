use crate::map::LocationDto;

pub fn display_location(loc : &LocationDto) {
    println!("You are in {}", loc.name);
    println!("{}", loc.description);
    println!("Exits: {}", loc.exits);
    println!("Objects: {}", loc.objects);
}