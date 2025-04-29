use piston_window::{Context, Button, Event, G2d, Glyphs, MouseButton, PressEvent};
use crate::model::game::*;

// Maybe implement?
pub enum ControllerEvent {
    StartGame,
    UnitSelected(f64, f64),
    GridSelected(f64, f64),
}

impl ControllerEvent {
    
}

pub struct Controller {
    model: Game,
    cursor: [f64; 2],
    selected: Option<usize>,
}

impl Controller {
    pub fn new(model: Game) -> Self {
        Self { model, cursor: [0.0, 0.0], selected: None }
    }


    pub fn get_model(&self) -> &Game {
        &self.model
    }
    
    pub fn update_cursor(&mut self, pos: [f64; 2]) {
        self.cursor = pos;
    }

    pub fn handle_event(&mut self, event: &Event) {
        if let Some(Button::Mouse(MouseButton::Left)) = event.press_args() {
            println!("MouseX: {:?}", self.cursor[0]);
            println!("MouseY: {:?}", self.cursor[1]);
            if let Some(idx) = self.model.click_event(self.cursor[0],
                                                          self.cursor[1]) { 
                let element = self.model.get_ui_element(idx).unwrap();
                if element.is_selectable() {
                    self.selected = Some(idx);
                }
                let game_event = element.click_event();
                match game_event {
                    Some(game_event) => { self.model.receive_event(&game_event); },
                    None => {}
                }
            }
        }
    }
    
    pub fn render_model(&self, con: &Context, graphics: &mut G2d, glyphs: &mut Glyphs) {
        self.model.render_scene(con, graphics, glyphs);
    }
}