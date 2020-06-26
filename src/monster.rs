use super::attributes::{Stats, VitalPoints};

#[derive(Debug, Clone)]
pub struct Monster {
    pub description: String,
    level: i8,
    pub stats: Stats,
    pub vital_points: VitalPoints,
}

impl Monster {
    pub fn new(description: String, level: i8) -> Self {
        let stats;
        let vital_points;
        // TODO: Random!
        match level {
            1 => {
                stats = Stats{
                    strength: 2,
                    agility: 2,
                    intelligence: 2,
                    will: 2,
                    charisma: 2,
                    intimidation: 2,
                    wealth: 2,
                    resistence: 2,
                };
                vital_points = VitalPoints{
                    life: 4,
                    luck: 4,
                    cardio: 4,
                    social: 4,
                }
            },
            _ => {
                stats = Stats{
                    strength: 2,
                    agility: 2,
                    intelligence: 2,
                    will: 2,
                    charisma: 2,
                    intimidation: 2,
                    wealth: 2,
                    resistence: 2,
                };
                vital_points = VitalPoints{
                    life: 4,
                    luck: 4,
                    cardio: 4,
                    social: 4,
                }
            },
        }

        Self {
            description,
            level,
            stats,
            vital_points,
        }
    }
}