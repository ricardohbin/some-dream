use strum_macros::{EnumString, Display};

#[derive(Debug)]
pub struct Player {
    pub name: String,
    pub role: Role,
    pub profile: Profile,
    pub attributes: Attributes,
    pub vital_points: VitalPoints
}

#[derive(Debug)]
pub struct Attributes {
    pub strength: i8,
    pub agility: i8,
    pub intelligence: i8,
    pub will: i8,
    pub charisma: i8,
    pub intimidation: i8,
    pub wealth: i8,
    pub resistence: i8,
}

#[derive(Debug)]
pub struct VitalPoints {
    pub life: i8,
    pub luck: i8,
    pub cardio: i8,
    pub social: i8,
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