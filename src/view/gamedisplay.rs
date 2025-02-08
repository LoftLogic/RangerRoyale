use piston_window::{Context, G2d};
use piston_window::types::Color;
use crate::model::game::Game;
use crate::view::display;
use crate::view::display::Display;

///
/// Represents the display of the game and its state
///
pub struct GameDisplay {
    game: Game,
}

impl GameDisplay {
    pub fn new(&self, game: Game) -> GameDisplay {
        GameDisplay { game }
    }
}

impl Display for GameDisplay {
    fn draw(&self, game: Game, color: Color, con: &Context, g: &mut G2d) {
        
    }
}