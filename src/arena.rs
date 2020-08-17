use rand::Rng;
use rand::rngs::ThreadRng;

use super::interaction;
use super::player::*;
use super::monster::*;
use super::itens::*;
use super::render;

#[derive(Debug, Clone)]
pub struct Arena {
    rng: ThreadRng,
    is_debug: bool,
}

impl Arena {
    pub fn new(rng: ThreadRng, is_debug: bool) -> Self {
        Self {
            rng,
            is_debug
        }
    }

    fn fight(&self, player: &mut Player, monster: &mut Monster, is_player_action: bool) -> bool {
        if self.is_debug {
            println!("Monster {:?}\n", monster);
            println!("Player {:?}\n", player);
        }

        if monster.vital_points.life <= 0 {
            println!("The monster is dead!");
            return true;
        }
    
        if player.vital_points.life <= 0 {
            println!("You are DEAD! Nooooo....");
            println!("{:?}", player);
            return false;
        }
    
        if is_player_action {
            println!("You turn!");
            let input: String = interaction::capture_input("What attack will you perform?", "", "You choosed", vec!(
                // TODO: actions to enum by class
                String::from("attack"),
            ));
        
            match input.as_str() {
                "attack" => {
                    let hit = player.attack();
                    monster.vital_points.life -= hit;
                    self.fight(player, monster, !is_player_action)
                },
                _ => {
                    panic!("What?");
                }
            }
        } else {
            println!("The monster {} turn", monster.description);
            let hit = monster.attack();
            player.vital_points.life -= hit;
            render::render_mini_stats(player.vital_points);
            self.fight(player, monster, !is_player_action)
        }
    }
    
    pub fn prepare(&self, player: &mut Player, monster: &mut Monster) -> bool {
        println!("You meet an {}\n", monster.description);
    
        // TODO: Aggro monsters! Without this prompt
        let input: String = interaction::capture_input("What you will do?", "", "You choosed", vec!(
            // TODO: actions to enum 
            String::from("start"),
            String::from("nothing")
        ));
    
        match input.as_str() {
            "start" => {
                let is_player_action = player.stats.agility >= monster.stats.agility;
                self.fight(player, monster, is_player_action)
            },
            "nothing" => {
                true
            }
            _ => {
                panic!("What?");
            }
        }
    }

    // TODO: remove this from arena module!!
    pub fn found_item(&mut self, player: &mut Player, encounter: &mut Item) {
        if self.is_debug {
            println!("Player {:?}\n", player);
        }

        if encounter.is_used {
            println!("You found an {}\n", encounter.used_description);
            return;
        }

        println!("You found an {}\n", encounter.description);
        let kind = &encounter.kind;
        let action;

        match kind {
            Kind::Potion => {
                action = "drink"
            },
            Kind::Weapon => {
                action = "pick"
            }
        }

        let input: String = interaction::capture_input("What you will do?", "", "You choosed", vec!(
            // TODO: actions to enum 
            action.to_string(),
            String::from("nothing")
        ));

        println!("{}", input);

        match input.as_str() {
            "drink" => {
                if encounter.can_be_evil {
                    let rand = self.rng.gen_range(0, 2);
                    if rand == 0 {
                        println!("Ouch, something goes wrong!");
                        player.vital_points.life -= 5;
                    } else {
                        println!("Yeah... you feel better");
                        player.vital_points.life += 5;
                    }
                }
                encounter.is_used = true;
            },
            "pick" => {
                player.weapon = encounter.weapon.clone();
                println!("You get a nice {}", encounter.description);
                encounter.is_used = true;
            }
            "nothing" => {
                
            }
            _ => {
                panic!("What?");
            }
        }
    }
}