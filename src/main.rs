use rand::Rng;
use rand::rngs::ThreadRng;

use strum_macros::{EnumString, Display};
// used implictly by strum...
use std::str::FromStr;
use std::string::ToString;

use std::collections::HashMap;


mod render;
mod interaction;

#[derive(Debug)]
pub struct Player {
    name: String,
    role: Role,
    profile: Profile,
    attributes: Attributes,
    vital_points: VitalPoints
}

#[derive(Debug)]
struct Attributes {
    strength: i8,
    agility: i8,
    intelligence: i8,
    will: i8,
    charisma: i8,
    intimidation: i8,
    wealth: i8,
    resistence: i8,
}

#[derive(Debug)]
pub struct VitalPoints {
    life: i8,
    luck: i8,
    cardio: i8,
    social: i8,
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


enum AttributesRanges {
    High,
    Medium,
    Poor,
    Horrible
}

pub enum ProfileOptions {
    Brute
}

struct SomeDreamApplication {
    rng: ThreadRng
}

impl SomeDreamApplication {
    pub fn initialize(rng: ThreadRng) -> Self {
        Self {
            rng
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

    fn calculate_vital_points(&mut self, attributes: &Attributes) -> VitalPoints {
        VitalPoints{
            life: attributes.wealth + attributes.strength,
            luck: attributes.agility + attributes.intimidation,
            cardio: attributes.resistence + attributes.will,
            social: attributes.charisma + attributes.intelligence,
        }
    }

    fn roll_status(&mut self, profile: Profile) -> Attributes {
        match profile {
            Profile::Knight => {
                Attributes {
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
                Attributes {
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
                Attributes {
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
                Attributes {
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
                Attributes {
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
                Attributes {
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
                Attributes {
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
                Attributes {
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
                Attributes {
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
                Attributes {
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
                Attributes {
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
                Attributes {
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
                Attributes {
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
                Attributes {
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
                Attributes {
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
                Attributes {
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

    fn onboarding(&mut self) -> Player {

        render::render_splash_screen();

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

        let attributes: Attributes = self.roll_status(profile);

        let vital_points: VitalPoints = self.calculate_vital_points(&attributes);

        let player = Player {
            name,
            role,
            profile,
            attributes,
            vital_points,
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
        self.onboarding()
    }

    fn main_loop(&mut self) {
        let _player: Player = self.onboarding();
        // TODO: dynamic path based in role
        //let mut options: HashMap<i8, String> = HashMap::new();
        //interaction::pick_an_option("Testing", "Sure?", "Ok, nice pick!", options);
        render::render_image_to_ansi("./src/art/fighter.gif");
    }
}

fn main() {
    let mut app = SomeDreamApplication::initialize(rand::thread_rng());
    app.main_loop();
}