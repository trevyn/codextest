use rand::rng;
use rand::Rng;
use std::env;
use std::io::{self, Write};

struct Character {
    name: String,
    hp: i32,
    max_hp: i32,
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

fn run_game(automatic: bool) {
    println!("Welcome to Rusty RPG!");
    let mut player = Character {
        name: String::from("Hero"),
        hp: 30,
        max_hp: 30,
    };
    let mut enemy = Character {
        name: String::from("Goblin"),
        hp: 20,
        max_hp: 20,
    };

    while player.hp > 0 && enemy.hp > 0 {
        println!(
            "\nYour HP: {}/{} | Enemy HP: {}/{}",
            player.hp, player.max_hp, enemy.hp, enemy.max_hp
        );

        let action_string;
        let action = if automatic {
            action_string = decide_action(&player).to_string();
            println!("[AUTO] Action chosen: {}", action_string);
            action_string.as_str()
        } else {
            print!("Choose action: (a)ttack, (h)eal, (q)uit > ");
            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
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
                return;
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
    } else {
        println!("You defeated the {}!", enemy.name);
    }
}

fn main() {
    let automatic = env::args().any(|arg| arg == "--automatic");
    run_game(automatic);
}
