use super::player::Player;
use super::attributes::VitalPoints;

use std::fs;
use image::imageops;
use image::imageops::FilterType;
use std::path::Path;
use ansi_term::Colour::RGB;

const GRID_HORIZONTAL: usize = 80;
const PIXEL: &str = "██";


fn render_block(text: String, pattern: String) -> String {
    let full_size: usize = GRID_HORIZONTAL;
    let left_padding: usize = full_size - text.len() / 2;
    let right_padding: usize = left_padding - text.len() % 2;
    [
        pattern.repeat(left_padding),
        String::from(" "),
        text,
        String::from(" "),
        pattern.repeat(right_padding),
        String::from("\n")
    ].concat()
}

pub fn render_splash_screen() {
    let content = fs::read_to_string("./src/art/splash_screen.txt");
    println!("{}", content.unwrap());
}

pub fn render_attributes(player: &Player) {
    let mut output: Vec<String> = vec!();
    output.push(render_block(String::from("Attributes"), String::from("=")));

    output.push(render_block(format!(
    "Strenght: {} | Agility: {} | Intelligence: {} | Willpower: {}",
    player.stats.strength, player.stats.agility, player.stats.intelligence, player.stats.will
    ), String::from(" ")));

    output.push(render_block(format!(
    "Charisma: {} | Intimidation: {} | Wealth: {} | Resistence: {}",
    player.stats.charisma, player.stats.intimidation, player.stats.wealth, player.stats.resistence
    ), String::from(" ")));

    output.push(render_block(String::from("Vital Points"), String::from("=")));

    output.push(render_block(format!(
    "Life: {} | Luck: {} | Cardio: {} | Social: {}",
    player.vital_points.life, player.vital_points.luck, player.vital_points.cardio, player.vital_points.social
    ), String::from(" ")));

    output.push(render_block(String::from("Equipment"), String::from("=")));

    output.push(render_block(format!(
        "Weapon: {}", player.weapon
    ), String::from(" ")));

    print!("{}", output.join("\n"));
}

pub fn render_mini_stats(vital_points: VitalPoints) {
    let mut output: Vec<String> = vec!();
    output.push(format!(
    "<Life: {} | Luck: {} | Cardio: {} | Social: {}> ",
    vital_points.life, vital_points.luck, vital_points.cardio, vital_points.social
    ));
    print!("{}", output.join(""))
}

pub fn render_image_to_ansi(file_path: &str) {
    let path: &Path = Path::new(file_path);
    let img = image::open(path).unwrap();
    let img_to_render = imageops::resize(&img, 16, 16, FilterType::Nearest);
    let width = img_to_render.width();
    let height = img_to_render.height();
    let mut output: Vec<String> = vec!();
    
    for y in 0..height {
        for x in 0..width {
            let top = img_to_render.get_pixel(x,y);
            // See https://en.wikipedia.org/wiki/Block_Elements
            output.push(format!("{}", RGB(top[0], top[1], top[2]).normal().paint(PIXEL)));
        }
        output.push("\n".to_string());
    }

    print!("{}", output.join(""));
}