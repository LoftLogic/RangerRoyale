///
/// Players associated difficulty (Associated implementation not designed yet)
///
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

///
/// The player in the game, and their progression and settings. There can be more than one player
/// but only two players can be playing at once.
///
#[derive(Debug, PartialEq, Eq, Hash)]
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

    pub fn level_up(&mut self) {
        self.highest_lvl = self.highest_lvl + 1;
    }
}