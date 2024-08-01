use std::io;
use crate::map::Map;
use crate::player::Player;
use crate::command::Command;
use crate::command::get_command;
use crate::display::display_location;
pub struct Game {
    player: Player,
    map: Map,
}

impl Game {
    pub fn new() -> Game {
        Game {
            player: Player::new(0),
            map: Map::new(),
        }
    }
    
    pub fn initialise(&mut self) {
        
    }

    fn display(&self){
        let current = self.map.get_location(self.player.location_id);
        match current{
            Some(current) => display_location(&current),
            None => println!("Location not found"),
        }
    }

    pub fn run(&mut self){
        let mut exit = false;
        while !exit {
            // for now we'll just do the input in here but this should be moved out into its own module shortly
            self.display();

            let command = get_command();
            match command {
                Some(Command::Quit) => {
                    exit = true;
                },
                Some(Command::Move(direction)) => {
                    let new_location = self.map.move_to(self.player.location_id, direction);
                    match new_location {
                        Some(new_location) => {
                            self.player.location_id = new_location;
                        },
                        None => {
                            println!("You can't go that way");
                        }
                    }
                },
                Some(Command::Open(direction)) => {
                    let res = self.map.open(self.player.location_id, direction);
                    match res {
                        Ok(_) => {
                            println!("you opened the door");
                        },
                        Err(e) => {
                            println!("{}", e);
                        }
                    }
                },
                Some(Command::Close(direction)) => {
                    let res = self.map.close(self.player.location_id, direction);
                    match res {
                        Ok(_) => {
                            println!("you closed the door");
                        },
                        Err(e) => {
                            println!("{}", e);
                        }
                    }
                },
                None => {
                    println!("Unknown command");
                }
            }
        }
    }

}
