///
/// Players associated difficulty (Associated implementation not designed yet)
///
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

///
/// The player in the game, and their progression and settings. There can be more than one player
/// but only two players can be playing at once.
///
pub struct Player {
    name: String,
    difficulty: Difficulty,
    highest_lvl: u32,
}

impl Player {
    pub fn new(name: String, difficulty: Difficulty) -> Player {
        Player{ name, difficulty, highest_lvl: 0 }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}