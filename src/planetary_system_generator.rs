use rand::prelude::*;

use crate::star_generator;

const PLANET_NUMBER_MINIMUM: u8 = 3;
const PLANET_NUMBER_MAXIMUM: u8 = 12;

struct PlanetarySystem {
    star: star_generator::Star,
    planets: u8,
}

impl PlanetarySystem {
    fn new() -> Self {
        let mut rng = rand::thread_rng();

        PlanetarySystem {
            star: star_generator::Star::new(),
            planets: rng.gen_range(PLANET_NUMBER_MINIMUM..PLANET_NUMBER_MAXIMUM),
        }
    }
}

pub fn explore_system() -> String {
    let solar_system = PlanetarySystem::new();
    format!("There is an {} star that is {}, {} million years old, and weighs {} M☉. \
            It has {} planets.", solar_system.star.class, solar_system.star.color,
            solar_system.star.age, solar_system.star.mass, solar_system.planets)
}