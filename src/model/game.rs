use crate::model::round::level::Level;
use crate::model::player::Player;
use crate::model::playerselector;
use crate::model::levelselector;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameError {
    PlayerDoesNotExist,
    PlayerAlreadyExists,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GamePhase {
    Menu,
    PlayerSelection,
    LevelSelection,
    InLevel,
    EndLevel,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Screen {
    PlayerSelection(),
    LevelSelection(),
}


///
/// Represents the game state, only can be one instantiated at a time.
pub struct Game {
    levels: Vec<Level>,
    selected_lvl_idx: Option<usize>,
    players: Vec<Player>, // Will never have two players with the same name
    selected_player_idx: Option<usize>,
    game_phase: GamePhase,
}

impl Game {
    pub fn new(levels: Vec<Level>, players: Vec<Player>) -> Game {
        Game { levels, selected_lvl_idx: None, players, selected_player_idx: None,
            game_phase: GamePhase::LevelSelection }
    }

    pub fn change_phase(&mut self, game_phase: GamePhase) {
        self.game_phase = game_phase;
    }

    pub fn game_phase(&self) -> &GamePhase {
        &self.game_phase
    }

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
}