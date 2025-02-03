use piston_window::{Context, G2d};
use piston_window::types::Color;
use crate::model::game::Game;

pub trait Display {

    ///
    /// Draws an object to the display. Used by all structs within the view module.
    /// 
    fn draw(&self, game: Game, color: Color, con: &Context, g: &mut G2d);
}