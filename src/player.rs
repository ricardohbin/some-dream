use strum_macros::{EnumString, Display};
use super::attributes::{Stats, VitalPoints};
use super::itens::{WeaponType};

#[derive(Debug, Clone)]
pub struct Player {
    pub name: String,
    pub role: Role,
    pub profile: Profile,
    pub stats: Stats,
    pub vital_points: VitalPoints,
    pub weapon: Option<Box<dyn WeaponType>>,
}

impl Player {
    pub fn attack(&self) -> i8 {
        match &self.weapon {
            None => self.stats.strength,
            Some(w) => {
                w.attack(self.stats).0
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumString, Display)]
#[strum(serialize_all = "snake_case")]
pub enum Role {
    Fighter,
    Mage,
    Survivor,
    Hypno
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumString, Display)]
#[strum(serialize_all = "snake_case")]
pub enum Profile {
    Knight,
    Warrior,
    Noble,
    Rogue,
    Mage,
    Warlock,
    Cleric,
    WitchDoctor,
    Bard,
    Templar,
    Assassin,
    Executioner,
    Hunter,
    Druid,
    Barbarian,
    Shaman,
    Berserker,
}


pub enum AttributesRanges {
    High,
    Medium,
    Poor,
    Horrible
}