use rand::Rng;
use rand::rngs::ThreadRng;

use super::color;
use super::attributes::{Stats};

#[derive(Debug, Clone)]
pub enum Kind {
    Weapon,
    Potion
}

pub trait WeaponType: WeaponTypeClone {
    fn attack(&self, stats: Stats) -> (i8, DamageType);
    fn show_power(&self) -> i8;
}

pub trait WeaponTypeClone {
    fn clone_box(&self) -> Box<dyn WeaponType>;
}

impl<T> WeaponTypeClone for T
	where T: 'static + WeaponType + Clone
{
	fn clone_box(&self) -> Box<dyn WeaponType> {
		Box::new(self.clone())
	}
}

// Explict Clone trait to Box<dyn WeaponType>
impl Clone for Box<dyn WeaponType> {
	fn clone(&self) -> Box<dyn WeaponType> {
        self.clone_box()
	}
}

// Explict Debug trait to Box<dyn WeaponType>
impl std::fmt::Debug for Box<dyn WeaponType> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "power: {}", self.show_power())
    }
}

impl WeaponType for Sword {
    fn attack(&self, stats: Stats) -> (i8, DamageType) {
        (self.power + (stats.strength + stats.agility / 2), self.damage_type)
    }
    fn show_power(&self) -> i8 {
        self.power
    }
}

impl WeaponType for Mace {
    fn attack(&self, stats: Stats) -> (i8, DamageType) {
        (self.power + stats.strength, self.damage_type)
    }
    fn show_power(&self) -> i8 {
        self.power
    }
}

impl WeaponType for Lance {
    fn attack(&self, stats: Stats) -> (i8, DamageType) {
        (self.power + stats.agility, self.damage_type)
    }
    fn show_power(&self) -> i8 {
        self.power
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
    pub power: i8,
    pub damage_type: DamageType,
}

#[derive(Debug, Clone)]
struct Mace {
    pub power: i8,
    pub damage_type: DamageType,
}

#[derive(Debug, Clone)]
struct Lance {
    pub power: i8,
    pub damage_type: DamageType,
}

#[derive(Debug, Clone)]
pub struct Item {
    pub can_be_evil: bool,
    pub description: String,
    pub used_description: String,
    pub kind: Kind,
    pub is_used: bool,
    pub weapon: Option<Box<dyn WeaponType>>
}

#[derive(Debug, Clone)]
pub struct ItemFactory {
    rng: ThreadRng,
    encounters: Vec<Item>,
}

fn create_generic_sword(power: i8) -> Sword {
    Sword {
        power,
        damage_type: DamageType::Slash
    }
}

fn create_generic_lance(power: i8) -> Lance {
    Lance {
        power,
        damage_type: DamageType::Piercing
    }
}

fn create_generic_mace(power: i8) -> Mace {
    Mace {
        power,
        damage_type: DamageType::Bash
    }
}

impl ItemFactory {
    pub fn new(rng: &mut ThreadRng) ->  Self {
        let encounters = vec!(
            Item {
                can_be_evil: true,
                description: color::paint_text(Box::new(color::Red{}), "A misterious red potion"),
                used_description: color::paint_text(Box::new(color::Gray{}), "An empty potion"),
                kind: Kind::Potion,
                is_used: false,
                weapon: None
            },
            Item {
                can_be_evil: false,
                description: color::paint_text(Box::new(color::Gray{}), "A rust short sword"),
                used_description: color::paint_text(Box::new(color::Gray{}), "An empty case"),
                kind: Kind::Weapon,
                is_used: false,
                weapon: Some(Box::new(create_generic_sword(rng.gen_range(3, 5))))
            },
            Item {
                can_be_evil: false,
                description: color::paint_text(Box::new(color::Gray{}), "A rust lance"),
                used_description: color::paint_text(Box::new(color::Gray{}), "An empty case"),
                kind: Kind::Weapon,
                is_used: false,
                weapon: Some(Box::new(create_generic_lance(rng.gen_range(3, 5))))
            },
            Item {
                can_be_evil: false,
                description: color::paint_text(Box::new(color::Gray{}), "A rust mace"),
                used_description: color::paint_text(Box::new(color::Gray{}), "An empty case"),
                kind: Kind::Weapon,
                is_used: false,
                weapon: Some(Box::new(create_generic_mace(rng.gen_range(3, 5))))
            },
        );

        Self {
            rng: *rng,
            encounters
        }
    }

    pub fn get_one(&mut self) -> Item {
        let encounter_count = self.encounters.len();
        let index = self.rng.gen_range(0, encounter_count);
        self.encounters.get(index).unwrap().clone()
    }
}