pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

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