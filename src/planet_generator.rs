use rand::prelude::*;
use strum_macros::Display;
use strum_macros::EnumIter;

use crate::star_generator;

const BACKGROUND_TEMPERATURE: u16 = 3; // in kelvins; prevents absolute zero worlds
const ATMOSPHERIC_INSULATION: u16 = 150; // in kelvins
const DISTANCE_FROM_STAR_MODIFIER: u16 = 4; // for calibrating how much temperature drops with distance from a star
const MINIMUM_STAR_AGE_FOR_LIFE: u16 = 1000; // in millions of years
const MINIMUM_MASS_FOR_MAGNETOSPHERE: u32 = 5000; // mass currently has no units and is arbitrary
const MINIMUM_MASS_FOR_ATMOSPHERE: u32 = 1000;

#[derive(Display)]
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
    pub mass: u32,
    pub magnetic_field: bool,
    pub pressure: u32,
    pub temperature: u16,
    pub ocean: Ocean,
    pub habitable: bool,
}

impl Planet {
    pub fn new(star: &star_generator::Star, distance: u8) -> Planet {

        let seed: u64 = star.age as u64 * distance as u64;
        let mut rng = StdRng::seed_from_u64(seed);

        // let planet_class = rng.gen_range(0..planet_classes.len());
        let size = Self::calculate_mass_and_class(&mut rng, distance);

        let magnetic_field = if size.0 >= MINIMUM_MASS_FOR_MAGNETOSPHERE { true } else { false };
        // TODO: pressure needs to be influenced by mass; small planets can't have high pressures; large planets can't have low pressures
        let pressure = Self::calculate_pressure(&mut rng, size.0);
        let temperature = Self::calculate_temperature(distance, &star, pressure);
        let ocean = Self::thalassogenesis(temperature, pressure);
        let habitable = if ocean != Ocean::None
            && magnetic_field && star.age > MINIMUM_STAR_AGE_FOR_LIFE { true } else { false };

        Planet {
            class: size.1,
            distance_from_star: distance,
            mass: size.0,
            magnetic_field,
            pressure,
            temperature,
            ocean,
            habitable,
        }
    }

    fn calculate_mass_and_class (rng: &mut impl Rng, distance: u8) -> (u32, PlanetClass) {
        // TODO: Incorporate star age in calculations to allow Hot Jupiters
        let mut mass = rng.gen_range(100..10000);
        if distance < 6 {
            if mass <= 1000 {
                (mass, PlanetClass::Dwarf)
            } else {
                (mass, PlanetClass::Rocky)
            }
        } else {
            if mass <= 1000 {
                (mass, PlanetClass::Dwarf)
            } else {
                mass = rng.gen_range(100000..1000000);
                if mass <= 500000 {
                    (mass, PlanetClass::IceGiant)
                } else {
                    (mass, PlanetClass::GasGiant)
                }
            }
        }
    }

    fn calculate_pressure (rng: &mut impl Rng, mass: u32) -> u32 {
        if mass > MINIMUM_MASS_FOR_ATMOSPHERE {
            rng.gen_range(1..10) * 10_u32.pow(rng.gen_range(0..8))
        }
        else {
            0
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

// Notes:
// Fulton Gap - Dearth of planets between 1.5 and 2 Earth radii.
// Neptune is 17 times the mass of Earth; 3.9 Earth radii
// Sub-Neptunes are 1.7 to 3.9 Earth radii
