use crate::planet_generator;
use crate::star_generator;
use crate::Galaxy;
use crate::PlanetarySystem;
use crate::PlanetaryEnvironment;

pub enum SceneName {
    Galaxy,
    PlanetarySystem,
    PlanetaryEnvironment,
}

pub trait Scene {
    fn get_scene_name(&self) -> SceneName;
    fn get_system_info(&self) -> String;
    fn get_unit_info(&self, index: usize) -> String;
}

pub struct Game {
    pub scene: Box <dyn Scene>,
    pub galaxy_size: u64,
    pub star_seed: star_generator::Star,
    pub planet_seed: planet_generator::Planet,
}

impl Game {
    pub fn new(number_of_stars: u64) -> Game {

        let star = star_generator::Star::new(0);
        let planet = planet_generator::Planet::new(&star, 1);

        Game {
            scene: Box::new(Galaxy::new(number_of_stars)),
            galaxy_size: number_of_stars,
            star_seed: star,
            planet_seed: planet,
        }
    }

    pub fn step_in(&mut self, index: usize){
        match self.scene.get_scene_name() {
            SceneName::Galaxy => {
                self.star_seed = star_generator::Star::new(index as u64);
                self.scene = Box::new(PlanetarySystem::new(&self.star_seed));
            }
            SceneName::PlanetarySystem => {
                self.planet_seed = planet_generator::Planet::new(&self.star_seed, index as u8 + 1);
                self.scene = Box::new(PlanetaryEnvironment::new(&self.planet_seed));
            }
            SceneName::PlanetaryEnvironment => {
                println!("You can't go any further inward. \n");
            }
        }
    }

    pub fn step_out(&mut self){
        match self.scene.get_scene_name() {
            SceneName::Galaxy => {
                println!("You can't go any further outward. \n");
            }
            SceneName::PlanetarySystem => {
                self.scene = Box::new(Galaxy::new(self.galaxy_size));
            }
            SceneName::PlanetaryEnvironment => {
                self.scene = Box::new(PlanetarySystem::new(&self.star_seed));
            }
        }
    }
}