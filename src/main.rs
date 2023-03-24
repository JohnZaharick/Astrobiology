mod gui;
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

// store every possible seed so any scene can be regenerated
struct Scene<S> {
    state: S,
    galaxy_size: u64,
    star_seed: star_generator::Star,
    planet_seed: planet_generator::Planet,
}

impl Scene<Galaxy> {
    fn new(number_of_stars: u64) -> Self {

        let star = star_generator::Star::new(0);
        let planet = planet_generator::Planet::new(&star, 1);

        Scene {
            state: galaxy_generator::Galaxy::new(number_of_stars),
            galaxy_size: number_of_stars,
            star_seed: star,
            planet_seed: planet,
        }
    }
}

impl From<Scene<Galaxy>> for Scene<PlanetarySystem> {
    fn from(galaxy: Scene<Galaxy>) -> Scene<PlanetarySystem> {
        Scene {
            state: planetary_system_generator::PlanetarySystem::new(&galaxy.star_seed),
            galaxy_size: galaxy.galaxy_size,
            star_seed: galaxy.star_seed,
            planet_seed: galaxy.planet_seed,
        }
    }
}

impl From<Scene<PlanetarySystem>> for Scene<Galaxy> {
    fn from(star: Scene<PlanetarySystem>) -> Scene<Galaxy> {
        Scene {
            state: galaxy_generator::Galaxy::new(star.galaxy_size),
            galaxy_size: star.galaxy_size,
            star_seed: star.star_seed,
            planet_seed: star.planet_seed,
        }
    }
}

impl From<Scene<PlanetarySystem>> for Scene<PlanetaryEnvironment> {
    fn from(star: Scene<PlanetarySystem>) -> Scene<PlanetaryEnvironment> {
        Scene {
            state: planetary_environment_generator::PlanetaryEnvironment::new(&star.planet_seed),
            galaxy_size: star.galaxy_size,
            star_seed: star.star_seed,
            planet_seed: star.planet_seed,
        }
    }
}

impl From<Scene<PlanetaryEnvironment>> for Scene<PlanetarySystem> {
    fn from(planet: Scene<PlanetaryEnvironment>) -> Scene<PlanetarySystem> {
        Scene {
            state: planetary_system_generator::PlanetarySystem::new(&planet.star_seed),
            galaxy_size: planet.galaxy_size,
            star_seed: planet.star_seed,
            planet_seed: planet.planet_seed,
        }
    }
}

enum SceneId {
    Galaxy(Scene<Galaxy>),
    Star(Scene<PlanetarySystem>),
    Planet(Scene<PlanetaryEnvironment>),
}

impl SceneId {
    fn step_in(mut self, mut seed: usize) -> Self {
        self = match self {
            SceneId::Galaxy(mut scene) => {
                if seed >= scene.state.stars.len() { seed = 0; }
                scene.star_seed = scene.state.stars.remove(seed);
                SceneId::Star(scene.into())
            },
            SceneId::Star(mut scene) => {
                if seed >= scene.state.planets.len() { seed = 0; }
                scene.planet_seed = scene.state.planets.remove(seed);
                SceneId::Planet(scene.into())
            },
            SceneId::Planet(mut scene) => {
                SceneId::Planet(scene.into())
            },
        };
        match &self {
            SceneId::Galaxy(scene) =>
                println!("{}", &scene.state.get_galaxy_info()),
            SceneId::Star(scene) =>
                println!("{}", &scene.state.get_planetary_system_info()),
            SceneId::Planet(scene) =>
                println!("{}", &scene.state.get_planet_info()),
        };
        self
    }

    fn step_out(mut self) -> Self {
        self = match self {
            SceneId::Galaxy(scene) =>
                SceneId::Galaxy(scene.into()),
            SceneId::Star(scene) =>
                SceneId::Galaxy(scene.into()),
            SceneId::Planet(scene) =>
                SceneId::Star(scene.into()),
        };
        match &self {
            SceneId::Galaxy(scene) =>
                println!("{}", &scene.state.get_galaxy_info()),
            SceneId::Star(scene) =>
                println!("{}", &scene.state.get_planetary_system_info()),
            SceneId::Planet(scene) =>
                println!("{}", &scene.state.get_planet_info()),
        };
        self
    }
}

struct Game {
    scene_id: SceneId,
}

impl Game {
    fn new() -> Self {
        Game {
            scene_id: SceneId::Galaxy(Scene::new(STARS_IN_GALAXY)),
        }
    }
}

fn main() {
    print_title_screen();

    let mut game = Game::new();
    let mut coord: usize = 0;

    if let SceneId::Galaxy(scene) = &game.scene_id {
        println!("{}", &scene.state.get_galaxy_info());
        // gui::render_galaxy(&scene.state);
    }

    loop {
        let command: Commands = parse_input();

        match command {
            Commands::Coord(index) => {
                match &game.scene_id {
                    SceneId::Galaxy(scene) =>
                        println!("{}", &scene.state.get_star_info(index)),
                        //gui::render_galaxy(&scene.state),
                    SceneId::Star(scene) =>
                        println!("{}", &scene.state.get_planet_info(index)),
                        //gui::render_planetary_system(&scene.star_seed, &scene.state),
                    SceneId::Planet(scene) =>
                        println!("{}", &scene.state.get_organism_info(index)),
                }
                coord = index;
            }
            Commands::Explore => {
                game.scene_id = game.scene_id.step_in(coord);
            }
            Commands::Leave => {
                game.scene_id = game.scene_id.step_out();
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