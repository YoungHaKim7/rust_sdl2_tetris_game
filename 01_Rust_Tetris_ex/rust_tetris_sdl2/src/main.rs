// #![windows_subsystem = "windows"]// Keep a console from popping up on windows
// extern crate lazy_static;
// extern crate rusttetris;

use lazy_static::lazy_static;

// #[macro_use]
// extern crate lazy_static;

//use std::sync::Mutex;

lazy_static! {
    pub static ref GAMEDATA: data::GameDataS = data::load_data("../assets/data.json".to_string());
}

pub mod bdimentions;
pub mod board;
pub mod data;
pub mod game;
pub mod gameend;
pub mod level;
pub mod player;
pub mod startmenu;

fn main() {
    let game = game::Game;
    game.run_game();
}
