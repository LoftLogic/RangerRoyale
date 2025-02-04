use std::fmt;
use std::collections::HashSet;

use crate::model::round::attack::DamageType;

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


const MAX_STAT: u32 = 30;

///
/// One of five stats a unit can have (excluding Armor and Resistance as they follow different rules
///
pub enum Stat {
    Health,
    Stamina,
    Strength,
    Defense,
    Speed
}


///
/// Represents a unit on the game board. Controlled by a player. Can move, attack, and use abilities.
///
#[derive(Debug)]
pub struct Unit {
    name: String,
    max_health: u32,
    health: u32,
    max_stamina: u32,
    stamina: u32,
    strength: u32,
    defense: u32,
    speed: u32,
    agility: u32,
    armor: Protection,
    resistance: Protection,
}

impl Unit {

    pub fn new(name: String, max_health: u32, health: u32, max_stamina: u32, stamina: u32, strength: u32,
       defense: u32, movement: u32, agility: u32, armor: Protection, resistance: Protection) -> Result<Unit, UnitError> {
        if (health > 100 || stamina > MAX_STAT || strength > MAX_STAT || defense > MAX_STAT || movement > MAX_STAT) {
            return Err(UnitError::StatTooHigh);
        }
        Ok(Unit { name, max_health, health, max_stamina, stamina, strength, defense, speed: movement, agility, armor, resistance })
    }

    // GETTER METHODS:
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
        self.speed
    }

    pub fn armor(&self) -> &Protection {
        &self.armor
    }

    pub fn resistance(&self) -> &Protection {
        &self.resistance
    }

    ///
    /// Heals the given amount. Cannot increase health past max health.
    /// Is not affected by any resistances.
    ///
    /// # Arguments
    /// * `health` to heal
    ///
    /// # Returns
    /// * UnitError if unit is dead, otherwise Ok
    ///
    pub fn add_health(&mut self, health: u32) -> Result<(), UnitError> {
        if (self.health == 0) {
            return Err(UnitError::UnitNotAlive);
        }
        self.health = self.max_health.min(self.health + health);
        Ok(())
    }


    ///
    /// Takes the given amount of damage. Based on the damage type and unit resistance,
    /// damage may be reduced.
    ///
    /// # Arguments
    /// * `damage` to take
    /// * `damage_type` of the damage (for resistances)
    ///
    /// # Return
    /// * UnitError if unit is dead, otherwise Ok
    ///
    pub fn take_damage(&mut self, damage: u32, damage_type: DamageType) -> Result<(), UnitError> {
        if (self.health == 0) {
            return Err(UnitError::UnitNotAlive);
        }
        let damage_taken: u32 = match damage_type {
            DamageType::Physical => damage * ((self.armor.blocking_power() as f64) / 100.0) as u32,
            DamageType::Elemental => damage * ((self.resistance.blocking_power() as f64) / 100.0) as u32,
            DamageType::True => damage
        };
        if (damage_taken > self.health) {
            self.health = 0;
            return Ok(());
        }
        self.health -= damage_taken;
        Ok(())
    }

    // Private abstraction variable
    fn set_stat(&mut self, value: u32, mut stat: u32) -> Result<(), UnitError> {
        if self.health == 0 {
            return Err(UnitError::UnitNotAlive);
        }
        if value > MAX_STAT {
            return Err(UnitError::StatTooHigh);
        }
        stat = value;
        Ok(())
    }

    ///
    /// Sets strength to the given value.
    ///
    /// # Arguments
    /// * `strength` to set to
    ///
    /// # Returns
    /// * UnitError if stat is too high or unit is not alive, otherwise Ok
    ///
    pub fn set_strength(&mut self, strength: u32) -> Result<(), UnitError> {
        self.set_stat(strength, self.strength)
    }

    ///
    /// Sets stamina to the given value.
    ///
    /// # Arguments
    /// * `stamina` to set to
    ///
    /// # Returns
    /// * UnitError if stat is too high or unit is not alive, otherwise Ok
    ///
    pub fn set_stamina(&mut self, stamina: u32) -> Result<(), UnitError> {
        self.set_stat(stamina, self.stamina)
    }

    ///
    /// Sets defense to the given value.
    ///
    /// # Arguments
    /// * `defense` to set to
    ///
    /// # Returns
    /// * UnitError if stat is too high or unit is not alive, otherwise Ok
    ///
    pub fn set_defense(&mut self, defense: u32) -> Result<(), UnitError> {
        self.set_stat(defense, self.defense)
    }

    ///
    /// Sets speed to the given value.
    ///
    /// # Arguments
    /// * `speed` to set to
    ///
    /// # Returns
    /// * UnitError if stat is too high or unit is not alive, otherwise Ok
    ///
    pub fn set_speed(&mut self, speed: u32) -> Result<(), UnitError> {
        self.set_stat(speed, self.speed)
    }
}