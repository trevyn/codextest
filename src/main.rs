use rand::rng;
use rand::Rng;
use std::env;
use std::io::{self, Write};

const GAME_FEATURES: &str = include_str!("../GAME_FEATURES.md");

fn print_features() {
    println!("Game Features:");
    for line in GAME_FEATURES.lines() {
        if let Some(stripped) = line.strip_prefix("- ") {
            println!(" - {}", stripped);
        }
    }
    println!("");
}

#[derive(Clone)]
struct Character {
    name: String,
    hp: i32,
    max_hp: i32,
}

struct Location {
    name: &'static str,
    description: &'static str,
    enemy: Option<Character>,
}

fn rand_range(min: i32, max: i32) -> i32 {
    rng().random_range(min..=max)
}

fn decide_action(player: &Character) -> &'static str {
    if player.hp < player.max_hp / 2 && rand_range(0, 1) == 0 {
        "heal"
    } else {
        "attack"
    }
}

fn battle(player: &mut Character, enemy: &mut Character, automatic: bool) -> bool {
    while player.hp > 0 && enemy.hp > 0 {
        println!(
            "\nYour HP: {}/{} | {} HP: {}/{}",
            player.hp, player.max_hp, enemy.name, enemy.hp, enemy.max_hp
        );

        let action_string;
        let action = if automatic {
            action_string = decide_action(player).to_string();
            println!("[AUTO] Action chosen: {}", action_string);
            action_string.as_str()
        } else {
            print!("Choose action: (a)ttack, (h)eal, (q)uit > ");
            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");
            action_string = input.trim().to_string();
            action_string.as_str()
        };

        match action {
            "a" | "attack" => {
                let damage = rand_range(3, 6);
                println!("You hit the {} for {} damage!", enemy.name, damage);
                enemy.hp -= damage;
            }
            "h" | "heal" => {
                let heal = rand_range(4, 8);
                player.hp = (player.hp + heal).min(player.max_hp);
                println!("You heal yourself for {} HP.", heal);
            }
            "q" | "quit" => {
                println!("You fled from battle!");
                return false;
            }
            _ => {
                println!("Invalid action.");
                continue;
            }
        }

        if enemy.hp > 0 {
            let damage = rand_range(2, 5);
            println!("{} strikes back for {} damage!", enemy.name, damage);
            player.hp -= damage;
        }
    }

    if player.hp <= 0 {
        println!("You have been defeated by the {}!", enemy.name);
        false
    } else {
        println!("You defeated the {}!", enemy.name);
        true
    }
}

fn run_game(automatic: bool) {
    println!("Welcome to Rusty RPG!");
    if !automatic {
        print_features();
    }
    let mut player = Character {
        name: String::from("Hero"),
        hp: 30,
        max_hp: 30,
    };

    let mut locations = vec![
        Location {
            name: "Village of Beginnings",
            description: "A quiet place where travelers rest.",
            enemy: None,
        },
        Location {
            name: "Mystic Woods",
            description: "Trees whisper as goblins lurk in the shadows.",
            enemy: Some(Character {
                name: String::from("Goblin"),
                hp: 20,
                max_hp: 20,
            }),
        },
        Location {
            name: "Ancient Ruins",
            description: "Crumbled stones hide shambling skeletons.",
            enemy: Some(Character {
                name: String::from("Skeleton"),
                hp: 25,
                max_hp: 25,
            }),
        },
        Location {
            name: "Dragon's Lair",
            description: "The final challenge awaits in the fiery depths.",
            enemy: Some(Character {
                name: String::from("Dragon"),
                hp: 40,
                max_hp: 40,
            }),
        },
    ];

    let mut index = 0;
    while player.hp > 0 && index < locations.len() {
        let location = &mut locations[index];
        println!("\n== {} ==", location.name);
        println!("{}", location.description);

        if let Some(mut enemy) = location.enemy.take() {
            println!("A {} appears!", enemy.name);
            if !battle(&mut player, &mut enemy, automatic) {
                return;
            }
        } else {
            println!("The area is peaceful.");
        }

        if index == locations.len() - 1 {
            break;
        }

        let mut action_string = String::new();
        let action = if automatic {
            if player.hp < player.max_hp / 2 {
                "rest"
            } else {
                "move"
            }
        } else {
            print!("Choose next step: (m)ove, (r)est, (q)uit > ");
            io::stdout().flush().unwrap();
            io::stdin()
                .read_line(&mut action_string)
                .expect("Failed to read line");
            action_string.trim()
        };

        match action {
            "m" | "move" => {
                index += 1;
            }
            "r" | "rest" => {
                let heal = rand_range(4, 8);
                player.hp = (player.hp + heal).min(player.max_hp);
                println!("You rest and recover {} HP.", heal);
            }
            "q" | "quit" => {
                println!("You abandon your quest.");
                return;
            }
            _ => {
                println!("Invalid action.");
            }
        }
    }

    if player.hp > 0 {
        println!("You have conquered the land. A hero is born!");
    }
}

fn main() {
    let automatic = env::args().any(|arg| arg == "--automatic");
    run_game(automatic);
}
