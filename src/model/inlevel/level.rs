use crate::model::inlevel::cell::Cell;
use std::collections::HashSet;
use crate::model::player;
use crate::model::player::Player;

///
/// Different special level settings (default levels will have none)
///
#[derive(Debug, Hash, PartialEq, Eq)]
pub enum Rule {
    LowIntel,
    FullIntel,
}


impl Rule {
    pub fn get_description(&self) -> String {
        match self {
            Rule::LowIntel => "Visibility lowered by one square".to_string(),
            Rule::FullIntel => "Player can see the whole board".to_string(),
        }
    }
}


///
/// Represents a particular level a player can play on, with associated map and rules
///
pub struct Level {
    name: String,
    grid: Vec<Vec<Cell>>,
    rules: HashSet<Rule>,
    in_progress: bool,
    enemy: Player,
}

impl Level {
    pub fn new(name: &str, grid: Vec<Vec<Cell>>, rules: HashSet<Rule>, enemy: Player) -> Level {
        Level { name: name.to_string(), grid, rules, in_progress: false, enemy }
    }

    pub fn start(&mut self) {
        self.in_progress = true;
    }

    pub fn pause(&mut self) {
        self.in_progress = false;
    }

    pub fn in_progress(&self) -> bool {
        self.in_progress
    }

    pub fn add_rule(&mut self, rule: Rule) {
        self.rules.insert(rule);
    }

    pub fn remove_rule(&mut self, rule: &Rule) {
        if (self.rules.contains(rule)) {
            self.rules.remove(rule);
        }
    }

    pub fn get_rules(&self) -> &HashSet<Rule> {
        &self.rules
    }

    pub fn get_grid(&self) -> &Vec<Vec<Cell>> {
        &self.grid
    }
}