use super::Player;

const GRID_HORIZONTAL: usize = 80;

fn render_block(text: String, pattern: String) -> String {
    let full_size: usize = GRID_HORIZONTAL;
    let left_padding: usize = full_size - text.len() / 2;
    let right_padding: usize = left_padding - text.len() % 2;
    [
        pattern.repeat(left_padding), String::from(" "), text, String::from(" "), pattern.repeat(right_padding)
    ].concat()
}

pub fn render_attributes(player: &Player) {
    println!("{}", render_block(String::from("Attributes"), String::from("=")));

    println!("{}", render_block(format!(
    "Strenght: {} | Agility: {} | Intelligence: {} | Willpower: {}",
    player.attributes.strength, player.attributes.agility, player.attributes.intelligence, player.attributes.will
    ), String::from(" ")));

    println!("{}", render_block(format!(
    "Charisma: {} | Intimidation: {} | Wealth: {} | Resistence: {}",
    player.attributes.charisma, player.attributes.intimidation, player.attributes.wealth, player.attributes.resistence
    ), String::from(" ")));

    println!("{}", render_block(String::from("Vital Points"), String::from("=")));

    println!("{}", render_block(format!(
    "Life: {} | Luck: {} | Cardio: {} | Social: {}",
    player.vital_points.life, player.vital_points.luck, player.vital_points.cardio, player.vital_points.social
    ), String::from(" ")));
}