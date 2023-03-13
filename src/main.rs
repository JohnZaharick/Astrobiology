mod galaxy_generator;
mod planet_generator;
mod planetary_system_generator;
mod star_generator;

use std::io;

const STARS_IN_GALAXY: usize = 100;

enum Commands {
    Coord(usize),
    Up,
    Explore,
    Invalid,
}

enum Scene {
    Galaxy,
    Star,
}

fn parse_input(vec_length: usize, scene: &Scene) -> Commands {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line.");

    match input.trim().parse::<usize>() {
        Ok(index) => {
            if index < vec_length{
                Commands::Coord(index)
            }
            else {
                Commands::Invalid
            }
        }
        Err(_) => {
            match &input.trim().to_lowercase() as &str {
                "quit" => {
                    match scene {
                        Scene::Galaxy => {
                            Commands::Up
                        }
                        Scene::Star => {
                            Commands::Invalid
                        }
                    }
                }
                "up" => {
                    match scene {
                        Scene::Galaxy => {
                            Commands::Invalid
                        }
                        Scene::Star => {
                            Commands::Up
                        }
                    }
                }
                "explore" => {
                    Commands::Explore
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

fn explore_galaxy(number_of_stars: usize) {
    let galaxy = galaxy_generator::Galaxy::new(number_of_stars);
    println!("{}", galaxy.get_info());

    let mut coord: usize = 0;
    let scene = Scene::Galaxy;

    loop {
        let command: Commands = parse_input(galaxy.stars.len(), &scene);
        match command {
            Commands::Coord(index) => {
                println!("{}", &galaxy.stars[index].get_info());
                println!("Type EXPLORE to travel to this star.");
                coord = index;
            }
            Commands::Up => {
                break;
            }
            Commands::Explore => {
                explore_planetary_system(galaxy.stars[coord].clone());
            }
            Commands::Invalid => {
                println!("Invalid coordinates.")
            }
        }
    }
}

fn explore_planetary_system(star: star_generator::Star)  {
    let solar_system = planetary_system_generator::PlanetarySystem::new(star);
    println!("{}", solar_system.get_info());

    // let mut coord: usize = 0;
    let scene = Scene::Star;

    loop {
        let command: Commands = parse_input(solar_system.planets.len(), &scene);
        match command {
            Commands::Coord(index) => {
                println!("{}", &solar_system.planets[index].get_info());
                println!("Type UP to return to the galactic view.")
                // coord = index;
            }
            Commands::Up => {
                break;
            }
            Commands::Explore => {
                // explore_planet(solar_system.planets.remove(coord));
                println!("TO DO");
            }
            Commands::Invalid => {
                println!("Invalid coordinates.");
            }
        }
    }
}

fn main() {
    print_title_screen();
    explore_galaxy(STARS_IN_GALAXY);
    println!("Thanks for playing!");
}
