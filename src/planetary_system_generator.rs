use rand::prelude::*;

use crate::planet_generator;
use crate::star_generator;

const PLANET_NUMBER_MINIMUM: u8 = 3;
const PLANET_NUMBER_MAXIMUM: u8 = 12;

pub struct PlanetarySystem {
    pub planets: Vec<planet_generator::Planet>,
    pub star: star_generator::Star,
}

impl PlanetarySystem {
    pub fn new(star: star_generator::Star) -> Self {
        let mut rng = StdRng::seed_from_u64(*&star.age as u64);

        PlanetarySystem {
            planets: Self::planets(
                &star,
                rng.gen_range(PLANET_NUMBER_MINIMUM..=PLANET_NUMBER_MAXIMUM),
            ),
            star,
        }
    }

    fn planets(star: &star_generator::Star, count: u8) -> Vec<planet_generator::Planet> {
        let mut planets = Vec::new();
        for i in 1..count {
            planets.push(planet_generator::Planet::new(star, i));
        }
        planets
    }

    pub fn get_info(&self) -> String{
        let mut s = String::new();
        for i in 0..self.planets.len() {
            s.push_str(&self.planets[i].class.to_string());
            s.push_str("_");
            s.push_str(&i.to_string());
            s.push_str(" ");
        }
        format!("There are {} planets: {}", &self.planets.len(), s)
    }
}
