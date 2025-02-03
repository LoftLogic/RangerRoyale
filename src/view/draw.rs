use piston_window::{rectangle, Context, G2d};
use piston_window::types::Color;

///
/// Contains a list of useful functions for G2D
///

pub const CELL_SIZE: f64 = 30.0;

pub fn to_coord(game_coord: i32) -> f64 {
    (game_coord as f64) * crate::view::windowdisplay::CELL_SIZE
}

pub fn to_coord_u32(game_coord: i32) -> u32 {
    to_coord(game_coord) as u32
}

pub fn draw_block(color: Color, x: i32, y: i32, con: &Context, g: &mut G2d) {
    let gui_x: f64 = to_coord(x);
    let gui_y: f64 = to_coord(y);
    rectangle(
        color,
        [gui_x, gui_y, CELL_SIZE, CELL_SIZE],
        con.transform,
        g
    );
}

pub fn draw_rectangle(color: Color, x: i32, y: i32, width: i32, height: i32,
                      con: &Context, g: &mut G2d) {
    let gui_x: f64 = to_coord(x);
    let gui_y: f64 = to_coord(y);
    rectangle(color, [gui_x, gui_y, width as f64, height as f64], con.transform, g);
}