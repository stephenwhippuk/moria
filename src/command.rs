use crate::map::Direction;
use std::io;
pub enum Command{
    Move(Direction),
    Open(Direction),
    Close(Direction),
    Quit,
}

pub fn get_command() -> Option<Command>{
    let mut input = String::new();
    
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    
    let input = input.trim().to_lowercase();
    let command = input.split_whitespace().collect::<Vec<&str>>();
    
    match command[0] {
        "quit" | "q" => {
            Some(Command::Quit)
        },
        "n" => {
            Some(Command::Move(Direction::North))
        },
        "e" => {
            Some(Command::Move(Direction::East))
        },
        "s" => {
            Some(Command::Move(Direction::South))
        },
        "w" => {
            Some(Command::Move(Direction::West))
        },
        "open" => {
            match command[1] {
                "n" => {
                    Some(Command::Open(Direction::North))
                },
                "e" => {
                    Some(Command::Open(Direction::East))
                },
                "s" => {
                    Some(Command::Open(Direction::South))
                },
                "w" => {
                    Some(Command::Open(Direction::West))
                },
                _ => {
                    None
                }
            }
        },
        "close" => {
            match command[1] {
                "n" => {
                    Some(Command::Close(Direction::North))
                },
                "e" => {
                    Some(Command::Close(Direction::East))
                },
                "s" => {
                    Some(Command::Close(Direction::South))
                },
                "w" => {
                    Some(Command::Close(Direction::West))
                },
                _ => {
                    None
                }
            }
        },
        _ => {
            None
        }
    }
}