use rand::Rng;
use rand::rngs::ThreadRng;

use std::collections::HashMap;
use super::player::*;
use super::interaction;
use super::render;
use super::itens;
use super::attributes::{Stats, VitalPoints};

// used implictly by strum...
use std::str::FromStr;
use std::string::ToString;

pub struct Onboarding {
    rng: ThreadRng,
}

impl Onboarding {
    pub fn init(rng: ThreadRng) -> Self {
        Self {
            rng,
        }
    }
    fn get_attributes(&mut self, ranges: AttributesRanges) -> i8 {
        match ranges {
            AttributesRanges::High => self.rng.gen_range(6, 9),
            AttributesRanges::Medium => self.rng.gen_range(4, 7),
            AttributesRanges::Poor => self.rng.gen_range(2, 5),
            AttributesRanges::Horrible => self.rng.gen_range(0, 3),
        }
    }

    fn calculate_vital_points(&mut self, stats: Stats) -> VitalPoints {
        VitalPoints{
            life: stats.wealth + stats.strength,
            luck: stats.agility + stats.intimidation,
            cardio: stats.resistence + stats.will,
            social: stats.charisma + stats.intelligence,
        }
    }

    fn roll_status(&mut self, profile: Profile) -> Stats {
        match profile {
            Profile::Knight => {
                Stats {
                    strength: self.get_attributes(AttributesRanges::High),
                    agility: self.get_attributes(AttributesRanges::Horrible),
                    intelligence: self.get_attributes(AttributesRanges::Medium),
                    will: self.get_attributes(AttributesRanges::Poor),
                    charisma: self.get_attributes(AttributesRanges::Medium),
                    intimidation: self.get_attributes(AttributesRanges::Poor),
                    wealth: self.get_attributes(AttributesRanges::Medium),
                    resistence: self.get_attributes(AttributesRanges::Poor)
                }
            },
            Profile::Warrior => {
                Stats {
                    strength: self.get_attributes(AttributesRanges::High),
                    agility: self.get_attributes(AttributesRanges::Horrible),
                    intelligence: self.get_attributes(AttributesRanges::Poor),
                    will: self.get_attributes(AttributesRanges::Medium),
                    charisma: self.get_attributes(AttributesRanges::Poor),
                    intimidation: self.get_attributes(AttributesRanges::Medium),
                    wealth: self.get_attributes(AttributesRanges::Poor),
                    resistence: self.get_attributes(AttributesRanges::Medium)
                }
            },
            Profile::Noble => {
                Stats {
                    strength: self.get_attributes(AttributesRanges::Horrible),
                    agility: self.get_attributes(AttributesRanges::High),
                    intelligence: self.get_attributes(AttributesRanges::Medium),
                    will: self.get_attributes(AttributesRanges::Poor),
                    charisma: self.get_attributes(AttributesRanges::Medium),
                    intimidation: self.get_attributes(AttributesRanges::Poor),
                    wealth: self.get_attributes(AttributesRanges::Medium),
                    resistence: self.get_attributes(AttributesRanges::Poor)
                }
            },
            Profile::Rogue => {
                Stats {
                    strength: self.get_attributes(AttributesRanges::Horrible),
                    agility: self.get_attributes(AttributesRanges::High),
                    intelligence: self.get_attributes(AttributesRanges::Poor),
                    will: self.get_attributes(AttributesRanges::Medium),
                    charisma: self.get_attributes(AttributesRanges::Poor),
                    intimidation: self.get_attributes(AttributesRanges::Medium),
                    wealth: self.get_attributes(AttributesRanges::Poor),
                    resistence: self.get_attributes(AttributesRanges::Medium)
                }
            },
            Profile::Mage => {
                Stats {
                    strength: self.get_attributes(AttributesRanges::Poor),
                    agility: self.get_attributes(AttributesRanges::Medium),
                    intelligence: self.get_attributes(AttributesRanges::High),
                    will: self.get_attributes(AttributesRanges::Horrible),
                    charisma: self.get_attributes(AttributesRanges::Poor),
                    intimidation: self.get_attributes(AttributesRanges::Medium),
                    wealth: self.get_attributes(AttributesRanges::Medium),
                    resistence: self.get_attributes(AttributesRanges::Poor)
                }
            },
            Profile::Warlock => {
                Stats {
                    strength: self.get_attributes(AttributesRanges::Medium),
                    agility: self.get_attributes(AttributesRanges::Poor),
                    intelligence: self.get_attributes(AttributesRanges::High),
                    will: self.get_attributes(AttributesRanges::Horrible),
                    charisma: self.get_attributes(AttributesRanges::Medium),
                    intimidation: self.get_attributes(AttributesRanges::Poor),
                    wealth: self.get_attributes(AttributesRanges::Poor),
                    resistence: self.get_attributes(AttributesRanges::Medium)
                }
            },
            Profile::Cleric => {
                Stats {
                    strength: self.get_attributes(AttributesRanges::Medium),
                    agility: self.get_attributes(AttributesRanges::Poor),
                    intelligence: self.get_attributes(AttributesRanges::Horrible),
                    will: self.get_attributes(AttributesRanges::High),
                    charisma: self.get_attributes(AttributesRanges::Medium),
                    intimidation: self.get_attributes(AttributesRanges::Poor),
                    wealth: self.get_attributes(AttributesRanges::Medium),
                    resistence: self.get_attributes(AttributesRanges::Poor)
                }
            },
            Profile::WitchDoctor => {
                Stats {
                    strength: self.get_attributes(AttributesRanges::Poor),
                    agility: self.get_attributes(AttributesRanges::Medium),
                    intelligence: self.get_attributes(AttributesRanges::High),
                    will: self.get_attributes(AttributesRanges::Horrible),
                    charisma: self.get_attributes(AttributesRanges::Poor),
                    intimidation: self.get_attributes(AttributesRanges::Medium),
                    wealth: self.get_attributes(AttributesRanges::Medium),
                    resistence: self.get_attributes(AttributesRanges::Poor)
                }
            },
            Profile::Bard => {
                Stats {
                    strength: self.get_attributes(AttributesRanges::Poor),
                    agility: self.get_attributes(AttributesRanges::Medium),
                    intelligence: self.get_attributes(AttributesRanges::Medium),
                    will: self.get_attributes(AttributesRanges::Poor),
                    charisma: self.get_attributes(AttributesRanges::High),
                    intimidation: self.get_attributes(AttributesRanges::Horrible),
                    wealth: self.get_attributes(AttributesRanges::Poor),
                    resistence: self.get_attributes(AttributesRanges::Medium)
                }
            },
            Profile::Templar => {
                Stats {
                    strength: self.get_attributes(AttributesRanges::Medium),
                    agility: self.get_attributes(AttributesRanges::Poor),
                    intelligence: self.get_attributes(AttributesRanges::Poor),
                    will: self.get_attributes(AttributesRanges::Medium),
                    charisma: self.get_attributes(AttributesRanges::High),
                    intimidation: self.get_attributes(AttributesRanges::Horrible),
                    wealth: self.get_attributes(AttributesRanges::Poor),
                    resistence: self.get_attributes(AttributesRanges::Medium)
                }
            },
            Profile::Assassin => {
                Stats {
                    strength: self.get_attributes(AttributesRanges::Poor),
                    agility: self.get_attributes(AttributesRanges::Medium),
                    intelligence: self.get_attributes(AttributesRanges::Medium),
                    will: self.get_attributes(AttributesRanges::Poor),
                    charisma: self.get_attributes(AttributesRanges::Horrible),
                    intimidation: self.get_attributes(AttributesRanges::High),
                    wealth: self.get_attributes(AttributesRanges::Poor),
                    resistence: self.get_attributes(AttributesRanges::Medium)
                }
            },
            Profile::Executioner => {
                Stats {
                    strength: self.get_attributes(AttributesRanges::Medium),
                    agility: self.get_attributes(AttributesRanges::Poor),
                    intelligence: self.get_attributes(AttributesRanges::Poor),
                    will: self.get_attributes(AttributesRanges::Medium),
                    charisma: self.get_attributes(AttributesRanges::Horrible),
                    intimidation: self.get_attributes(AttributesRanges::High),
                    wealth: self.get_attributes(AttributesRanges::Medium),
                    resistence: self.get_attributes(AttributesRanges::Poor)
                }
            },
            Profile::Hunter => {
                Stats {
                    strength: self.get_attributes(AttributesRanges::Medium),
                    agility: self.get_attributes(AttributesRanges::Poor),
                    intelligence: self.get_attributes(AttributesRanges::Poor),
                    will: self.get_attributes(AttributesRanges::Medium),
                    charisma: self.get_attributes(AttributesRanges::Medium),
                    intimidation: self.get_attributes(AttributesRanges::Poor),
                    wealth: self.get_attributes(AttributesRanges::High),
                    resistence: self.get_attributes(AttributesRanges::Horrible)
                }
            },
            Profile::Druid => {
                Stats {
                    strength: self.get_attributes(AttributesRanges::Poor),
                    agility: self.get_attributes(AttributesRanges::Medium),
                    intelligence: self.get_attributes(AttributesRanges::Medium),
                    will: self.get_attributes(AttributesRanges::Poor),
                    charisma: self.get_attributes(AttributesRanges::Poor),
                    intimidation: self.get_attributes(AttributesRanges::Medium),
                    wealth: self.get_attributes(AttributesRanges::High),
                    resistence: self.get_attributes(AttributesRanges::Horrible)
                }
            },
            Profile::Barbarian => {
                Stats {
                    strength: self.get_attributes(AttributesRanges::Medium),
                    agility: self.get_attributes(AttributesRanges::Poor),
                    intelligence: self.get_attributes(AttributesRanges::Poor),
                    will: self.get_attributes(AttributesRanges::Medium),
                    charisma: self.get_attributes(AttributesRanges::Poor),
                    intimidation: self.get_attributes(AttributesRanges::Medium),
                    wealth: self.get_attributes(AttributesRanges::Horrible),
                    resistence: self.get_attributes(AttributesRanges::High)
                }
            },
            Profile::Shaman => {
                Stats {
                    strength: self.get_attributes(AttributesRanges::Poor),
                    agility: self.get_attributes(AttributesRanges::Medium),
                    intelligence: self.get_attributes(AttributesRanges::Medium),
                    will: self.get_attributes(AttributesRanges::Poor),
                    charisma: self.get_attributes(AttributesRanges::Medium),
                    intimidation: self.get_attributes(AttributesRanges::Poor),
                    wealth: self.get_attributes(AttributesRanges::Horrible),
                    resistence: self.get_attributes(AttributesRanges::High)
                }
            }
            _ => {
                panic!("Unknow role. Panic!")
            }
        }
    }

