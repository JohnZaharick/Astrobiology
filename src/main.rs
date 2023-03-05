mod galaxy_generator;
mod planet_generator;
mod planetary_system_generator;
mod star_generator;

use std::io;

const STARS_IN_GALAXY: usize = 100;

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

    let mut coord: usize = STARS_IN_GALAXY;

    loop {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line.");

        match input.trim().parse::<usize>() {
            Ok(index) => {
                if index < STARS_IN_GALAXY {
                    println!(
                        "{}",
                        &galaxy.stars[index].get_info()
                    );
                    println!("Type EXPLORE to travel to this star.");
                    coord = index;
                }
                else { println!("Invalid coordinates.") }
            }
            Err(_) => {
                match input.trim() as &str {
                    "QUIT" => {
                        println!("Thanks for playing!");
                        break;
                    }
                    "EXPLORE" => {
                        explore_planetary_system(galaxy.stars[coord].clone());
                    }
                    _ => {
                        println!("Invalid coordinates.")
                    }
                }
            }
        }
    }
}

fn explore_planetary_system(star: star_generator::Star)  {
    let solar_system = planetary_system_generator::PlanetarySystem::new(star);
    println!("{}", solar_system.get_info());

    // let mut coord: usize = solar_system.planets.len();

    loop {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line.");

        match input.trim().parse::<usize>() {
            Ok(index) => {
                if index < solar_system.planets.len() {
                    println!(
                        "{}",
                        &solar_system.planets[index].get_info()
                    );
                    println!("Type UP to return to the galactic view.")
                    // coord = index;
                }
                else { println!("Invalid coordinates.") }
            }
            Err(_) => {
                match input.trim() as &str {
                    "UP" => {
                        println!("Enter a star's value to measure its properties. Type QUIT and ENTER to end game.");
                        break;
                    }
                    // "EXPLORE" => {
                    //     explore_planet(solar_system.planets.remove(coord));
                    // }
                    _ => {
                        println!("Invalid coordinates.")
                    }
                }
            }
        }
    }
}

fn main() {
    print_title_screen();
    explore_galaxy(STARS_IN_GALAXY);
}
