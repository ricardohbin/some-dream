use text_io::read;
use std::collections::HashMap;

const DEFAULT_OPTIONS: &'static str = "[y/n]";

fn give_feedback(phrase: &str, value: String) -> String {
    println!("{} - \"{}\"", phrase, value);
    value
}

pub fn capture_input(
    before_input_phrase: &str,
    after_input_phrase: &str,
    success_input_phrase: &str,
    options: std::vec::Vec<String>
) -> String {
    let ask_confirmation: bool = after_input_phrase != "";

    let has_options: bool = !options.is_empty();

    println!("{}", before_input_phrase);

    if has_options {
        let nice_options = options.join(" | ");
        println!("Options: {}", nice_options)
    }

    let value: String = read!();

    if has_options && !options.contains(&value) {
        println!("Your option {} doesn't exists in the list {:?}. try again.", value, options);
        return capture_input(
            before_input_phrase,
            after_input_phrase,
            success_input_phrase,
            options
        )
    }

    if after_input_phrase != "" {
        let mut after_input_phrase_with_confirmation: String = after_input_phrase.to_owned();
        if ask_confirmation {
            after_input_phrase_with_confirmation = format!("{} - {}", after_input_phrase_with_confirmation, DEFAULT_OPTIONS)
        }
        println!("{}", after_input_phrase_with_confirmation);
    }

    if ask_confirmation {
        if ask() {
            return give_feedback(success_input_phrase, value);
        } else {
            println!("Ok! Let's try again,");
            return capture_input(
                before_input_phrase,
                after_input_phrase,
                success_input_phrase,
                options
            )
        }
    }

    give_feedback(success_input_phrase, value)
}

pub fn ask() -> bool {
    let option: String = read!();
    if option == "y" {
        return true;
    } else if option == "n" {
        return false;
    }
    println!("Wrong option. Try again. The options are {}", DEFAULT_OPTIONS);
    ask()
}

pub fn pick_an_option(
    before_input_phrase: &str,
    after_input_phrase: &str,
    success_input_phrase: &str,
    options_map: HashMap<i8, String>
) -> i8 {
    let ask_confirmation: bool = after_input_phrase != "";

    println!("{}", before_input_phrase);

    println!("Choose one option");

    // TODO: hash order
    for (option, description) in &options_map {
        println!("{} - {}", option, description);
    }

    let value: i8 = read!();

    let option = options_map.get(&value);

    let option_ok: String;

    match option {
        None => {
            println!("Your option {} doesn't exists in the list {:?}. try again.", value, options_map);
            return pick_an_option(
                before_input_phrase,
                after_input_phrase,
                success_input_phrase,
                options_map
            )
        },
        Some(x) => {
            option_ok = String::from(x);
        }
    }

    if after_input_phrase != "" {
        let mut after_input_phrase_with_confirmation: String = after_input_phrase.to_owned();
        if ask_confirmation {
            after_input_phrase_with_confirmation = format!("{} - {}", after_input_phrase_with_confirmation, DEFAULT_OPTIONS)
        }
        println!("{}", after_input_phrase_with_confirmation);
    }

    if ask_confirmation {
        if ask() {
            give_feedback(success_input_phrase, option_ok);
            return value;
        } else {
            println!("Ok! Let's try again,");
            return pick_an_option(
                before_input_phrase,
                after_input_phrase,
                success_input_phrase,
                options_map
            )
        }
    }

    value
}