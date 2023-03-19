use rand::prelude::*;
use strum_macros::Display;
use strum_macros::EnumIter;
use strum::IntoEnumIterator;

use crate::star_generator;

const BACKGROUND_TEMPERATURE: u16 = 3; // in kelvins; prevents absolute zero worlds
const ATMOSPHERIC_INSULATION: u16 = 150; // in kelvins
const DISTANCE_FROM_STAR_MODIFIER: u16 = 4; // for calibrating how much temperature drops with distance from a star
const MINIMUM_STAR_AGE_FOR_LIFE: u16 = 1000; // in millions of years

#[derive(Display, EnumIter)]
pub enum PlanetClass {
    Rocky,
    GasGiant,
    IceGiant,
    Dwarf,
}

#[derive(PartialEq)]
pub enum Ocean {
    Water,
    Ammonia,
    None,
}

pub struct Planet {
    pub class: PlanetClass,
    pub distance_from_star: u8,
    pub mass: u8,
    pub magnetic_field: bool,
    pub pressure: u32,
    pub temperature: u16,
    pub ocean: Ocean,
    pub habitable: bool,
}

impl Planet {
    pub fn new(star: &star_generator::Star, distance: u8) -> Planet {

        let mut planet_classes = Vec::new();
        for planet in PlanetClass::iter() {
            planet_classes.push(planet);
        }

        let seed: u64 = star.age as u64 * distance as u64;
        let mut rng = StdRng::seed_from_u64(seed);
        let planet_class = rng.gen_range(0..planet_classes.len());

        // TODO: mass needs to be influenced by class
        let mass = rng.gen_range(1..10);
        let magnetic_field = if rng.gen_range(0..2) == 0 { false } else { true };
        // TODO: pressure needs to be influenced by mass; small planets can't have high pressures; large planets can't have low pressures
        let pressure = rng.gen_range(0..10) * 10_u32.pow(rng.gen_range(0..8));
        let temperature = Self::calculate_temperature(distance, &star, pressure);
        let ocean = Self::thalassogenesis(temperature, pressure);
        let habitable = if ocean != Ocean::None
            && magnetic_field && star.age > MINIMUM_STAR_AGE_FOR_LIFE { true } else { false };

        Planet {
            class: planet_classes.remove(planet_class),
            distance_from_star: distance,
            mass,
            magnetic_field,
            pressure,
            temperature,
            ocean,
            habitable,
        }
    }

    fn calculate_temperature (distance: u8, star: &star_generator::Star, pressure: u32) -> u16 {
        let atmospheric_insulation = if pressure > 0 { ATMOSPHERIC_INSULATION } else { 0 };
        &star.temperature / (distance as u16 * distance as u16 * DISTANCE_FROM_STAR_MODIFIER)
            + BACKGROUND_TEMPERATURE + atmospheric_insulation
    }

    fn thalassogenesis (temperature: u16, pressure: u32) -> Ocean {
        if temperature >= 273 && temperature <= 373 && pressure > 0 { Ocean::Water }
        else if temperature >= 196 && temperature <= 240 && pressure > 0 { Ocean::Ammonia }
        else { Ocean::None }
    }

    pub fn get_info(&self) -> String{
        format!("This is a {} planet, which is {} AU from its star, \
        temperature: {} K, pressure: {}, mass: {}, magnetic field: {}, \
         and it's {} that there is life here.", &self.class, &self.distance_from_star,
                &self.temperature, &self.pressure, &self.mass,
                &self.magnetic_field, &self.habitable)
    }
}