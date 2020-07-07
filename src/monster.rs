use super::attributes::{Stats, VitalPoints};
use super::color;

#[derive(Debug, Clone)]
pub struct Monster {
    level: usize,
    pub stats: Stats,
    pub vital_points: VitalPoints,
    pub description: String,
}

impl Monster {
    pub fn attack(&self) -> i8 {
        self.stats.strength
    }
}

pub struct Skeleton {}
pub struct Goblin {}
pub struct Ogre {}

pub trait MonsterType {
    fn description(&self) -> String;
    fn get_attributes(&self, level: usize) -> (Stats, VitalPoints);
}

impl MonsterType for Skeleton {
    fn description(&self) -> String {
        color::paint_text(Box::new(color::Gray{}), "An skeleton with sword and shield")
    }
    fn get_attributes(&self, level: usize) -> (Stats, VitalPoints) {
        vec!((
            Stats{
                strength: 2,
                agility: 3,
                intelligence: 0,
                will: 0,
                charisma: 0,
                intimidation: 2,
                wealth: 1,
                resistence: 2,
            },
            VitalPoints{
                life: 3,
                luck: 0,
                cardio: 4,
                social: 0,
            }
        ))[level]
    }
}

impl MonsterType for Goblin {
    fn description(&self) -> String {
        color::paint_text(Box::new(color::Green{}), "A ugly goblin")
    }
    fn get_attributes(&self, level: usize) -> (Stats, VitalPoints) {
        vec!((
            Stats{
                strength: 2,
                agility: 2,
                intelligence: 1,
                will: 1,
                charisma: 0,
                intimidation: 1,
                wealth: 1,
                resistence: 3,
            },
            VitalPoints{
                life: 2,
                luck: 1,
                cardio: 3,
                social: 0,
            }
        ))[level]
    }
}

impl MonsterType for Ogre {
    fn description(&self) -> String {
        color::paint_text(Box::new(color::Orange{}), "A enourmous ogre")
    }
    fn get_attributes(&self, level: usize) -> (Stats, VitalPoints) {
        vec!((
            Stats{
                strength: 4,
                agility: 1,
                intelligence: 1,
                will: 1,
                charisma: 0,
                intimidation: 3,
                wealth: 3,
                resistence: 3,
            },
            VitalPoints{
                life: 8,
                luck: 1,
                cardio: 8,
                social: 0,
            }
        ))[level]
    }
}

impl Monster {
    pub fn new(monster_type: Box<dyn MonsterType>, level: usize) -> Self {
        // The level is get from vec index, not safety at all for now
        let monster = monster_type.get_attributes(level - 1);

        Self {
            description: monster_type.description(),
            level,
            stats: monster.0,
            vital_points: monster.1,
        }
    }
}