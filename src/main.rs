use rand::Rng;
use rand::rngs::ThreadRng;

use strum_macros::{EnumString, Display};
// used implictly by strum...
use std::str::FromStr;
use std::string::ToString;

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
enum Role {
    Fighter,
    Mage,
    Survivor,
    Hypno
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumString, Display)]
#[strum(serialize_all = "snake_case")]
enum Profile {
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
    HIGH,
    MEDIUM,
    POOR,
    HORRIBLE
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
            AttributesRanges::HIGH => self.rng.gen_range(6, 9),
            AttributesRanges::MEDIUM => self.rng.gen_range(4, 7),
            AttributesRanges::POOR => self.rng.gen_range(2, 5),
            AttributesRanges::HORRIBLE => self.rng.gen_range(0, 3),
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
                    strength: self.get_attributes(AttributesRanges::HIGH),
                    agility: self.get_attributes(AttributesRanges::HORRIBLE),
                    intelligence: self.get_attributes(AttributesRanges::MEDIUM),
                    will: self.get_attributes(AttributesRanges::POOR),
                    charisma: self.get_attributes(AttributesRanges::MEDIUM),
                    intimidation: self.get_attributes(AttributesRanges::POOR),
                    wealth: self.get_attributes(AttributesRanges::MEDIUM),
                    resistence: self.get_attributes(AttributesRanges::POOR)
                }
            },
            Profile::Warrior => {
                Attributes {
                    strength: self.get_attributes(AttributesRanges::HIGH),
                    agility: self.get_attributes(AttributesRanges::HORRIBLE),
                    intelligence: self.get_attributes(AttributesRanges::POOR),
                    will: self.get_attributes(AttributesRanges::MEDIUM),
                    charisma: self.get_attributes(AttributesRanges::POOR),
                    intimidation: self.get_attributes(AttributesRanges::MEDIUM),
                    wealth: self.get_attributes(AttributesRanges::POOR),
                    resistence: self.get_attributes(AttributesRanges::MEDIUM)
                }
            },
            Profile::Noble => {
                Attributes {
                    strength: self.get_attributes(AttributesRanges::HORRIBLE),
                    agility: self.get_attributes(AttributesRanges::HIGH),
                    intelligence: self.get_attributes(AttributesRanges::MEDIUM),
                    will: self.get_attributes(AttributesRanges::POOR),
                    charisma: self.get_attributes(AttributesRanges::MEDIUM),
                    intimidation: self.get_attributes(AttributesRanges::POOR),
                    wealth: self.get_attributes(AttributesRanges::MEDIUM),
                    resistence: self.get_attributes(AttributesRanges::POOR)
                }
            },
            Profile::Rogue => {
                Attributes {
                    strength: self.get_attributes(AttributesRanges::HORRIBLE),
                    agility: self.get_attributes(AttributesRanges::HIGH),
                    intelligence: self.get_attributes(AttributesRanges::POOR),
                    will: self.get_attributes(AttributesRanges::MEDIUM),
                    charisma: self.get_attributes(AttributesRanges::POOR),
                    intimidation: self.get_attributes(AttributesRanges::MEDIUM),
                    wealth: self.get_attributes(AttributesRanges::POOR),
                    resistence: self.get_attributes(AttributesRanges::MEDIUM)
                }
            },
            Profile::Mage => {
                Attributes {
                    strength: self.get_attributes(AttributesRanges::POOR),
                    agility: self.get_attributes(AttributesRanges::MEDIUM),
                    intelligence: self.get_attributes(AttributesRanges::HIGH),
                    will: self.get_attributes(AttributesRanges::HORRIBLE),
                    charisma: self.get_attributes(AttributesRanges::POOR),
                    intimidation: self.get_attributes(AttributesRanges::MEDIUM),
                    wealth: self.get_attributes(AttributesRanges::MEDIUM),
                    resistence: self.get_attributes(AttributesRanges::POOR)
                }
            },
            Profile::Warlock => {
                Attributes {
                    strength: self.get_attributes(AttributesRanges::MEDIUM),
                    agility: self.get_attributes(AttributesRanges::POOR),
                    intelligence: self.get_attributes(AttributesRanges::HIGH),
                    will: self.get_attributes(AttributesRanges::HORRIBLE),
                    charisma: self.get_attributes(AttributesRanges::MEDIUM),
                    intimidation: self.get_attributes(AttributesRanges::POOR),
                    wealth: self.get_attributes(AttributesRanges::POOR),
                    resistence: self.get_attributes(AttributesRanges::MEDIUM)
                }
            },
            Profile::Cleric => {
                Attributes {
                    strength: self.get_attributes(AttributesRanges::MEDIUM),
                    agility: self.get_attributes(AttributesRanges::POOR),
                    intelligence: self.get_attributes(AttributesRanges::HORRIBLE),
                    will: self.get_attributes(AttributesRanges::HIGH),
                    charisma: self.get_attributes(AttributesRanges::MEDIUM),
                    intimidation: self.get_attributes(AttributesRanges::POOR),
                    wealth: self.get_attributes(AttributesRanges::MEDIUM),
                    resistence: self.get_attributes(AttributesRanges::POOR)
                }
            },
            Profile::WitchDoctor => {
                Attributes {
                    strength: self.get_attributes(AttributesRanges::POOR),
                    agility: self.get_attributes(AttributesRanges::MEDIUM),
                    intelligence: self.get_attributes(AttributesRanges::HIGH),
                    will: self.get_attributes(AttributesRanges::HORRIBLE),
                    charisma: self.get_attributes(AttributesRanges::POOR),
                    intimidation: self.get_attributes(AttributesRanges::MEDIUM),
                    wealth: self.get_attributes(AttributesRanges::MEDIUM),
                    resistence: self.get_attributes(AttributesRanges::POOR)
                }
            },
            Profile::Bard => {
                Attributes {
                    strength: self.get_attributes(AttributesRanges::POOR),
                    agility: self.get_attributes(AttributesRanges::MEDIUM),
                    intelligence: self.get_attributes(AttributesRanges::MEDIUM),
                    will: self.get_attributes(AttributesRanges::POOR),
                    charisma: self.get_attributes(AttributesRanges::HIGH),
                    intimidation: self.get_attributes(AttributesRanges::HORRIBLE),
                    wealth: self.get_attributes(AttributesRanges::POOR),
                    resistence: self.get_attributes(AttributesRanges::MEDIUM)
                }
            },
            Profile::Templar => {
                Attributes {
                    strength: self.get_attributes(AttributesRanges::MEDIUM),
                    agility: self.get_attributes(AttributesRanges::POOR),
                    intelligence: self.get_attributes(AttributesRanges::POOR),
                    will: self.get_attributes(AttributesRanges::MEDIUM),
                    charisma: self.get_attributes(AttributesRanges::HIGH),
                    intimidation: self.get_attributes(AttributesRanges::HORRIBLE),
                    wealth: self.get_attributes(AttributesRanges::POOR),
                    resistence: self.get_attributes(AttributesRanges::MEDIUM)
                }
            },
            Profile::Assassin => {
                Attributes {
                    strength: self.get_attributes(AttributesRanges::POOR),
                    agility: self.get_attributes(AttributesRanges::MEDIUM),
                    intelligence: self.get_attributes(AttributesRanges::MEDIUM),
                    will: self.get_attributes(AttributesRanges::POOR),
                    charisma: self.get_attributes(AttributesRanges::HORRIBLE),
                    intimidation: self.get_attributes(AttributesRanges::HIGH),
                    wealth: self.get_attributes(AttributesRanges::POOR),
                    resistence: self.get_attributes(AttributesRanges::MEDIUM)
                }
            },
            Profile::Executioner => {
                Attributes {
                    strength: self.get_attributes(AttributesRanges::MEDIUM),
                    agility: self.get_attributes(AttributesRanges::POOR),
                    intelligence: self.get_attributes(AttributesRanges::POOR),
                    will: self.get_attributes(AttributesRanges::MEDIUM),
                    charisma: self.get_attributes(AttributesRanges::HORRIBLE),
                    intimidation: self.get_attributes(AttributesRanges::HIGH),
                    wealth: self.get_attributes(AttributesRanges::MEDIUM),
                    resistence: self.get_attributes(AttributesRanges::POOR)
                }
            },
            Profile::Hunter => {
                Attributes {
                    strength: self.get_attributes(AttributesRanges::MEDIUM),
                    agility: self.get_attributes(AttributesRanges::POOR),
                    intelligence: self.get_attributes(AttributesRanges::POOR),
                    will: self.get_attributes(AttributesRanges::MEDIUM),
                    charisma: self.get_attributes(AttributesRanges::MEDIUM),
                    intimidation: self.get_attributes(AttributesRanges::POOR),
                    wealth: self.get_attributes(AttributesRanges::HIGH),
                    resistence: self.get_attributes(AttributesRanges::HORRIBLE)
                }
            },
            Profile::Druid => {
                Attributes {
                    strength: self.get_attributes(AttributesRanges::POOR),
                    agility: self.get_attributes(AttributesRanges::MEDIUM),
                    intelligence: self.get_attributes(AttributesRanges::MEDIUM),
                    will: self.get_attributes(AttributesRanges::POOR),
                    charisma: self.get_attributes(AttributesRanges::POOR),
                    intimidation: self.get_attributes(AttributesRanges::MEDIUM),
                    wealth: self.get_attributes(AttributesRanges::HIGH),
                    resistence: self.get_attributes(AttributesRanges::HORRIBLE)
                }
            },
            Profile::Barbarian => {
                Attributes {
                    strength: self.get_attributes(AttributesRanges::MEDIUM),
                    agility: self.get_attributes(AttributesRanges::POOR),
                    intelligence: self.get_attributes(AttributesRanges::POOR),
                    will: self.get_attributes(AttributesRanges::MEDIUM),
                    charisma: self.get_attributes(AttributesRanges::POOR),
                    intimidation: self.get_attributes(AttributesRanges::MEDIUM),
                    wealth: self.get_attributes(AttributesRanges::HORRIBLE),
                    resistence: self.get_attributes(AttributesRanges::HIGH)
                }
            },
            Profile::Shaman => {
                Attributes {
                    strength: self.get_attributes(AttributesRanges::POOR),
                    agility: self.get_attributes(AttributesRanges::MEDIUM),
                    intelligence: self.get_attributes(AttributesRanges::MEDIUM),
                    will: self.get_attributes(AttributesRanges::POOR),
                    charisma: self.get_attributes(AttributesRanges::MEDIUM),
                    intimidation: self.get_attributes(AttributesRanges::POOR),
                    wealth: self.get_attributes(AttributesRanges::HORRIBLE),
                    resistence: self.get_attributes(AttributesRanges::HIGH)
                }
            }
            _ => {
                panic!("Unknow role. Panic!")
            }
        }
    }

    fn onboarding(&mut self) -> Player {
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

        let mut attributes_options: Vec<String> = vec!();
        let mut profile_suggestions: Vec<String> = vec!();

        // TODO: these attributes can be an option with description instead of string match
        match role {
            Role::Fighter => {
                attributes_options.push(String::from("STRENGTH"));
                attributes_options.push(String::from("AGILITY"));
            },
            Role::Mage => {
                attributes_options.push(String::from("INTELLIGENCE"));
                attributes_options.push(String::from("WILLPOWER"));
            },
            Role::Hypno => {
                attributes_options.push(String::from("CHARISMA"));
                attributes_options.push(String::from("INTIMIDATION"));
            },
            Role::Survivor => {
                attributes_options.push(String::from("CONSTITUTION"));
                attributes_options.push(String::from("RESISTENCE"));
            },
        }

        let profiles: String = interaction::capture_input(
            "Okay, now I want to know what will be your primary attribute, then I will ask you some profiles based on it",
            "Are you sure?",
            "",
            attributes_options,
        );

        match profiles.to_uppercase().as_str() {
            "STRENGTH" => {
                profile_suggestions.push(Profile::Knight.to_string());
                profile_suggestions.push(Profile::Warrior.to_string());
            },
            "AGILITY" => {
                profile_suggestions.push(Profile::Noble.to_string());
                profile_suggestions.push(Profile::Rogue.to_string());
            },
            "INTELLIGENCE" => {
                profile_suggestions.push(Profile::Mage.to_string());
                profile_suggestions.push(Profile::Warlock.to_string());
            },
            "WILLPOWER" => {
                profile_suggestions.push(Profile::Cleric.to_string());
                profile_suggestions.push(Profile::WitchDoctor.to_string());
            },
            "CHARISMA" => {
                profile_suggestions.push(Profile::Bard.to_string());
                profile_suggestions.push(Profile::Templar.to_string());
            },
            "INTIMIDATION" => {
                profile_suggestions.push(Profile::Assassin.to_string());
                profile_suggestions.push(Profile::Berserker.to_string());
            },
            "CONSTITUTION" => {
                profile_suggestions.push(Profile::Hunter.to_string());
                profile_suggestions.push(Profile::Druid.to_string());
            },
            "RESISTENCE" => {
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
    }
}

fn main() {
    let mut app = SomeDreamApplication::initialize(rand::thread_rng());
    app.main_loop();
}