use std::collections::HashMap;
use rand::Rng;
use rand::rngs::ThreadRng;
use super::player::*;
use super::monster::*;
use super::arena;
use super::itens::*;

const PLAYER: &str = "\u{001b}[38;5;51m@\u{001b}[0m";

#[derive(Debug)]
pub struct MapOptions {
    pub minimap: String,
    pub directions: Vec<String>,
    pub x: usize,
    pub y: usize,
    pub index: usize,
    pub is_player_alive: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Position2d {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone)]
pub struct Map {
    enterpoints: HashMap<usize, (usize, usize)>,
    exits: HashMap<usize, (usize, usize)>,
    minimap: String,
    events: HashMap<Position2d, Event>
}

#[derive(Debug, Clone)]
// TODO: CHANGE THIS
pub struct Event {
    monster: Option<Monster>,
    encounter: Option<Item>
}

#[derive(Debug, Clone)]
pub struct MapCore {
    rng: ThreadRng,
    event_point: HashMap<(usize, Position2d), Event>,
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

    fn generate_position(&mut self, possible_positions: &Position2d, used_positions: &[Position2d]) -> Position2d {
        let new_position = Position2d {
            x: self.rng.gen_range(1, possible_positions.x), y: self.rng.gen_range(1, possible_positions.y)
        };

        if used_positions.contains(&new_position) {
            return self.generate_position(possible_positions, used_positions);
        }

        new_position
    }

    fn generate_map_seed(
        &mut self, index: &str,
        range_x: (usize, usize),
        range_y: (usize, usize),
        monsters: Vec<Monster>,
        encounters: Vec<Item>,
    ) -> Map {
        let width = self.rng.gen_range(range_x.0, range_x.1);
        let height = self.rng.gen_range(range_y.0, range_y.1);
        let mut map = String::from("");

        //Length os 1 fixed encounter + 1 fixed monster slot for now
        let uindex: usize = index.parse().unwrap();
        let mut ent: HashMap<usize, (usize, usize)>= HashMap::new();
        let mut exits: HashMap<usize, (usize, usize)>= HashMap::new();
        let mut events = HashMap::new();

        let existing_positions = Position2d{ x: width - 1, y: height - 1 };
        let init_position = Position2d{ x: 1, y: 1 };
        let mut used_positions:Vec<Position2d> = vec!(init_position);
        let mut monster_positions: HashMap<Position2d, Monster> = HashMap::new();
        let mut encounters_positions: HashMap<Position2d, Item> = HashMap::new();

        //TODO: Random monsters/encounters limits but slots 
        // Checking all possible positions to monsters
        for i in monsters {
            let m_position = self.generate_position(&existing_positions, &used_positions);
            used_positions.push(m_position.clone());
            monster_positions.insert(m_position.clone(), i);
        }

        // Checking all possible positions to encounters
        for i in encounters {
            let m_position = self.generate_position(&existing_positions, &used_positions);
            used_positions.push(m_position.clone());
            encounters_positions.insert(m_position.clone(), i);
        }

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

                let actual_position = Position2d {
                    x: _x,
                    y: _y
                };

                match monster_positions.get(&actual_position) {
                    Some(m) => {
                        map.push_str("?");
                        events.insert(actual_position, Event {
                            monster: Some(m.clone()),
                            encounter: None
                        });
                    },
                    None => {
                        // Fix this nested match pattern
                        match encounters_positions.get(&actual_position) {
                            Some(e) => {
                                map.push_str("X");
                                events.insert(actual_position, Event {
                                    monster: None,
                                    encounter: Some(e.clone()),
                                });
                            },
                            None => {
                                map.push_str(".");
                            }
                        }
                    }
                }
            }
            map.push_str("\n");
        }

        let map = Map {
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
        let mut monster_factory = MonsterFactory::new(self.rng);
        let mut encounter_factory = ItemFactory::new(&mut self.rng);
        m.insert(0, self.generate_map_seed(
            "0",
            (14, 20),
            (4, 5),
            vec!(
                monster_factory.generate(0),
                monster_factory.generate(0),
            ),
            vec!(
                encounter_factory.get_one(),
                encounter_factory.get_one(),
                encounter_factory.get_one()
            ))
        );
        m.insert(1, self.generate_map_seed(
            "1", (30, 40), (5, 6), vec!(), vec!())
        );
        m.insert(2, self.generate_map_seed(
            "2", (50, 80), (5, 6), vec!(), vec!())
        );

        self.world = m;
    }

    fn prepare_arena(&mut self, m: &mut Monster) -> bool {
        // TODO: start this initialization in MapCore::new
        let arena = arena::Arena::new(self.rng, self.is_debug);
        arena.prepare(&mut self.player, m)
    }

    fn found_item(&mut self, e: &mut Item) {
        let mut arena = arena::Arena::new(self.rng, self.is_debug);
        arena.found_item(&mut self.player, e);
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
                    match next_map.enterpoints.get(&map_index) {
                        Some(map_points) => {
                            return self.point(map_index, map_points.0, map_points.1);
                        },
                        None => panic!("No enterpoint {} found at map #{}", map_index, index),
                    }
                } else {
                    match next_map.exits.get(&index) {
                        Some(map_points) => {
                            return self.point(map_index, map_points.0, map_points.1);
                        },
                        None => panic!("No exit {} found at map #{}", map_index, index),
                    }
                }
            } else {
                let actual_event_point = self.event_point.get(&(index, Position2d { x, y }));
                match actual_event_point {
                    Some(event) => {
                        if self.is_debug {
                            println!("REPLAY!!!");
                            println!("{:?}", event);
                        }

                        if let Some(mut m) = event.monster.clone() {
                            if m.vital_points.life <= 0 {
                                println!("You see a dead {}", m.description);
                            } else {
                                is_player_alive = self.prepare_arena(&mut m);
                            }
                        } else if let Some(mut e) = event.encounter.clone() {
                            self.found_item(&mut e);
                        }

                    },
                    // Let's get the random one :D
                    None => {
                        let interactions = map.events.get(&Position2d {
                            x, y
                        });
                        match interactions {
                            Some(i) => {
                                self.event_point.insert((index, Position2d {
                                    x, y
                                }), i.clone());
                                let monster = i.monster.clone();
                                let encounter = i.encounter.clone();

                                // TODO: Add other events here!
                                if let Some(mut m) = monster {
                                    is_player_alive = self.prepare_arena(&mut m);
                                    
                                    // Override previous state when exists fight
                                    // for now only fights to DEATH will be allowed
                                    // TODO: escape using skills, etc
                                    if is_player_alive {
                                        self.event_point.insert((index, Position2d { x, y }), Event {
                                            monster: Some(m),
                                            encounter: None,
                                        });
                                    }
                                } else if let Some(mut e) = encounter {
                                    self.found_item(&mut e);
                                    // Override encounter state
                                    self.event_point.insert((index, Position2d { x, y }), Event {
                                        monster: None,
                                        encounter: Some(e),
                                    });
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
            directions,
            x,
            y,
            index,
            is_player_alive,
        }
    }
}
