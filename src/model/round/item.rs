pub enum Item {
    Ore(Ore),
    Herb(Herb),
    Gem(Gem),
    Gold,
}

pub enum Ore {
    IronOre,
    SilverOre,
    PlatinumOre,
}

pub enum Herb {
    RestorationHerb,
    EnchantedHerb,
    PowerHerb,
}

pub enum Gem {
    Ruby,
    Sapphire,
    Emerald,
    Topaz,
}

impl Item {
    pub fn is_ore(&self) -> bool {
        matches!(self, Item::Ore(_))
    }

    pub fn is_herb(&self) -> bool {
        matches!(self, Item::Herb(_))
    }

    pub fn is_gem(&self) -> bool {
        matches!(self, Item::Gem(_))
    }
}