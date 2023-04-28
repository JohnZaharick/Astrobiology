mod state_machine;
mod galaxy_generator;
mod planet_generator;
mod planetary_system_generator;
mod star_generator;
mod planetary_environment_generator;
mod organism_generator;

use std::io;
use std::io::Write;
use crate::galaxy_generator::Galaxy;
use crate::planetary_environment_generator::PlanetaryEnvironment;
use crate::planetary_system_generator::PlanetarySystem;

const STARS_IN_GALAXY: u64 = 100;

enum Commands {
    Coord(usize),
    Exit,
    Explore,
    Leave,
    Invalid,
}

fn parse_input() -> Commands {

    println!();
    print!("> ");
    io::stdout().flush().unwrap();
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line.");

    // add newline after input
    println!();

    match input.trim().parse::<usize>() {
        Ok(index) => {
            Commands::Coord(index)
        }
        Err(_) => {
            match &input.trim().to_lowercase() as &str {
                "quit" => {
                    Commands::Exit
                }
                "exit" => {
                    Commands::Exit
                }
                "explore" => {
                    Commands::Explore
                }
                "leave" => {
                    Commands::Leave
                }
                _ => {
                    Commands::Invalid
                }
            }
        }
    }
}

fn print_title_screen(){
    println!("------------------------");
    println!("A S T R O B I O L O G Y");
    println!("------------------------");
    println!();
    println!("Enter a star's value to measure its properties. Type QUIT and ENTER to end game.");
    println!();
}



fn main() {
    print_title_screen();

    let mut game = state_machine::Game::new(STARS_IN_GALAXY);
    let mut coord: usize = 0;

    println!("{}", game.scene_id.get_system_info());

    loop {
        let command: Commands = parse_input();

        match command {
            Commands::Coord(index) => {
                println!("{}", &game.scene_id.get_unit_info(index));
                coord = index;
            }
            Commands::Explore => {
                game.scene_id = game.scene_id.step_in(coord);
                println!("{}", game.scene_id.get_system_info());
            }
            Commands::Leave => {
                game.scene_id = game.scene_id.step_out();
                println!("{}", game.scene_id.get_system_info());
            }
            Commands::Exit => {
                break;
            }
            Commands::Invalid => {
                println!("Invalid coordinates.")
            }
        }
    }
    println!("Thanks for playing!");
}