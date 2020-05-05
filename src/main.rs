use text_io::read;
use rand::Rng;
use rand::rngs::ThreadRng;


#[derive(Debug)]
struct Player {
    name: String,
    profession: Profession,
    attributes: Attributes
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
    resistence: i8
}

#[derive(Debug)]
enum Profession {
    FIGHTER,
    MAGE,
    SURVIVOR,
    HYPNO,
    UNKOWN
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
            rng: rng
        }
    }

    fn get_attributes(&mut self, ranges: AttributesRanges) -> i8 {
        match ranges {
            AttributesRanges::HIGH => self.rng.gen_range(6, 9),
            AttributesRanges::MEDIUM => self.rng.gen_range(4, 7),
            AttributesRanges::POOR => self.rng.gen_range(2, 5),
            AttributesRanges::HORRIBLE => self.rng.gen_range(0, 3)
        }
    }

    fn roll_status(&mut self, value: String) -> Attributes {
        match value.to_uppercase().as_str() {
            "KNIGHT" => {
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
            "WARRIOR" => {
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
            "NOBLE" => {
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
            "ROGUE" => {
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
            "MAGE" => {
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
            "CLERIC" => {
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
            "WITCH_DOCTOR" => {
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
            "BARD" => {
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
            "TEMPLAR" => {
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
            "ASSASSIN" => {
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
            "EXECUTIONER" => {
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
            "HUNTER" => {
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
            "DRUID" => {
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
            "BARBARIAN" => {
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
            "SHAMAN" => {
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

    fn retrieve_profession(&mut self, value: String) -> Profession {
        match value.to_uppercase().as_str() {
            "FIGHTER" => Profession::FIGHTER,
            "MAGE" => Profession::MAGE,
            "SURVIVOR" => Profession::SURVIVOR,
            "HYPNO" => Profession::HYPNO,
            _ => Profession::UNKOWN
        }
    }

    fn capture_input(
        &mut self,
        before_input_phrase: &str,
        after_input_phrase: &str,
        success_input_phrase: &str,
        options: std::vec::Vec<String>
    ) -> String {
        fn give_feedback(phrase: &str, value: String) -> String {
            println!("{}{}", phrase, value);
            return value;
        }

        let ask_confirmation: bool = after_input_phrase != "";

        let has_options: bool = options.len() > 0;

        println!("{}", before_input_phrase);

        if has_options {
            let nice_options = options.join(" | ");
            println!("Options: {}", nice_options)
        }

        let value: String = read!();

        if has_options && !options.contains(&value.to_uppercase()) {
            println!("Your option {} doesn't exists in the list {:?}. try again.", value, options);
            return self.capture_input(
                before_input_phrase,
                after_input_phrase,
                success_input_phrase,
                options
            )
        }

        if after_input_phrase != "" {
            let mut after_input_phrase_with_confirmation: String = after_input_phrase.to_owned();
            if ask_confirmation {
                after_input_phrase_with_confirmation = format!("{} - {}", after_input_phrase_with_confirmation, "[Y/N]")
            }
            println!("{}", after_input_phrase_with_confirmation);
        }

        if ask_confirmation {
            if self.ask() {
                return give_feedback(success_input_phrase, value);
            } else {
                println!("Ok! Let's try again,");
                return self.capture_input(
                    before_input_phrase,
                    after_input_phrase,
                    success_input_phrase,
                    options
                )
            }
        }

        return give_feedback(success_input_phrase, value);
    }

    fn ask(&mut self) -> bool {
        let option: String = read!();
        if option == "Y" {
            return true;
        } else if option == "N" {
            return false;
        }
        println!("Wrong option. Try again. The options are [Y/N]");
        return self.ask();
    }

    fn main_loop(&mut self) {
        let name: String = self.capture_input(
            "Hello! Welcome to awesome world of some dream. Tell me your name!",
            "Very well. Your name is correct?",
            "Nice to meet you!",
            vec!(),
        );

        let professions: Vec<String> = vec!(
            String::from("FIGHTER"),
            String::from("MAGE"),
            String::from("SURVIVOR"),
            String::from("HYPNO"),
        );

        let profession_opt: String = self.capture_input(
            "Okay, now I want to know what will be your job here",
            "You want to proceed with this info?",
            "",
            professions,
        );

        let profession: Profession = self.retrieve_profession(profession_opt);

        match profession {
            Profession::UNKOWN => {
                panic!("Unknow profession. Panic!")
            },
            _ => {}
        }

        {
            let mut attributes_options: Vec<String> = vec!();
            let mut profile_suggestions: Vec<String> = vec!();

            match profession {
                Profession::FIGHTER => {
                    attributes_options.push(String::from("STRENGTH"));
                    attributes_options.push(String::from("AGILITY"));
                },
                Profession::MAGE => {
                    attributes_options.push(String::from("INTELLIGENCE"));
                    attributes_options.push(String::from("WILLPOWER"));
                },
                Profession::HYPNO => {
                    attributes_options.push(String::from("CHARISMA"));
                    attributes_options.push(String::from("INTIMIDATION"));
                },
                Profession::SURVIVOR => {
                    attributes_options.push(String::from("CONSTITUTION"));
                    attributes_options.push(String::from("RESISTENCE"));
                },
                _ => {}
            }

            let profiles: String = self.capture_input(
                "Okay, now I want to know what will be your primary attribute, then I will ask you some profiles based on it",
                "Are you sure?",
                "",
                attributes_options,
            );

            match profiles.to_uppercase().as_str() {
                "STRENGTH" => {
                    profile_suggestions.push(String::from("KNIGHT"));
                    profile_suggestions.push(String::from("WARRIOR"));
                },
                "AGILITY" => {
                    profile_suggestions.push(String::from("NOBLE"));
                    profile_suggestions.push(String::from("ROGUE"));
                },
                "INTELLIGENCE" => {
                    profile_suggestions.push(String::from("MAGE"));
                    profile_suggestions.push(String::from("WARLOCK"));
                },
                "WILLPOWER" => {
                    profile_suggestions.push(String::from("CLERIC"));
                    profile_suggestions.push(String::from("WITCH_DOCTOR"));
                },
                "CHARISMA" => {
                    profile_suggestions.push(String::from("BARD"));
                    profile_suggestions.push(String::from("TEMPLAR"));
                },
                "INTIMIDATION" => {
                    profile_suggestions.push(String::from("ASSASSIN"));
                    profile_suggestions.push(String::from("BERSERKER"));
                },
                "CONSTITUTION" => {
                    profile_suggestions.push(String::from("HUNTER"));
                    profile_suggestions.push(String::from("DRUID"));
                },
                "RESISTENCE" => {
                    profile_suggestions.push(String::from("BARBARIAN"));
                    profile_suggestions.push(String::from("SHAMAN"));
                },
                _ => {
                    panic!("What attribute was missing? {}", profiles);
                }
            }

            let profile: String = self.capture_input(
                "You choose a nice profile. And now, which profile you want? This is the last step of this onboarding",
                "Are you sure?",
                "Ok, let's roll the stats",
                profile_suggestions,
            );

            let attributes: Attributes = self.roll_status(profile);

            let player = Player {
                name: name,
                profession: profession,
                attributes: attributes
            };

            println!("{:?}", player);

            self.capture_input(
                "Your status is done",
                "Let's begin the adventure?",
                "Very well",
                vec!(),
            );
        }

    }
}

fn main() {
    let mut app = SomeDreamApplication::initialize(rand::thread_rng());
    app.main_loop();
}