use super::Player;

use std::fs;
use image::imageops;
use image::imageops::FilterType;
use std::path::Path;
use ansi_term::Colour::RGB;

const GRID_HORIZONTAL: usize = 80;

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
    ].concat()
}

pub fn render_splash_screen() {
    let content = fs::read_to_string("./src/art/splash_screen.txt");
    println!("{}", content.unwrap());
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

pub fn render_image_to_ansi(file_path: &str) {
    let path: &Path = Path::new(file_path);
        let img = image::open(path).unwrap();
        let img_to_render = imageops::resize(&img, 15, 15, FilterType::Nearest);
        let width = img_to_render.width();
        let height = img_to_render.height();
        
        for y in 0..height {
            for x in 0..width {
                let top = img_to_render.get_pixel(x,y);
                print!("{}", RGB(top[0], top[1], top[2]).normal().paint("▀▀"));
            }
            println!();
        }
}