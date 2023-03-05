use rand::prelude::*;
use rand_derive2::RandGen;
use strum_macros::Display;

use crate::star_generator;

#[derive(Display, RandGen)]
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
        Planet {
            class: random(),
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
