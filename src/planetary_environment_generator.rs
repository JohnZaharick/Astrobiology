use rand::prelude::*;

use crate::planet_generator;
use crate::organism_generator;

const RICHNESS_MINIMUM: u8 = 3;
const RICHNESS_MAXIMUM: u8 = 12;

pub struct PlanetaryEnvironment {
    pub moons: usize,
    pub biosphere: Vec<organism_generator::Organism>,
}

impl PlanetaryEnvironment {
    pub fn new (coord: u64) -> PlanetaryEnvironment {
        let mut rng = StdRng::seed_from_u64(coord);

        // let planet = planet_generator::Planet::new();

        PlanetaryEnvironment {
            moons: 2,
            biosphere: Self::organisms(
                rng.gen_range(RICHNESS_MINIMUM..=RICHNESS_MAXIMUM),
            ),
        }
    }

    fn organisms(count: u8) -> Vec<organism_generator::Organism> {
        let mut organisms = Vec::new();
        for i in 1..count {
            organisms.push(organism_generator::Organism::new());
        }
        organisms
    }

    pub fn get_organism_info(&self, index: usize) -> String {
        if index < self.biosphere.len() {
            self.biosphere[index].get_info()
        }
        else {
            format!("Invalid coordinates.")
        }
    }

    pub fn get_planet_info(&self) -> String{
        let mut s = String::new();
        for i in 0..self.biosphere.len() {
            s.push_str(&self.biosphere[i].size.to_string());
            s.push_str("_");
            s.push_str(&i.to_string());
            s.push_str(" ");
        }
        format!("There are {} moons and {}", &self.moons, s)
    }
}