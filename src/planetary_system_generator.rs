use rand::prelude::*;

use crate::star_generator;
use crate::planet_generator;

const PLANET_NUMBER_MINIMUM: u8 = 3;
const PLANET_NUMBER_MAXIMUM: u8 = 12;

struct PlanetarySystem {
    star: star_generator::Star,
    planets: Vec<planet_generator::Planet>,
}

impl PlanetarySystem {
    fn new() -> Self {
        let mut rng = rand::thread_rng();

        PlanetarySystem {
            star: star_generator::Star::new(),
            planets: planet_generator::Planet::planets(
                rng.gen_range(PLANET_NUMBER_MINIMUM..=PLANET_NUMBER_MAXIMUM)),
        }
    }
}

pub fn explore_system() -> String {
    let solar_system = PlanetarySystem::new();
    format!("There is an {} star that is {}, {} million years old, weighs {} M☉, and is {} K. \
            It has {} planets.", solar_system.star.class, solar_system.star.color,
            solar_system.star.age, solar_system.star.mass, solar_system.star.temperature,
            solar_system.planets.len())
}