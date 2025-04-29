/// UTIL.RS:
/// Contains useful drawing utility functions for rendering grid-based games
/// using the Piston game engine. Provides coordinate conversion and drawing
/// primitives to work with a grid of fixed-size cells.

use piston_window::{Context, G2d, rectangle, ellipse, Text, Transformed, CharacterCache};
use piston_window::types::Color;
use piston_window::Glyphs;

/// The size of one grid cell in pixels.
///
/// Used to convert between game logic coordinates (grid-based) and
/// screen coordinates (pixel-based).
pub const CELL_SIZE: f64 = 75.0;

/// Converts a grid-based coordinate to a screen (pixel-based) coordinate.
///
/// # Arguments
///
/// * `game_coord` - The coordinate on the logical grid.
///
/// # Returns
///
/// * `f64` - The corresponding pixel-based coordinate.
pub fn to_coord(game_coord: i32) -> f64 {
    (game_coord as f64) * CELL_SIZE
}

/// Converts a grid-based coordinate to a screen coordinate as a `u32`.
///
/// # Arguments
///
/// * `game_coord` - The coordinate on the logical grid.
///
/// # Returns
///
/// * `u32` - The corresponding pixel-based coordinate rounded down to an integer.
pub fn to_coord_u32(game_coord: i32) -> u32 {
    to_coord(game_coord) as u32
}

/// Draws a square block on the screen at the given grid position.
///
/// This function uses the constant `CELL_SIZE` to determine the size of the block.
///
/// # Arguments
///
/// * `color` - The color of the block (RGBA array).
/// * `x` - The x-coordinate on the game grid.
/// * `y` - The y-coordinate on the game grid.
/// * `con` - The drawing context from Piston.
/// * `g` - The graphics buffer used to render the block.
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

/// Draws a square block with a border on the screen at the given grid position.
///
/// The block is filled with `fill_color` and surrounded by a border of a constant thickness
/// in `border_color`. The size of the block is determined by `CELL_SIZE`.
///
/// # Arguments
///
/// * `fill_color` - The color of the inner block (RGBA).
/// * `border_color` - The color of the border (RGBA).
/// * `x` - The x-coordinate on the game grid.
/// * `y` - The y-coordinate on the game grid.
/// * `con` - The drawing context from Piston.
/// * `g` - The graphics buffer used to render the shapes.
pub fn draw_block_with_border(
    fill_color: Color,
    border_color: Color,
    x: i32,
    y: i32,
    con: &Context,
    g: &mut G2d
) {
    const BORDER_THICKNESS: f64 = 5.0;

    let gui_x = to_coord(x);
    let gui_y = to_coord(y);

    // Draw border (slightly larger rectangle underneath)
    rectangle(
        border_color,
        [
            gui_x - BORDER_THICKNESS,
            gui_y - BORDER_THICKNESS,
            CELL_SIZE + 2.0 * BORDER_THICKNESS,
            CELL_SIZE + 2.0 * BORDER_THICKNESS,
        ],
        con.transform,
        g,
    );

    // Draw block (on top of the border)
    rectangle(
        fill_color,
        [gui_x, gui_y, CELL_SIZE, CELL_SIZE],
        con.transform,
        g,
    );
}

/// Draws a rectangle on the screen with dimensions specified in grid units.
///
/// Converts the grid-based dimensions into pixel coordinates using `CELL_SIZE`.
///
/// # Arguments
///
/// * `color` - The color of the rectangle (RGBA array).
/// * `x` - The x-coordinate of the top-left corner on the game grid.
/// * `y` - The y-coordinate of the top-left corner on the game grid.
/// * `width` - The width of the rectangle in grid cells.
/// * `height` - The height of the rectangle in grid cells.
/// * `con` - The drawing context from Piston.
/// * `g` - The graphics buffer used to render the rectangle.
pub fn draw_rectangle(color: Color, x: i32, y: i32, width: i32, height: i32,
                      con: &Context, g: &mut G2d) {
    let gui_x: f64 = to_coord(x);
    let gui_y: f64 = to_coord(y);
    let gui_width: f64 = to_coord(width);
    let gui_height: f64 = to_coord(height);
    rectangle(color, [gui_x, gui_y, gui_width, gui_height], con.transform, g);
}

/// Helper: draw a rounded‐corner rect by combining rectangles + corner ellipses.
///
/// * `col` – fill color RGBA
/// * `(x,y)` – top‐left in pixels
/// * `(w,h)` – size in pixels
/// * `r` – corner radius in pixels
fn draw_rounded_rect(
    color: Color,
    x: f64, y: f64,
    w: f64, h: f64,
    r: f64,
    con: &Context,
    g: &mut G2d,
) {
    // center cross
    rectangle(color, [x + r, y,       w - 2.0*r, h      ], con.transform, g);
    rectangle(color, [x,       y + r,   w,       h - 2.0*r], con.transform, g);

    // corner circles (ellipses)
    let d = 2.0 * r;
    ellipse(color, [x,       y,       d, d], con.transform, g);             // top‐left
    ellipse(color, [x + w - d, y,       d, d], con.transform, g);           // top‐right
    ellipse(color, [x,       y + h - d, d, d], con.transform, g);           // bot‐left
    ellipse(color, [x + w - d, y + h - d, d, d], con.transform, g);         // bot‐right
}

/// Draws a rectangular button with rounded corners, a black border, and centered text.
///
/// # Arguments
///
/// * `label`      – The text to draw inside the button.
/// * `font_size`  – Font size in pixels for the label.
/// * `fill_color` – RGB color array for the button’s fill.
/// * `x`, `y`     – Top-left corner of the button, in grid coords.
/// * `width`, `height` – Size of the button, in grid cells.
/// * `con`        – Piston drawing context.
/// * `g`          – Graphics backend.
/// * `glyphs`     – Loaded font glyphs for text rendering.
pub fn draw_button(
    label: &str,
    font_size: u32,
    fill_color: Color,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    con: &Context,
    g: &mut G2d,
    glyphs: &mut Glyphs,
) {
    // Convert grid coords to pixels
    let px = to_coord(x);
    let py = to_coord(y);
    let pw = to_coord(width);
    let ph = to_coord(height);

    // Style
    let border_thick = 2.0;
    let corner_r = 8.0;
    let border_col: Color = [0.0, 0.0, 0.0, 1.0];

    // 1) Border (draw slightly larger rounded rect)
    draw_rounded_rect(border_col, px, py, pw, ph, corner_r, con, g);

    // 2) Fill (inset by border thickness; shrink corner radius accordingly)
    let inner_r = corner_r - border_thick;
    draw_rounded_rect(
        fill_color,
        px + border_thick,
        py + border_thick,
        pw - 2.0 * border_thick,
        ph - 2.0 * border_thick,
        inner_r.max(0.0),
        con,
        g,
    );

    // 3) Centered label
    // measure text width
    let text_w = glyphs
        .width(font_size, label)
        .unwrap_or(label.len() as f64 * (font_size as f64 * 0.5));
    let text_x = px + (pw - text_w) / 2.0;
    let text_y = py + (ph + font_size as f64) / 2.0 - 2.0;

    let text = Text::new_color(border_col, font_size);
    text.draw(
        label,
        glyphs,
        &con.draw_state,
        con.transform.trans(text_x, text_y),
        g,
    ).ok();
}
 

