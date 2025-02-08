use std::collections::HashSet;

use crate::model::player::Player;
use crate::model::menu::Menu;
use crate::model::playerselector::PlayerSelector;
use crate::model::levelselector::LevelSelector;
use crate::model::inlevel::level::Level;
use crate::model::inlevel::cell::Cell;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameError {
    PlayerDoesNotExist,
    PlayerAlreadyExists,
    WrongScreen,
    ScreenAlreadySelected
}

impl GameError {
    pub fn recoverable(&self) -> bool {
        match self {
            GameError::WrongScreen => true,
            GameError::ScreenAlreadySelected => true,
            GameError::PlayerDoesNotExist => true,
            GameError::PlayerAlreadyExists => true,
        }
    }
}

#[derive(Debug, Copy, PartialEq, Eq, Hash)]
pub enum Screen {
    PlayerSelection(),
    LevelSelection(),
    InLevel(),
    EndLevel(),
}


///
/// Represents the game state, only can be one instantiated at a time.
pub struct Game {
    // levels: Vec<Level>,
    // selected_lvl_idx: Option<usize>,
    level: Level, // temp
    player: Player, // temp
    players: Vec<Player>,
    current_screen: Screen,
}

impl Game {
    pub fn new(level: Level, player: Player) -> Game {
        Game {  level, player, players: vec![player], current_screen: Screen::InLevel() }
    }

    ///
    /// Adds the given player
    ///
    /// # Arguments
    /// * `player` to add
    ///
    /// # Returns
    /// * GameError if the player is already stored (checking by name)
    /// * Ok() otherwise
    ///
    pub fn add_player(&mut self, player: Player) -> Result<(), GameError> {
        /* The below code snippet is an optimized version of:
         let mut names: Vec<&str> = Vec::new();
        for p in &self.players {
            names.push(p.name());
        }
        if names.contains(&player.name()) {
            return Err(GameError::PlayerAlreadyExists);
        }
        */
        if self.players.iter().any(|p| p.name() == player.name()) {
            return Err(GameError::PlayerAlreadyExists);
        }
        self.players.push(player);
        Ok(())
    }

    pub fn remove_player(&mut self, player: Player) -> Result<(), GameError> {
        let original_len = self.players.len();

        self.players.retain(|p| p.name() != player.name());

        if self.players.len() == original_len {
            return Err(GameError::PlayerDoesNotExist);
        }
        Ok(())
    }


    ///
    /// Changes the screen to the given screen
    ///
    /// # Arguments
    /// * `screen` to change to
    ///
    /// # Returns
    /// * GameError if the given screen is already selected
    /// * Ok() otherwise
    pub fn change_screen(&mut self, screen: Screen) -> Result<(), GameError> {
        if (self.current_screen == screen) {
            return Err(GameError::ScreenAlreadySelected);
        }
        self.current_screen = screen;
        Ok(())
    }

    pub fn get_screen(&self) -> Screen {
        self.current_screen
    }

    pub fn get_grid(&self) -> Result<&Vec<Vec<Cell>>, GameError> {
        if self.current_screen != Screen::InLevel() {
            return Err(GameError::WrongScreen);
        }
        return Ok(self.level.get_grid())
    }
}