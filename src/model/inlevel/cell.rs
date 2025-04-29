use crate::model::game::{GameError, GameEvent, UIElement, UIEvent};
use crate::model::inlevel::unit;
use crate::model::inlevel::unit::Unit;

pub enum CellError {
    CellOccupied,
    CellNotOccupied
}

pub enum Terrain {
    Plains,
    Forest,
    Mountain
}
pub struct Cell {
    terrain: Terrain,
    unit: Option<Unit>,
    top_left: (f64, f64),
    bottom_right: (f64, f64),
}

impl Cell {
    pub fn new(terrain: Terrain, top_left: (f64, f64), bottom_right: (f64, f64)) -> Cell {
        Cell { terrain, unit: None, top_left, bottom_right }
    }
    
    pub fn has_unit(&self) -> bool {
        self.unit.is_some()
    }
    
    pub fn set_unit(&mut self, unit: Unit) -> Result<(), CellError> {
        if (self.has_unit()) {
            return Err(CellError::CellOccupied);
        }
        self.unit = Some(unit);
        Ok(())
    }
    
    pub fn remove_unit(&mut self) -> Result<(), CellError> {
        if (!self.has_unit()) {
            return Err(CellError::CellNotOccupied);
        }
        self.unit = None;
        Ok(())
    }
    
    pub fn get_terrain(&self) -> &Terrain {
        &self.terrain
    }
}

impl UIElement for Cell {
    fn get_name(&self) -> String {
        format!("Cell at {}, {}", self.top_left.0, self.top_left.1)
    }

    fn update(&self, event: UIEvent) -> Result<Option<GameEvent>, GameError> {
        todo!()
    }

    fn is_selectable(&self) -> bool {
        self.has_unit()
    }

    fn get_top_left(&self) -> (f64, f64) {
        (self.top_left.0, self.top_left.1)
    }

    fn get_bottom_right(&self) -> (f64, f64) {
        (self.bottom_right.0, self.bottom_right.1)
    }

    fn click_event(&self) -> Option<GameEvent> {
        todo!()
    }
}