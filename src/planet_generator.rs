use strum_macros::Display;
use rand::prelude::*;
use rand_derive2::RandGen;

#[derive(Display, RandGen)]
pub enum PlanetClass{
    Rocky, GasGiant, IceGiant, Dwarf,
}

pub struct Planet {
    pub class: PlanetClass,
}

impl Planet {
    fn new() -> Planet{
        Planet {
            class: random(),
        }
    }

    pub fn planets(count: u8) -> Vec<Planet> {
        let mut planets = Vec::new();
        for _temp in 1..count{
            planets.push(Planet::new());
        }
        planets
    }
}