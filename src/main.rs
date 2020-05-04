use text_io::read;

struct Player {
    name: String,
    profession: Profession,
    attributes: Attributes
}

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

enum Profession {
    FIGHTER,
    MAGE,
    SURVIVOR,
    HYPNO,
    UNKOWN
}

struct SomeDreamApplication {
    
}

impl SomeDreamApplication {
    pub fn initialize() -> Self {
        Self {}
    }

    fn retrieve_profession(&mut self, value: String) -> Profession {
        return match value.to_uppercase().as_str() {
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

        println!("{}", before_input_phrase);

        if options.len() > 0 {
            let nice_options = options.join(" | ");
            println!("Options: {}", nice_options)
        }

        let value: String = read!();

        if options.len() > 0 && !options.contains(&value.to_uppercase()) {
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

        let mut attributes: Vec<i8> = vec!();

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
                    profile_suggestions.push(String::from("KNIGHT")); // ++str, int, cha, cons
                    profile_suggestions.push(String::from("BARBARIAN")); // ++str, will, inti, res
                },
                "AGILITY" => {
                    profile_suggestions.push(String::from("NOBLE")); // ++agi, int, cha, cons
                    profile_suggestions.push(String::from("ROGUE")); // ++agi, will, inti, res
                },
                "INTELLIGENCE" => {
                    profile_suggestions.push(String::from("MAGE")); // ++int, agi, cha, cons
                    profile_suggestions.push(String::from("WARLOCK")); // ++int, str, inti, res
                },
                "WILLPOWER" => {
                    profile_suggestions.push(String::from("CLERIC"));  //++will, str, cha, cons
                    profile_suggestions.push(String::from("WITCH_DOCTOR")); //++will, agi, inti, res
                },
                "CHARISMA" => {
                    profile_suggestions.push(String::from("BARD")); //++cha, agi, res, int
                    profile_suggestions.push(String::from("TEMPLAR")); //++cha, str, cons, will
                },
                "INTIMIDATION" => {
                    profile_suggestions.push(String::from("ASSASSIN")); //++inti, agi, res, int
                    profile_suggestions.push(String::from("BERSERKER")); //++inti, str, cons, will
                },
                "CONSTITUTION" => {
                    profile_suggestions.push(String::from("TANK")); //++cons, str, cha, int
                    profile_suggestions.push(String::from("EXECUTIONER")); //++cons, agi, inti, will
                },
                "RESISTENCE" => {
                    profile_suggestions.push(String::from("BARBARIAN")); //++res, str, will, inti
                    profile_suggestions.push(String::from("RANGER")); //++res, agi, cha, int
                },
                _ => {
                    panic!("What attribute was missing? {}", profiles);
                }
            }
            

            let profile: String = self.capture_input(
                "You choose a nice profile",
                "Are you sure?",
                "Ok, let's roll the stats",
                profile_suggestions,
            );
        }

    }
}

fn main() {
    let mut app = SomeDreamApplication::initialize();
    app.main_loop();
}