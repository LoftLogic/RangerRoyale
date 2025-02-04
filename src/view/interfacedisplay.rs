use piston_window::{Context, G2d};
use piston_window::types::Color;
use crate::model::game::Game;
use crate::view::display;
use crate::view::display::Display;

///
/// The display for the user's interface in the game.
///
pub struct InterfaceDisplay {

}

impl Display for InterfaceDisplay {
    fn draw(&self, game: Game, color: Color, con: &Context, g: &mut G2d) {

    }
}