use piston_window::{rectangle,Context, G2d};
use piston_window::types::Color;
use crate::model::game::Game;
use crate::view::display;
use crate::view::draw::*;
use crate::view::gamedisplay::GameDisplay;
use crate::view::interfacedisplay::InterfaceDisplay;

pub struct WindowDisplay {
    interface_display: InterfaceDisplay,
    game_display: GameDisplay,
}

impl WindowDisplay {
    pub fn new(interface_display: InterfaceDisplay, game_display: GameDisplay) -> Self {
        WindowDisplay { interface_display, game_display }
    }
}

impl display for WindowDisplay {
    fn draw(&self, game: Game, color: Color, con: &Context, g: &mut G2d) {
        self.game_display.display();
    }
}






