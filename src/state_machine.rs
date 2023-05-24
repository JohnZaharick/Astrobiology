use crate::galaxy_generator;
use crate::galaxy_generator::Galaxy;
use crate::planetary_system_generator;
use crate::planetary_system_generator::PlanetarySystem;
use crate::planetary_environment_generator;
use crate::planetary_environment_generator::PlanetaryEnvironment;
use crate::star_generator;
use crate::planet_generator;

// store every possible seed so any scene can be regenerated
struct Scene<S> {
    state: S,
    galaxy_size: u64,
    star_seed: star_generator::Star,
    planet_seed: planet_generator::Planet,
}

impl Scene<Galaxy> {
    fn new(number_of_stars: u64) -> Self {

        let star = star_generator::Star::new(0);
        let planet = planet_generator::Planet::new(&star, 1);

        Scene {
            state: galaxy_generator::Galaxy::new(number_of_stars),
            galaxy_size: number_of_stars,
            star_seed: star,
            planet_seed: planet,
        }
    }
}

impl From<Scene<Galaxy>> for Scene<PlanetarySystem> {
    fn from(galaxy: Scene<Galaxy>) -> Scene<PlanetarySystem> {
        Scene {
            state: planetary_system_generator::PlanetarySystem::new(&galaxy.star_seed),
            galaxy_size: galaxy.galaxy_size,
            star_seed: galaxy.star_seed,
            planet_seed: galaxy.planet_seed,
        }
    }
}

impl From<Scene<PlanetarySystem>> for Scene<Galaxy> {
    fn from(star: Scene<PlanetarySystem>) -> Scene<Galaxy> {
        Scene {
            state: galaxy_generator::Galaxy::new(star.galaxy_size),
            galaxy_size: star.galaxy_size,
            star_seed: star.star_seed,
            planet_seed: star.planet_seed,
        }
    }
}

impl From<Scene<PlanetarySystem>> for Scene<PlanetaryEnvironment> {
    fn from(star: Scene<PlanetarySystem>) -> Scene<PlanetaryEnvironment> {
        Scene {
            state: planetary_environment_generator::PlanetaryEnvironment::new(&star.planet_seed),
            galaxy_size: star.galaxy_size,
            star_seed: star.star_seed,
            planet_seed: star.planet_seed,
        }
    }
}

impl From<Scene<PlanetaryEnvironment>> for Scene<PlanetarySystem> {
    fn from(planet: Scene<PlanetaryEnvironment>) -> Scene<PlanetarySystem> {
        Scene {
            state: planetary_system_generator::PlanetarySystem::new(&planet.star_seed),
            galaxy_size: planet.galaxy_size,
            star_seed: planet.star_seed,
            planet_seed: planet.planet_seed,
        }
    }
}

pub enum SceneId {
    Galaxy(Scene<Galaxy>),
    Star(Scene<PlanetarySystem>),
    Planet(Scene<PlanetaryEnvironment>),
}

impl SceneId {
    pub fn get_system_info(&self) -> String {
        match &self {
            SceneId::Galaxy(scene) =>
                scene.state.get_galaxy_info(),
            SceneId::Star(scene) =>
                scene.state.get_planetary_system_info(),
            SceneId::Planet(scene) =>
                scene.state.get_planet_info(),
        }
    }

    pub fn get_unit_info(&self, index: usize) -> String {
        match &self {
            SceneId::Galaxy(scene) =>
                scene.state.get_star_info(index),
            SceneId::Star(scene) =>
                scene.state.get_planet_info(index),
            SceneId::Planet(scene) =>
                scene.state.get_organism_info(index),
        }
    }
}

impl SceneId {
    pub fn step_in(mut self, mut seed: usize) -> Self {
        self = match self {
            SceneId::Galaxy(mut scene) => {
                scene.star_seed = scene.state.get_star(seed);
                SceneId::Star(scene.into())
            },
            SceneId::Star(mut scene) => {
                scene.planet_seed = scene.state.get_planet(seed);
                SceneId::Planet(scene.into())
            },
            SceneId::Planet(mut scene) => {
                SceneId::Planet(scene.into())
            },
        };
        self
    }

    pub fn step_out(mut self) -> Self {
        self = match self {
            SceneId::Galaxy(scene) =>
                SceneId::Galaxy(scene.into()),
            SceneId::Star(scene) =>
                SceneId::Galaxy(scene.into()),
            SceneId::Planet(scene) =>
                SceneId::Star(scene.into()),
        };
        self
    }
}

pub struct Game {
    pub scene_id: SceneId,
}

impl Game {
    pub fn new(stars_in_galaxy: u64) -> Self {
        Game {
            scene_id: SceneId::Galaxy(Scene::new(stars_in_galaxy)),
        }
    }
}