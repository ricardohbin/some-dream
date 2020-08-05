use rand::Rng;
use rand::rngs::ThreadRng;

use super::color;
use super::attributes::{Stats};

#[derive(Debug, Clone)]
pub enum Kind {
    Weapon,
    Potion
}

pub trait WeaponType {
    fn new(power: i8) -> Self;
    fn attack(&self, stats: Stats) -> (i8, DamageType);
}

impl WeaponType for Sword {
    fn new(power: i8) -> Self {
        Self {
            power,
            damage_type: DamageType::Slash
        }
    }
    fn attack(&self, stats: Stats) -> (i8, DamageType) {
        (self.power + (stats.strength + stats.agility / 2), self.damage_type)
    }
}

impl WeaponType for Mace {
    fn new(power: i8) -> Self {
        Self {
            power,
            damage_type: DamageType::Bash
        }
    }
    fn attack(&self, stats: Stats) -> (i8, DamageType) {
        (self.power + stats.strength, self.damage_type)
    }
}

impl WeaponType for Lance {
    fn new(power: i8) -> Self {
        Self {
            power,
            damage_type: DamageType::Bash
        }
    }
    fn attack(&self, stats: Stats) -> (i8, DamageType) {
        (self.power + stats.agility, self.damage_type)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum DamageType {
    Bash,
    Piercing,
    Slash,
}

#[derive(Debug, Clone)]
struct Sword {
    power: i8,
    damage_type: DamageType,
}

#[derive(Debug, Clone)]
struct Mace {
    power: i8,
    damage_type: DamageType,
}

#[derive(Debug, Clone)]
struct Lance {
    power: i8,
    damage_type: DamageType,
}

#[derive(Debug, Clone)]
pub struct Item {
    pub can_be_evil: bool,
    pub description: String,
    pub used_description: String,
    pub kind: Kind,
    pub is_used: bool,
    sword: Option<Sword>,
}

#[derive(Debug, Clone)]
pub struct ItemFactory {
    rng: ThreadRng,
    encounters: Vec<Item>,
}

impl ItemFactory {
    pub fn new(rng: ThreadRng) ->  Self {
        let encounters = vec!(
            Item {
                can_be_evil: true,
                description: color::paint_text(Box::new(color::Red{}), "A misterious red potion"),
                used_description: color::paint_text(Box::new(color::Gray{}), "An empty potion"),
                kind: Kind::Potion,
                is_used: false,
                sword: None
            },
            Item {
                can_be_evil: true,
                description: color::paint_text(Box::new(color::Gray{}), "A shining long sword"),
                used_description: color::paint_text(Box::new(color::Gray{}), "A shining long sword"),
                kind: Kind::Weapon,
                is_used: false,
                sword: Option::from(Sword::new(10))
            }
        );

        Self {
            rng,
            encounters
        }
    }

    pub fn get_one(&mut self) -> Item {
        let encounter_count = self.encounters.len();
        let index = self.rng.gen_range(0, encounter_count);
        self.encounters.get(index).unwrap().clone()
    }
}