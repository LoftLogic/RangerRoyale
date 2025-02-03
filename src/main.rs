use piston_window::{clear, PistonWindow, WindowSettings};
use crate::model::round::cell::Biome;

extern crate piston_window;
mod model;
mod view;

use crate::model::game::Game;
use crate::view::windowdisplay::{to_coord, to_coord_u32};
use crate::model::game;
use crate::model::round::level::Level;
use crate::model::round::cell::Cell;
use crate::model::player::Player;
use crate::model::player::Difficulty;

use std::collections::HashSet;
use piston_window::types::Color;

const BLACK_COLOR: Color = [0.5, 0.5, 0.5, 1.0];


fn main() {
    let (width, height): (i32, i32) = (20, 20);
    let mut window: PistonWindow = WindowSettings::new(
        "Snake",
        [to_coord_u32(width), to_coord_u32(height)]
    ).exit_on_esc(true).build().unwrap();
    let grid: Vec<Vec<Cell>> = basic_grid();
    let level: Level = Level::new("Default Level", grid, HashSet::new(),
          Player::new("Enemy".to_string(), Difficulty::Easy));
    let game: Game = Game::new(vec![level], vec![Player::new("Player".to_string(), Difficulty::Easy)]);
    while let Some(event) = window.next() {
        window.draw_2d(&event, |context, graphics, _device| {
            clear(BLACK_COLOR, graphics);
        });
    }
}

fn basic_grid() -> Vec<Vec<Cell>> {
    let mut result: Vec<Vec<Cell>> = Vec::new();
    for i in 1..=8 {
        result.push(Vec::new());
        for j in 1..=8 {
            result[i].push(Cell::new(Biome::Plain, None, None));
        }
    }
    result
}