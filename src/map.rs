use std::collections::HashMap;
use rand::Rng;
use rand::rngs::ThreadRng;
use super::player::*;
use super::monster::*;
use super::arena;
use super::color;

const PLAYER: &str = "P";

#[derive(Debug)]
pub struct MapOptions {
    pub minimap: String,
    pub description: String,
    pub directions: Vec<String>,
    pub x: usize,
    pub y: usize,
    pub index: usize,
    pub is_player_alive: bool,
}

#[derive(Debug, Clone)]
pub struct Map {
    enterpoints: HashMap<usize, (usize, usize)>,
    exits: HashMap<usize, (usize, usize)>,
    description: &'static str,
    minimap: String,
    events: HashMap<String, Vec<Event>>,
}

#[derive(Debug, Clone)]
pub struct Event {
    // TODO: remove description - it is redundant
    description: String,
    monster: Option<Monster>,
}

#[derive(Debug, Clone)]
pub struct MapCore {
    rng: ThreadRng,
    event_point: HashMap<(usize, usize, usize), Event>,
    pub player: Player,
    is_debug: bool,
    world: HashMap<usize, Map>,
}

impl MapCore {
    fn clean_white_spaces(&self, value: &str) -> String {
        value.to_string().replace(" ", "")
    }

    pub fn new(rng: ThreadRng, player: Player, is_debug: bool) -> Self {
        Self {
            rng,
            event_point: HashMap::new(),
            player,
            is_debug,
            world: HashMap::new(),
        }
    }

    fn generate_map_seed(&mut self, index: &str, interactions: usize, range_x: (usize, usize), range_y: (usize, usize), possible_events: Vec<Event>) -> Map {
        let width = self.rng.gen_range(range_x.0, range_x.1);
        let height = self.rng.gen_range(range_y.0, range_y.1);
        let mut map = String::from("");

        let mut total_points = (width - 2) * (height - 2);
        let mut possible_points = interactions;
        let uindex: usize = index.parse().unwrap();

        let mut ent: HashMap<usize, (usize, usize)>= HashMap::new();
        let mut exits: HashMap<usize, (usize, usize)>= HashMap::new();

        for _y in 0..height {
            for _x in 0..width {

                if _x == 0 && _y == 1 {
                    if uindex > 0 {
                        map.push_str((uindex - 1).to_string().as_str());
                    } else {
                        map.push_str("#");
                    }
                    // + one more X to right in scenario ->>>
                    ent.insert(uindex, (_x + 1, _y));
                    continue;
                }

                if _x == width - 1 && _y == height - 2 {
                    map.push_str((uindex + 1).to_string().as_str());
                    // - one more X to right in scenario <<<-
                    exits.insert(uindex + 1, (_x - 1, _y));
                    continue;
                }

                if _y == 0 || _y == height - 1 {
                    map.push_str("#");
                    continue;
                }
                if _x == 0 || _x == width - 1 {
                    map.push_str("#");
                    continue;
                }

                if total_points == 0 || (self.rng.gen_range(0, total_points) == 0 && possible_points > 0) {
                    map.push_str("?");
                    possible_points -= 1;
                } else {
                    total_points -= 1;
                    map.push_str(".");
                }
            }
            map.push_str("\n");
        }


        let mut events: HashMap<String, Vec<Event>> = HashMap::new();
        events.insert("?".to_string(), possible_events);

        let map = Map {
            description: "a",
            minimap: map,
            enterpoints: ent,
            events,
            exits,
        };

        if self.is_debug {
            println!("{:?}", map);
        }
        
        map
    }

    pub fn generate_world(&mut self) {
        let mut m: HashMap<usize, Map> = HashMap::new();
        m.insert(0, self.generate_map_seed("0", 1, (8, 10), (5, 6), vec!(
            Event{
                description: color::paint_text(Box::new(color::Yellow{}), "There's a coin here"),
                monster: Some(
                    Monster::new(Box::new(Goblin{}),
                    1
                )),
            },
            Event{
                description: color::paint_text(Box::new(color::Red{}), "There's a flower here"),
                monster: Some(
                    Monster::new(Box::new(Skeleton{}),
                    1
                )),
            },
            Event{
                description: color::paint_text(Box::new(color::Gray{}), "There's a sword here"),
                monster: Some(
                    Monster::new(Box::new(Ogre{}),
                    1
                )),
            }
        )));
        m.insert(1, self.generate_map_seed("1", 0, (30, 40), (5, 6), vec!()));
        m.insert(2, self.generate_map_seed("2", 0, (50, 80), (5, 6), vec!()));

        self.world = m;
    }

