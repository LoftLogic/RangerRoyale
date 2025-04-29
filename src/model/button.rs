use std::cmp::PartialEq;
use crate::model::game::{GameError, GameEvent, UIElement, UIEvent};

pub struct Button {
    text: String,
    top_left: (f64, f64),
    bottom_right: (f64, f64),
    activation_event: GameEvent,
}

impl Button {
    pub fn new(text: &str, top_left: (f64, f64), bottom_right: (f64, f64), 
           activation_event: GameEvent) -> Self {
        let text: String = text.to_string();
        Self { text, top_left, bottom_right, activation_event }
    }
    
    pub fn get_label(&self) -> &str {
        &self.text
    }
}



impl UIElement for Button {
    fn get_name(&self) -> String {
        let mut s: String = "Button".to_string();
        s.push_str(self.get_label());
        s
    }
    
    fn update(&self, event: UIEvent) -> Result<Option<GameEvent>, GameError> {
        if event == UIEvent::ClickStartButton {
            return Ok(Some(GameEvent::StartGame));
        }
        Err(GameError::InvalidEvent)
    }

    fn is_selectable(&self) -> bool {
        false
    }

    fn get_top_left(&self) -> (f64, f64) {
        self.top_left
    }

    fn get_bottom_right(&self) -> (f64, f64) {
        self.bottom_right
    }

    fn click_event(&self) -> Option<GameEvent> {
        println!("Button Clicked");
        Some(self.activation_event.clone())
    }
}