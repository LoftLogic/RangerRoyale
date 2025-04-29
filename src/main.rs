extern crate piston_window;

use piston_window::*;
use piston_window::types::Color;

use view::util::to_coord_u32;
use crate::view::util::to_coord;
use crate::view::util::draw_button;
use crate::controller::controller::Controller;
use crate::model::game::Game;

mod model;
mod view;
mod controller;

const BG_COLOR: Color = [0.8, 0.8, 0.8, 1.0];
const WIDTH: i32 = 15;
const HEIGHT: i32 = 10;

const HALF_X : i32 = (WIDTH - 2) / 2;
const HALF_Y : i32 = (HEIGHT - 2) / 2;

fn main() {
    let mut window: PistonWindow = WindowSettings::new(
        "Ranger Royale",
        [to_coord_u32(WIDTH), to_coord_u32(HEIGHT)]
    ).exit_on_esc(true).build().unwrap();

    let mut glyphs = window
        .load_font("src/assets/FiraSans-Medium.ttf")
        .expect("Couldn’t load font");
    
    let mut controller: Controller = Controller::new(Game::new());
    
    while let Some(event) = window.next() {
        controller.handle_event(&event);
        
        if let Some(pos) = event.mouse_cursor_args() {
            controller.update_cursor(pos)
        }
        
        window.draw_2d(&event, |context, graphics, device| {
            clear(BG_COLOR, graphics);
            let model: &Game = controller.get_model();
            model.render_scene(&context, graphics, &mut glyphs);

            glyphs.factory.encoder.flush(device);
        });

    }
}