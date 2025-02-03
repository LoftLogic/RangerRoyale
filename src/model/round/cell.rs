use std::fmt;
use crate::model::round::item::Item;
use crate::model::round::unit::{Protection, Unit};

pub enum CellError {
    CellTaken,
    CellNotOccupied,
}

///
/// One of the biomes a cell can have (the environment its in)
///
pub enum Biome {
    Plain,
    Forest,
    Structure,
    Water,
    Shallow,
    Mud,
    Stronghold,
    Tower,
    Brewery,
    Blacksmith,
}

impl Biome {

    pub fn is_strategic_point(&self) -> bool {
        matches!(self, Biome::Blacksmith | Biome::Stronghold | Biome::Tower | Biome::Brewery)
    }

    pub fn is_walkable(&self) -> bool {
        !(matches!(self, Biome::Water))
    }

    pub fn movement_modifier(&self) -> u8 {
        match self {
            Biome::Plain => 100,
            Biome::Forest => 100,
            Biome::Structure => 100,
            Biome::Water => 100,
            Biome::Shallow => 50,
            Biome::Mud => 33,
            Biome::Stronghold => 100,
            Biome::Tower => 100,
            Biome::Brewery => 100,
            Biome::Blacksmith => 100,
        }
    }
}

impl fmt::Display for Biome {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let text = match self {
            Biome::Plain => "A basic plot of land",
            Biome::Forest => "The forest adds basic cover from ranged attacks",
            Biome::Structure => "The structure provides great cover from ranged attacks",
            Biome::Water => "The water is uncrossable on foot",
            Biome::Shallow => "The water reaches the knees, slowing the men down",
            Biome::Mud => "The mud greatly slows the men down",
            Biome::Stronghold => "The stronghold houses men and troops. It is easily defended.",
            Biome::Tower => "The tower gives bonus attack range for whoever stands in it.",
            Biome::Brewery => "The brewery allows the holder to create potions",
            Biome::Blacksmith => "The blacksmith allows the holder to create weapons"
        };
        write!(f, "{}", text)
    }
}

///
/// A spot on the map. Has a biome and can have a unit or an item (Never both).
///
pub struct Cell {
    biome: Biome,
    unit: Option<Unit>,
    item: Option<Item>
}

impl Cell {
    pub fn new(biome: Biome, unit: Option<Unit>, item: Option<Item>) -> Self {
        Self { biome, unit, item }
    }

    pub fn biome(&self) -> &Biome {
        &self.biome
    }

    pub fn unit(&self) -> &Option<Unit> {
        &self.unit
    }

    pub fn has_unit(&self) -> bool {
        self.unit.is_some()
    }

    pub fn has_item(&self) -> bool {
        self.item.is_some()
    }

    pub fn remove_item(&mut self) {
        self.item = None;
    }

    pub fn add_unit(&mut self, unit: Unit) -> Result<(), CellError> {
        if (self.has_unit()) {
            return Err(CellError::CellTaken);
        }
        self.remove_item();
        self.unit = Some(unit);
        Ok(())
    }

    // Throws an error if there is no unit inside
    pub fn remove_unit(&mut self) -> Result<(), CellError> {
        if self.has_unit() {
            return Err(CellError::CellNotOccupied);
        }
        self.unit = None;
        Ok(())
    }

}