use rand::prelude::*;
use strum_macros::Display;
use strum_macros::EnumIter;
use strum::IntoEnumIterator;

use crate::star_generator;

#[derive(Display, EnumIter)]
pub enum PlanetClass {
    Rocky,
    GasGiant,
    IceGiant,
    Dwarf,
}

// temperature;
// radiation;
// pressure;
// mass;
// magneticField;
// isHabitable;
// liquid;
pub struct Planet {
    pub class: PlanetClass,
    pub distance_from_star: u8,
    pub habitable: bool,
}

impl Planet {
    fn new(star: &star_generator::Star, distance: u8) -> Planet {

        let mut planet_classes = Vec::new();
        for planet in PlanetClass::iter() {
            planet_classes.push(planet);
        }
        let seed: u64 = star.age as u64 * distance as u64;
        let mut rng = StdRng::seed_from_u64(seed);
        let planet_class = rng.gen_range(0..planet_classes.len());

        Planet {
            class: planet_classes.remove(planet_class),
            distance_from_star: distance,
            habitable: if star.age > 1000 { true } else { false },
        }
    }

    pub fn planets(star: &star_generator::Star, count: u8) -> Vec<Planet> {
        let mut planets = Vec::new();
        for i in 1..count {
            planets.push(Planet::new(star, i));
        }
        planets
    }

    pub fn get_info(&self) -> String{
        format!("This is a {} planet, which is {} AU from its star,\
         and it's {} that there is life here.", &self.class, &self.distance_from_star,
                &self.habitable)
    }
}
