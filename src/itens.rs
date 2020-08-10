use rand::Rng;
use rand::rngs::ThreadRng;

use super::color;
use super::attributes::{Stats};

#[derive(Debug, Clone)]
pub enum Kind {
    Weapon,
    Potion
}

trait WeaponType: WeaponTypeClone {
    fn attack(&self, stats: Stats) -> (i8, DamageType);
}

trait WeaponTypeClone {
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
        let x = self.clone_box();
        return x;
	}
}

// Explict Debug trait to Box<dyn WeaponType> - Not working propertly
impl std::fmt::Debug for Box<dyn WeaponType> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "-")
    }
}

impl WeaponType for Sword {
    fn attack(&self, stats: Stats) -> (i8, DamageType) {
        (self.power + (stats.strength + stats.agility / 2), self.damage_type)
    }
}

impl WeaponType for Mace {
    fn attack(&self, stats: Stats) -> (i8, DamageType) {
        (self.power + stats.strength, self.damage_type)
    }
}

impl WeaponType for Lance {
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
    weapon: Box<dyn WeaponType>
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
                weapon: Box::new(Sword {
                        power: 10,
                        damage_type: DamageType::Slash
                })
            },
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