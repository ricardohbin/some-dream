use rand::Rng;
use rand::rngs::ThreadRng;

use super::color;


#[derive(Debug, Clone)]
pub enum Kind {
    Weapon,
    Potion
}

#[derive(Debug, Clone)]
pub struct Encounter {
    pub can_be_evil: bool,
    pub description: String,
    pub used_description: String,
    pub kind: Kind,
    pub is_used: bool,
}

#[derive(Debug, Clone)]
pub struct EncounterFactory {
    rng: ThreadRng,
    encounters: Vec<Encounter>,
}

impl EncounterFactory {
    pub fn new(rng: ThreadRng) ->  Self {
        let encounters = vec!(
            Encounter {
                can_be_evil: true,
                description: color::paint_text(Box::new(color::Red{}), "A misterious red potion"),
                used_description: color::paint_text(Box::new(color::Gray{}), "An empty potion"),
                kind: Kind::Potion,
                is_used: false
            },
            Encounter {
                can_be_evil: true,
                description: color::paint_text(Box::new(color::Gray{}), "A shining long sword"),
                used_description: color::paint_text(Box::new(color::Gray{}), "A shining long sword"),
                kind: Kind::Weapon,
                is_used: false
            }
        );

        Self {
            rng,
            encounters
        }
    }

    pub fn get_one(&mut self) -> Encounter {
        let encounter_count = self.encounters.len();
        let index = self.rng.gen_range(0, encounter_count);
        self.encounters.get(index).unwrap().clone()
    }
}