use crate::model::round::cell::Cell;
use std::collections::HashSet;

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

pub struct Level {
    name: String,
    grid: Vec<Vec<Cell>>,
    rules: HashSet<Rule>,
    in_progress: bool
}

impl Level {
    pub fn new(name: &str, grid: Vec<Vec<Cell>>, rules: HashSet<Rule>) -> Level {
        Level { name: name.to_string(), grid, rules, in_progress: false }
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
}