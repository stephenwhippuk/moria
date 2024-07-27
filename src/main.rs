use crate::game::Game;

pub mod map;
pub mod gameobj;
pub mod player;
pub mod game;
pub mod command;
pub mod display;

fn main() {
    let mut game = Game::new();
    game.initialise();
    game.run();
}