    fn prepare_arena(&mut self, m: &mut Monster) -> bool {
        let arena = arena::Arena::new(self.rng, self.is_debug);
        arena.prepare(&mut self.player, m)
    }

    pub fn point(&mut self, index: usize, x: usize, y: usize) -> MapOptions {
        // Ignoring first `"` in split
        // TODO: better split to this, to use x as is
        let column = x + 1;
        let mut is_player_alive = true;

        let map_string;
        let map;
        let world = self.world.clone();

        let current_world = world.get(&index);

        match current_world {
            Some(m) => {
                map = m;
                // removing extra spaces
                map_string = self.clean_white_spaces(&m.minimap.to_string());
            },
            None => panic!("No map match with the key {} - ", index),
        }

        // note: char '\n' instead of "\n" is proposital
        let mut lines: Vec<&str> = map_string.split('\n').collect();

        let mut columns: Vec<&str> = lines[y].split("").collect();

        let position = &lines[y][x..column];

        
        //check if there's an room change. Numbers means rooms, otherwise let's try match string
        if position != "#" && position != "." {
            let result_change_map: Result<usize, std::num::ParseIntError> = position.parse::<usize>();
            if let Ok(map_index) = result_change_map {
                let next_map;
                match world.get(&map_index) {
                    Some(m) => {
                        next_map = m;
                    },
                    None => panic!("No map match with the key {} - ", index),
                }
                if index < map_index {
                    println!("{}{}???", index, map_index);
                    match next_map.enterpoints.get(&map_index) {
                        Some(map_points) => {
                            return self.point(map_index, map_points.0, map_points.1);
                        },
                        None => panic!("No enterpoint {} found at map #{}", map_index, index),
                    }
                } else {
                    println!("{}{}!!!", index, map_index);
                    match next_map.exits.get(&index) {
                        Some(map_points) => {
                            return self.point(map_index, map_points.0, map_points.1);
                        },
                        None => panic!("No exit {} found at map #{}", map_index, index),
                    }
                }
            } else {
                let actual_event_point = self.event_point.get(&(index, x, y));
                match actual_event_point {
                    Some(event) => {
                        if self.is_debug {
                            println!("REPLAY!!!");
                            println!("{:?}", event);
                        }

                        println!("{}", event.description);

                        if let Some(mut m) = event.monster.clone() {
                            if m.vital_points.life <= 0 {
                                println!("You see a dead {}", m.description);
                            } else {
                                is_player_alive = self.prepare_arena(&mut m);
                            }
                        }

                    },
                    // Let's get a random one :D
                    None => {
                        let interactions = map.events.get(position);
                        match interactions {
                            Some(i) => {
                                let mut interaction_random_range: usize;
                                interaction_random_range = self.rng.gen_range(0, i.len());

                                if self.is_debug {
                                    println!("FIRST TIME!");
                                    // TO DEBUG purposes in this early stage 
                                    // interaction_random_range = 2;
                                }

                                let interaction_temp = i[interaction_random_range].clone();
                                self.event_point.insert((index, x, y), interaction_temp);
                                let event = &i[interaction_random_range];
                                let monster = event.monster.clone();

                                println!("{}", event.description);

                                if let Some(mut m) = monster {
                                    is_player_alive = self.prepare_arena(&mut m);
                                    
                                    // Override previous state when exists fight
                                    // for now only fights to DEATH will be allowed
                                    // TODO: escape using skills, etc

                                    if is_player_alive {
                                        self.event_point.insert((index, x, y), Event{
                                            description: event.description.clone(),
                                            monster: Option::from(m)
                                        });
                                    }
                                }                                
                            },
                            None => panic!("Unknow caracter {}", position),
                        }
                    }
                }
            }
        }
        
        columns[column] = PLAYER;

        // creating a binding to avoid temporary value dropped while borrowed
        let new_line = columns.join("");
        lines[y] = new_line.as_str();
        let new_map = lines.join("\n");

        let mut directions: Vec<String> = vec!();

        // checking directions and points of interest
        // TODO: put this in a function
        if columns[column - 1] != "#" {
            directions.push("e".to_string());
        }

        if columns[column + 1] != "#" {
            directions.push("w".to_string());
        }

        if &lines[y + 1][x..column] != "#" {
            directions.push("s".to_string());
        }

        if &lines[y - 1][x..column] != "#" {
            directions.push("n".to_string());
        }

        MapOptions{
            minimap: new_map,
            description: map.description.trim().to_string(),
            directions,
            x,
            y,
            index,
            is_player_alive,
        }
    }
}