    pub fn start(&mut self) -> Player {
        let name: String = interaction::capture_input(
            "Hello! Welcome to awesome world of some dream. Tell me your name!",
            "Very well. Your name is correct?",
            "Nice to meet you!",
            vec!(),
        );

        let roles: Vec<String> = vec!(
            Role::Fighter.to_string(),
            Role::Mage.to_string(),
            Role::Survivor.to_string(),
            Role::Hypno.to_string(),
        );

        let main_role: String = interaction::capture_input(
            "Okay, now I want to know what will be your job here",
            "You want to proceed with this info?",
            "",
            roles,
        );

        let role: Role = Role::from_str(main_role.as_str()).unwrap();

        let mut profile_suggestions: Vec<String> = vec!();
        let mut attributes_options: HashMap<i8, String> = HashMap::new();

        match role {
            Role::Fighter => {
                attributes_options.insert(1, String::from("A fighter who wants to kill?"));
                attributes_options.insert(2, String::from("A furtive and gracious killer?"));
            },
            Role::Mage => {
                attributes_options.insert(3, String::from("An intellectual moved by pure math and know the deep art of magic?"));
                attributes_options.insert(4, String::from("A person moved by his beliefs and its intuition?"));
            },
            Role::Hypno => {
                attributes_options.insert(5, String::from("A person with High presence and influence?"));
                attributes_options.insert(6, String::from("A person based in intimidation and manipulation based in its aggression?"));
            },
            Role::Survivor => {
                attributes_options.insert(7, String::from("A wealthy person with great cardio and good costumes"));
                attributes_options.insert(8, String::from("A survivor, hurt by life?"));
            },
        }

        let profiles: i8 = interaction::pick_an_option(
            "Okay, now I want to know what will be your primary attribute, then I will ask you some profiles based on it",
            "Are you sure?",
            "",
            attributes_options,
        );

        match profiles {
            1 => {
                profile_suggestions.push(Profile::Knight.to_string());
                profile_suggestions.push(Profile::Warrior.to_string());
            },
            2 => {
                profile_suggestions.push(Profile::Noble.to_string());
                profile_suggestions.push(Profile::Rogue.to_string());
            },
            3 => {
                profile_suggestions.push(Profile::Mage.to_string());
                profile_suggestions.push(Profile::Warlock.to_string());
            },
            4 => {
                profile_suggestions.push(Profile::Cleric.to_string());
                profile_suggestions.push(Profile::WitchDoctor.to_string());
            },
            5 => {
                profile_suggestions.push(Profile::Bard.to_string());
                profile_suggestions.push(Profile::Templar.to_string());
            },
            6 => {
                profile_suggestions.push(Profile::Assassin.to_string());
                profile_suggestions.push(Profile::Berserker.to_string());
            },
            7 => {
                profile_suggestions.push(Profile::Hunter.to_string());
                profile_suggestions.push(Profile::Druid.to_string());
            },
            8 => {
                profile_suggestions.push(Profile::Barbarian.to_string());
                profile_suggestions.push(Profile::Shaman.to_string());
            },
            _ => panic!("What attribute was missing? {}", profiles)
        }

        let profile: Profile = Profile::from_str(interaction::capture_input(
            "You choose a nice profile. And now, which profile you want? This is the last step of this onboarding",
            "Are you sure?",
            "Ok, let's roll the stats",
            profile_suggestions,
        ).as_str()).unwrap();

        let stats: Stats = self.roll_status(profile);

        let vital_points: VitalPoints = self.calculate_vital_points(stats);

        let player = Player {
            name,
            role,
            profile,
            stats,
            vital_points,
            weapon: itens::basic_weapon()
        };

        render::render_attributes(&player);

        let final_confirmation = interaction::capture_input(
            "Confirm everything? Let's begin?",
            "",
            "Nice!",
            vec!(String::from("YES"), String::from("NO")),
        );

        if final_confirmation == "YES" {
            return player;
        }

        println!("Ok! Let's do it again");
        self.start()
    }
}