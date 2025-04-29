use std::collections::HashMap;

const MAX_STAT: u32 = 30;

#[derive(Debug, Hash, PartialEq, Eq)]
pub enum Stat {
    Strength,
    Magic,
    Agility,
    Precision,
    Defense,
    Resistance,
    Wisdom
}

impl Stat {
    pub fn get_description(&self) -> &str {
        match self {
            Stat::Strength => "Increases damage with physical weapons",
            Stat::Magic => "Increases damage with magical weapons",
            Stat::Agility => "Increases dodge chance and lowers the chance of receiving critical damage",
            Stat::Precision => "Increases accuracy and critical chance",
            Stat::Defense => "Decreases damage from physical sources",
            Stat::Resistance => "Decreases damage from magical sources",
            Stat::Wisdom => "Increases experience gain"
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq)]
pub enum Class {
    Swordsman,
    Archer,
    Mage,
    Cleric
}

impl Class {
    pub fn base_stats(&self) -> HashMap<Stat, u32> {
        match self {
            Class::Swordsman => [
                (Stat::Strength, 7),
                (Stat::Magic, 2),
                (Stat::Agility, 5),
                (Stat::Precision, 4),
                (Stat::Defense, 9),
                (Stat::Resistance, 5),
                (Stat::Wisdom, 5)
            ].into_iter().collect(),
            Class::Archer => [
                    (Stat::Strength, 6),
                    (Stat::Magic, 3),
                    (Stat::Agility, 6),
                    (Stat::Precision, 7),
                    (Stat::Defense, 5),
                    (Stat::Resistance, 6),
                    (Stat::Wisdom, 5)
                ].into_iter().collect(),
            Class::Mage => [
                (Stat::Strength, 2),
                (Stat::Magic, 7),
                (Stat::Agility, 3),
                (Stat::Precision, 6),
                (Stat::Defense, 2),
                (Stat::Resistance, 7),
                (Stat::Wisdom, 5)
            ].into_iter().collect(),
            Class::Cleric => [
                (Stat::Strength, 1),
                (Stat::Magic, 7),
                (Stat::Agility, 5),
                (Stat::Precision, 3),
                (Stat::Defense, 3),
                (Stat::Resistance, 4),
                (Stat::Wisdom, 5)
            ].into_iter().collect(),
        }
    }
}


pub struct Unit {
    name: String,
    class: Class,
    level: u32,
    hp: u32,
    max_hp: u32,
    stamina: u32,
    max_stamina: u32,
    stat_map: HashMap<String, Stat>,
}

impl Unit {
    pub fn new(&self, name: &str, class: Class) -> Self {
        
    }
}