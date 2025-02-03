use std::fmt;
use std::collections::HashSet;

#[derive(Debug)]
pub enum UnitError {
    StatTooHigh,
    UnitNotAlive,
}

///
/// One of four protection tiers a character can have for defense or resistance.
///
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Protection {
    Light,
    Medium,
    Heavy,
    SuperHeavy,
}

impl Protection {
    pub fn blocking_power(&self) -> u8 {
        match *self {
            Protection::Light => 20,
            Protection::Medium => 45,
            Protection::Heavy => 75,
            Protection::SuperHeavy => 95,
        }
    }
}


///
/// One of five stats a unit can have (excluding Armor and Resistance as they follow different rules
///
pub enum Stat {
    Health,
    Stamina,
    Strength,
    Defense,
    Movement
}

impl fmt::Display for Protection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let text = match self {
            Protection::Light => "Light",
            Protection::Medium => "Medium",
            Protection::Heavy => "Heavy",
            Protection::SuperHeavy => "SuperHeavy",
        };
        write!(f, "{}", text)
    }
}

///
/// Represents a unit on the game board. Controlled by a player. Can move, attack, and use abilities.
///
#[derive(Debug)]
pub struct Unit {
    name: String,
    health: u32,
    stamina: u32,
    strength: u32,
    defense: u32,
    movement: u32,
    agility: u32,
    armor: Protection,
    resistance: Protection,
}

impl Unit {

    pub fn new(name: String, health: u32, stamina: u32, strength: u32,
       defense: u32, movement: u32, agility: u32, armor: Protection, resistance: Protection) -> Result<Unit, UnitError> {
        if (health > 100 || stamina > 30 || strength > 30 || defense > 30 || movement > 30) {
            return Err(UnitError::StatTooHigh);
        }
        Ok(Unit { name, health, stamina, strength, defense, movement, agility, armor, resistance })
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn health(&self) -> u32 {
        self.health
    }

    pub fn stamina(&self) -> u32 {
        self.stamina
    }

    pub fn strength(&self) -> u32 {
        self.strength
    }

    pub fn defense(&self) -> u32 {
        self.defense
    }

    pub fn movement(&self) -> u32 {
        self.movement
    }

    pub fn armor(&self) -> &Protection {
        &self.armor
    }

    pub fn resistance(&self) -> &Protection {
        &self.resistance
    }

    pub fn set_health(&mut self, health: u32) -> Result<(), UnitError> {
        if (self.health == 0) {
            return Err(UnitError::StatTooHigh);
        }
        if (health > 100) {
            return Err(UnitError::StatTooHigh);
        }
        self.health = health;
        Ok(())
    }

    // Shorthand version of set_health
    pub fn take_damage(&mut self, damage: u32) -> Result<(), UnitError> {
        if (self.health == 0) {
            return Err(UnitError::UnitNotAlive);
        }
        if (damage > self.health) {
            self.health = 0;
            return Ok(());
        }
        self.health -= damage;
        Ok(())
    }

    // Shorthand version of set_health
    pub fn heal_health(&mut self, healing: u32) -> Result<(), UnitError> {
        if (self.health == 0) {
            return Err(UnitError::UnitNotAlive);
        }
        if (self.health + healing > 100) {
            return Err(UnitError::StatTooHigh);
        }
        Ok(())
    }

    fn set_stat(&mut self, value: u32, max: u32, mut stat: u32) -> Result<(), UnitError> {
        if self.health == 0 {
            return Err(UnitError::StatTooHigh);
        }
        if value > max {
            return Err(UnitError::StatTooHigh);
        }
        stat = value;
        Ok(())
    }

    pub fn set_strength(&mut self, strength: u32) -> Result<(), UnitError> {
        self.set_stat(strength, 30, self.strength)
    }

    pub fn set_stamina(&mut self, stamina: u32) -> Result<(), UnitError> {
        self.set_stat(stamina, 30, self.stamina)
    }

    pub fn set_defense(&mut self, defense: u32) -> Result<(), UnitError> {
        self.set_stat(defense, 30, self.defense)
    }

    pub fn set_movement(&mut self, movement: u32) -> Result<(), UnitError> {
        self.set_stat(movement, 30, self.movement)
    }

}