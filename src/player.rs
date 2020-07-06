use strum_macros::{EnumString, Display};
use super::attributes::{Stats, VitalPoints};

#[derive(Debug, Clone)]
pub struct Player {
    pub name: String,
    pub role: Role,
    pub profile: Profile,
    pub stats: Stats,
    pub vital_points: VitalPoints
}

impl Player {
    pub fn attack(&self) -> i8 {
        self.stats.strength
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