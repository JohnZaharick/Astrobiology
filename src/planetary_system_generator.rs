use rand::prelude::*;

use crate::game_manager::{Scene, SceneName};
use crate::planet_generator;
use crate::star_generator;

const PLANET_NUMBER_MINIMUM: u8 = 3;
const PLANET_NUMBER_MAXIMUM: u8 = 12;

pub struct PlanetarySystem {
    planets: Vec<planet_generator::Planet>,
}

impl PlanetarySystem {
    pub fn new(star: &star_generator::Star) -> Self {
        let mut rng = StdRng::seed_from_u64(star.get_age() as u64);

        PlanetarySystem {
            planets: Self::generate_planets(
                &star,
                rng.gen_range(PLANET_NUMBER_MINIMUM..=PLANET_NUMBER_MAXIMUM),
            ),
        }
    }

    fn generate_planets(star: &star_generator::Star, count: u8) -> Vec<planet_generator::Planet> {
        let mut planets = Vec::new();
        for i in 1..count {
            planets.push(planet_generator::Planet::new(star, i));
        }
        planets
    }
}

impl Scene for PlanetarySystem {
    fn get_scene_name(&self) -> SceneName {
        SceneName::PlanetarySystem
    }

    fn get_system_info(&self) -> String {
        let mut s = String::new();
        for i in 0..self.planets.len() {
            s.push_str(&self.planets[i].get_class().to_string());
            s.push_str("_");
            s.push_str(&i.to_string());
            s.push_str(" ");
        }
        format!("There are {} planets: {}", &self.planets.len(), s)
    }

    fn get_unit_info(&self, index: usize) -> String {
        if index < self.planets.len() {
            self.planets[index].get_info()
        }
        else {
            format!("Invalid coordinates.")
        }
    }
}
