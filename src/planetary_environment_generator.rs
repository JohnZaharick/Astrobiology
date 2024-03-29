use rand::prelude::*;

use crate::game_manager::{Scene, SceneName};
use crate::planet_generator;
use crate::organism_generator::Organism;
use crate::planet_generator::PlanetClass;

const RICHNESS_MINIMUM: u8 = 3;
const RICHNESS_MAXIMUM: u8 = 12;

pub struct PlanetaryEnvironment {
    moons: usize,
    biosphere: Vec<Organism>,
}

impl PlanetaryEnvironment {
    pub fn new (planet: &planet_generator::Planet) -> PlanetaryEnvironment {
        let mut rng = StdRng::seed_from_u64(planet.get_temperature() as u64);

        let number_of_moons = match planet.get_class() {
            PlanetClass::Rocky => { rng.gen_range(0..=5) }
            PlanetClass::GasGiant => { rng.gen_range(15..=80) }
            PlanetClass::IceGiant => { rng.gen_range(15..=30) }
            PlanetClass::Dwarf => { rng.gen_range(0..=5) }
        };

        PlanetaryEnvironment {
            moons: number_of_moons,
            biosphere: Self::generate_organisms(
                &planet,
                rng.gen_range(RICHNESS_MINIMUM..=RICHNESS_MAXIMUM),
            ),
        }
    }

    fn generate_organisms(planet: &planet_generator::Planet, richness: u8) -> Vec<Organism> {
        let mut organisms = Vec::new();

        if planet.get_habitability() {
            for i in 1..richness {
                organisms.push(Organism::new(planet, (planet.get_temperature() + i as u16) as u64));
            }
        }
        organisms
    }

}

impl Scene for PlanetaryEnvironment {
    fn get_scene_name(&self) -> SceneName {
        SceneName::PlanetaryEnvironment
    }

    fn get_system_info(&self) -> String {
        let mut s = String::new();
        for i in 0..self.biosphere.len() {
            s.push_str(&self.biosphere[i].get_size().to_string());
            s.push_str("_");
            s.push_str(&i.to_string());
            s.push_str(" ");
        }
        format!("There are {} moons and {}", &self.moons, s)
    }

    fn get_unit_info(&self, index: usize) -> String {
        if index < self.biosphere.len() {
            self.biosphere[index].get_info()
        }
        else {
            format!("Invalid coordinates.")
        }
    }
}