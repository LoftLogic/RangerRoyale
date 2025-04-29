use std::collections::{HashMap, HashSet};
use piston_window::{Context, G2d, Glyphs};
use crate::model::button::Button;
use crate::model::game::GameEvent::SelectLevel;
use crate::{WIDTH, HEIGHT, HALF_X, HALF_Y};
use crate::model::inlevel::cell::Cell;
use crate::view::scenedrawer::*;
use crate::view::util::to_coord;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameError {
    SceneSelectionError,
    SceneActivationError,
    InvalidEvent
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameEvent {
    StartGame,
    SelectLevel(i32),
    EndLevel,
    MoveUnit(i32, i32, i32, i32)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UIEvent {
    ClickStartButton,
}

///
/// Represents a clickable object in a game.
///
pub trait UIElement {

    fn get_name(&self) -> String;

    ///
    /// Reacts to a certain event taking place.
    /// # Returns
    /// * Whether the update was successful
    ///
    fn update(&self, event: UIEvent) -> Result<Option<GameEvent>, GameError>;


    ///
    /// # Returns
    /// * If this item is selectable or not.
    /// * Selectable refers to when you click an element, and further action is required.
    ///
    fn is_selectable(&self) -> bool;

    ///
    /// # Returns
    /// * The coordinate of this elements top left corner.
    /// 
    fn get_top_left(&self) -> (f64, f64);

    ///
    /// # Returns
    /// * The coordinate of this elements bottom right corner.
    ///
    fn get_bottom_right(&self) -> (f64, f64);

    fn click_event(&self) -> Option<GameEvent>;

    ///
    /// # Returns
    /// * The area that this object encompasses. Used for click events.
    /// * 0: x1, 1: y1, 2: x2, 3: y2
    ///
    fn get_container(&self) -> (f64, f64, f64, f64) {
        let tl = self.get_top_left();
        let br = self.get_bottom_right();
        (tl.0, tl.1, br.0, br.1)
    }

    ///
    /// # Returns
    /// * The area that this object encompasses, in coordinates (instead of grid). Used for click events.
    /// * 0: x1, 1: y1, 2: x2, 3: y2
    ///
    fn get_container_coords(&self) -> (f64, f64, f64, f64) {
        let tl = self.get_top_left();
        let br = self.get_bottom_right();
        (to_coord(tl.0 as i32), to_coord(tl.1 as i32),
         to_coord(br.0 as i32), to_coord(br.1 as i32))
    }

    ///
    /// # Returns
    /// * If the click is in the container.
    ///
    fn cursor_in_container(&self, mouse_x: f64, mouse_y: f64) -> bool {
        let (x1, y1, x2, y2) = self.get_container_coords();
        mouse_x >= x1 && mouse_x <= x2 && mouse_y >= y1 && mouse_y <= y2
    }
}

///
/// Represents one of the scenes that the game can be currently in.
///
pub trait Scene {

    fn get_name(&self) -> &str;

    ///
    /// Activates the scene. Allows for mutation and functionality.
    ///
    /// # Returns
    /// * Whether the activation was successful.
    ///
    fn activate(&mut self) -> Result<(), GameError>;

    ///
    /// Deactivates the scene. Disables mutation and behavior.
    ///
    /// # Returns
    /// * Whether the deactivation was successful.
    ///
    fn deactivate(&mut self) -> Result<(), GameError>;

    ///
    /// Calls an event on this item.
    ///
    fn receive_event(&mut self, event: &GameEvent);

    fn get_ui_elements(&self) -> Vec<&dyn UIElement>;

    fn render(&self, con: &Context, g: &mut G2d, glyphs: &mut Glyphs);
}

pub struct StartScene {
    name: String,
    start_button: Button,
    activated: bool,
}

impl StartScene {
    fn new() -> Self {
        let start_button = Button::new("Start", (HALF_X as f64,
             HALF_Y as f64), ((HALF_X + 2) as f64, (HALF_Y + 1) as f64),
            GameEvent::StartGame);
        Self { name: "Start".to_string(), start_button, activated: false }
    }

    pub fn get_button(&self) -> &Button {
        &self.start_button
    }
}

impl Scene for StartScene {
    fn get_name(&self) -> &str {
        self.name.as_str()
    }

    fn activate(&mut self) -> Result<(), GameError> {
        if self.activated {
            return Err(GameError::SceneActivationError);
        }
        self.activated = true;
        Ok(())
    }

    fn deactivate(&mut self) -> Result<(), GameError> {
        if !self.activated {
            return Err(GameError::SceneActivationError);
        }
        self.activated = false;
        Ok(())
    }

    fn receive_event(&mut self, event: &GameEvent) {
        if !self.activated {
            println!("StartScene received event while deactivated: {:?}", event);
            return;
        }
        todo!()
    }

    fn get_ui_elements(&self) -> Vec<&dyn UIElement> {
        vec![&self.start_button]
    }

    fn render(&self, con: &Context, g: &mut G2d, glyphs: &mut Glyphs) {
        if !self.activated {
            println!("StartScene rendered while not activated");
            return;
        }
        draw_start(&self, con, g, glyphs);
    }
}

pub struct LevelSelectScene {
    name: String,
    activated: bool,
    level_buttons: Vec<Button>,
}

impl LevelSelectScene {
    pub fn new(num_levels: i32) -> Self {
        if num_levels < 1 {
            panic!("Need at least one level");
        }

        // GRID-BASED MARGINS 
        // Side margins = 25% of WIDTH
        let side_margin: i32 = ((WIDTH as f64) * 0.25).round() as i32;
        // Top & bottom margins = 20% of HEIGHT
        let vert_margin: i32 = ((HEIGHT as f64) * 0.20).round() as i32;

        // How many grid-rows remain for buttons + gaps?
        let usable_rows = HEIGHT - 2 * vert_margin;
        // How many grid-cols for each button?
        let button_cols = WIDTH - 2 * side_margin;

        // ─── VERTICAL GAPS ───
        // Reserve 25% of usable_rows for gaps
        let total_gap_rows: i32 =
            ((usable_rows as f64) * 0.25).round() as i32;
        // Distribute those rows into (num_levels - 1) gaps, rounding up so each gap is >=1 when possible
        let gap_rows = if num_levels > 1 {
            (total_gap_rows + (num_levels - 2)) / (num_levels - 1)
        } else {
            0
        };

        // ─── BUTTON HEIGHT ───
        // Remaining rows divided evenly among buttons
        let button_rows: i32 = ((usable_rows - gap_rows * (num_levels - 1)) as f64
            / num_levels as f64)
            .round() as i32;

        // ─── BUILD THE BUTTONS ──
        let mut level_buttons = Vec::with_capacity(num_levels as usize);
        let mut current_y = vert_margin;

        for i in 0..num_levels {
            let top_left     = (side_margin      as f64, current_y       as f64);
            let bottom_right = ( (side_margin + button_cols) as f64,
                                 (current_y   + button_rows)   as f64 );

            level_buttons.push(
                Button::new(
                    &format!("Level {}", i + 1),
                    top_left,
                    bottom_right,
                    SelectLevel(i + 1)
                )
            );

            // advance past this button + one gap
            current_y += button_rows + gap_rows;
        }

        Self {
            name: "Level Selection".to_string(),
            level_buttons,
            activated: false,
        }
    }

    pub fn get_buttons(&self) -> &Vec<Button> {
        &self.level_buttons
    }
}

impl Scene for LevelSelectScene {
    fn get_name(&self) -> &str {
        self.name.as_str()
    }

    fn activate(&mut self) -> Result<(), GameError> {
        if self.activated {
            return Err(GameError::SceneActivationError);
        }
        self.activated = true;
        Ok(())
    }

    fn deactivate(&mut self) -> Result<(), GameError> {
        if !self.activated {
            return Err(GameError::SceneActivationError);
        }
        self.activated = false;
        Ok(())
    }

    fn receive_event(&mut self, event: &GameEvent) {
        todo!()
    }

    fn get_ui_elements(&self) -> Vec<&dyn UIElement> {
        self.level_buttons
            .iter()
            .map(|btn| btn as &dyn UIElement)
            .collect()
    }

    fn render(&self, con: &Context, g: &mut G2d, glyphs: &mut Glyphs) {
        draw_level_selection(&self, con, g, glyphs);
    }
}

pub struct MidLevelScene {
    name: String,
    grid: Vec<Vec<Cell>>,
    player_turn: bool,
    activated: bool,
}

impl MidLevelScene {
    fn new() -> Self {
        Self { name: "Mid-Level".to_string(), grid: vec![], player_turn: true, activated: false }
    }
}

impl Scene for MidLevelScene {
    fn get_name(&self) -> &str {
        self.name.as_str()
    }

    fn activate(&mut self) -> Result<(), GameError> {
        if self.activated {
            return Err(GameError::SceneActivationError);
        }
        self.activated = true;
        Ok(())
    }

    fn deactivate(&mut self) -> Result<(), GameError> {
        if !self.activated {
            return Err(GameError::SceneActivationError);
        }
        self.activated = false;
        Ok(())
    }

    fn receive_event(&mut self, event: &GameEvent) {
        todo!()
    }

    fn get_ui_elements(&self) -> Vec<&dyn UIElement> {
        todo!()
    }

    fn render(&self, con: &Context, g: &mut G2d, glyphs: &mut Glyphs) {
        todo!()
    }

}

pub struct Game {
    scenes: HashMap<String, Box<dyn Scene>>,
    current_scene: String,
}


impl Game {
    pub fn new() -> Self {
        let mut scenes: HashMap<String, Box<dyn Scene>> = HashMap::new();
        let mut start_scene = Box::new(StartScene::new());
        start_scene.activate().unwrap();
        scenes.insert("Start".to_string(), start_scene);
        scenes.insert("Level Selection".to_string(), Box::new(LevelSelectScene::new(3)));
        scenes.insert("Mid-Level".to_string(), Box::new(MidLevelScene::new()));
        Self { scenes, current_scene: "Start".to_string() }
    }

    pub fn switch_scene(&mut self, scene_name: String) -> Result<(), GameError> {
        if self.current_scene == scene_name {
            return Err(GameError::SceneActivationError)
        }
        self.current_scene = scene_name;
        self.scenes.get_mut(self.current_scene.as_str()).unwrap().activate().expect("TODO: panic message");
        println!("Switching scene {}", self.current_scene);
        Ok(())
    }

    pub fn receive_event(&mut self, event: &GameEvent) {
        match event {
            GameEvent::StartGame => {
                if self.current_scene != "Start" { panic!(); }
                if let Err(e) = self.switch_scene("Level Selection".to_string()) {
                    println!("Failed to switch to Mid-Level scene: {:?}", e);
                    panic!();
                }
            }
            SelectLevel(level) => {
                println!("Selected level: {:?}", level);
                if let Err(e) = self.switch_scene("Level Selection".to_string()) {
                    println!("Failed to switch to Mid-Level scene: {:?}", e);
                    panic!();
                }
                todo!()
            },
            GameEvent::EndLevel => {
                todo!()
            }
            _ => {
                panic!()
            }
        }
    }

    pub fn get_current_scene(&self) -> &Box<dyn Scene> {
        self.get_scene(self.current_scene.as_str())
    }

    pub fn get_scene(&self, key: &str) -> &Box<dyn Scene> {
        self.scenes.get(key).unwrap()
    }

    pub fn get_scene_mut(&mut self, key: &str) -> &mut Box<dyn Scene> {
        self.scenes.get_mut(key).unwrap()
    }

    pub fn click_event(&mut self, mouse_x: f64, mouse_y: f64) -> Option<usize> {
        for (index, element) in self.get_ui_elements().iter().enumerate() {
            { // Debug
                println!("Checking element {}", element.get_name());
                println!("Container: {}, {}, {}, {}", element.get_container_coords().0,
                         element.get_container_coords().1, element.get_container_coords().2,
                         element.get_container_coords().3);
            }
            
            if element.cursor_in_container(mouse_x, mouse_y) {
                println!("IN CONTAINER");
                return Some(index);
            } else {
                println!("Missed container");
            }
        }
        None
    }

    pub fn get_ui_elements(&self) -> Vec<&dyn UIElement> {
        self.get_scene(self.current_scene.as_str()).get_ui_elements()
    }

    pub fn get_ui_element(&self, idx: usize) -> Option<&dyn UIElement> {
        self.get_ui_elements().get(idx).copied()
    }

    pub fn render_scene(&self, con: &Context, g: &mut G2d, glyphs: &mut Glyphs) {
        self.get_current_scene().render(con, g, glyphs);
    }
}
