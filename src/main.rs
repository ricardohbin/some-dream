use rand::rngs::ThreadRng;

use std::env;


pub mod render;
pub mod interaction;
pub mod player;
pub mod attributes;
pub mod monster;
pub mod arena;
pub mod encounter;

pub mod color;
mod onboarding;
mod map;


struct SomeDreamApplication {
    rng: ThreadRng,
    is_debug_mode: bool,
}

impl SomeDreamApplication {
    pub fn initialize(rng: ThreadRng, is_debug_mode: bool) -> Self {
        Self {
            rng,
            is_debug_mode,
        }
    }

    fn main_loop(&mut self) {
        let mut _player: player::Player;

        render::render_splash_screen();

        let player = if self.is_debug_mode { player::Player{
                name: String::from("Bin"),
                role: player::Role::Fighter,
                profile: player::Profile::Knight,
                stats: attributes::Stats{
                    strength: 3,
                    agility: 3,
                    intelligence: 3,
                    will: 3,
                    charisma: 3,
                    intimidation: 3,
                    wealth: 3,
                    resistence: 3,
                },
                vital_points: attributes::VitalPoints{
                    life: 6,
                    luck: 6,
                    cardio: 6,
                    social: 6,
                }
            }
        } else {
            let mut prompt = onboarding::Onboarding::init(
                self.rng
            );
            prompt.start()
        };

        // TODO: dynamic path based in role. And the art itself... :P
        render::render_image_to_ansi("./src/art/fighter.gif");

        println!("It's you! Nice shape ahn? Let's begin finally....\n");

        // The player is borrowed to mapcore until the end of program. Let's keep a copy here in this scope
        let player_name = player.name.clone();

        let mut map_core = map::MapCore::new(self.rng, player, self.is_debug_mode);

        map_core.generate_world();

        // Start points - get later from Map with index 0
        // TODO: FIX THIS
        let mut x: usize = 1;
        let mut y: usize = 1;
        let mut index: usize = 0;

        loop {
            let map_options: map::MapOptions = map_core.point(index, x, y);
            let description = map_options.description;
            let mut minimap = map_options.minimap;

            // TODO: Adding custom color in minimap - move this from here later in the map class
            // TODO2: Suport tags like to description coloring <color>Xis</color>?
            minimap = color::paint(Box::new(color::Gray{}), "#", minimap.as_str());
            minimap = color::paint(Box::new(color::Green{}), ".", minimap.as_str());
            minimap = color::paint(Box::new(color::Yellow{}), "?", minimap.as_str());
            minimap = color::paint(Box::new(color::Blue{}), "X", minimap.as_str());

            println!("{}\n\n{}", minimap, description);

            //resync in possible repaint of map
            x = map_options.x;
            y = map_options.y;
            index = map_options.index;

            println!("{:?}", map_options.is_player_alive);

            if !map_options.is_player_alive {
                println!("Your journey ends here. Sorry {}", player_name);
                return;
            }

            let direction: String = interaction::capture_input("What path you go?", "", "You choose", map_options.directions);

            match direction.as_str() {
                "n" => y -= 1,
                "s" => y += 1,
                "e" => x -= 1,
                "w" => x += 1,
                _ => panic!("Invalid direction... this can't happen"),
            }
        }
    }
}

fn main() {
    let mut app = SomeDreamApplication::initialize(
        rand::thread_rng(),
        env::var("DEBUG").is_ok() && env::var("DEBUG").unwrap() == "1"
    );
    app.main_loop();
}