use piston_window::{rectangle, Context, G2d, Glyphs};
use piston_window::types::Color;
use crate::model::game::*;
use crate::view::util;
use crate::view::util::draw_button;

use crate::{HALF_Y, HALF_X};

pub fn draw_start(scene: &StartScene, con: &Context, graphics: &mut G2d, glyphs: &mut Glyphs) {
    let button = scene.get_button();
    let label = button.get_label();
    let font_size = 14;
    let fill_color = [0.5, 0.5, 0.9, 1.0];
    let x = button.get_top_left().0 as i32;
    let y = button.get_top_left().1 as i32;
    let width = button.get_bottom_right().0 as i32 - x;
    let height = button.get_bottom_right().1 as i32 - y;
    draw_button(button.get_label(), font_size, fill_color, x, y, width, height, con, graphics, glyphs);
}

pub fn draw_level_selection(scene: &LevelSelectScene, con: &Context, graphics: &mut G2d, glyphs: &mut Glyphs) {
    let buttons = scene.get_buttons();
    for button in buttons.iter() {
        let label = button.get_label();
        let font_size = 10;
        let fill_color = [0.5, 0.5, 0.65, 1.0];
        let x = button.get_top_left().0 as i32;
        let y = button.get_top_left().1 as i32;
        let width = button.get_bottom_right().0 as i32 - x;
        let height = button.get_bottom_right().1 as i32 - y;
        draw_button(button.get_label(), font_size, fill_color, x, y, width, height, con, graphics, glyphs);
    }
}