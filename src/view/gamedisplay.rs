use piston_window::{Context, G2d};
use piston_window::types::Color;
use crate::model::game::Game;
use crate::view::display;

///
/// Represents the display of the game and its state
///
pub struct GameDisplay {

}

impl GameDisplay {

}

impl display for GameDisplay {
    fn draw(&self, game: Game, color: Color, con: &Context, g: &mut G2d) {

    }
}