use strum_macros::Display;
use rand::prelude::*;
use rand::distributions::WeightedIndex;

const PLANET_NUMBER_MINIMUM: u8 = 3;
const PLANET_NUMBER_MAXIMUM: u8 = 12;

// In millions of years
const STAR_AGE_MINIMUM: u16 = 1;
const STAR_AGE_MAXIMUM: u16 = 13000;

// In stellar mass (M☉)
const STAR_MASS_MINIMUM: u8 = 1;
const STAR_MASS_MAXIMUM: u8 = 150;

#[derive(Display)]
enum StarClass {
    O, B, A, F, G, K, M,
    WhiteDwarf,
    Giant,
    SuperGiant,
}

struct Star {
    class: StarClass,
    mass: u8,
    age: u16,
}

struct PlanetarySystem {
    star: Star,
    planets: u8,
}

impl PlanetarySystem {

    fn new() -> Self {

        let mut rng = rand::thread_rng();

        PlanetarySystem {
            star: Star {
                class: Self::chose_star_class(),
                mass: rng.gen_range(STAR_MASS_MINIMUM..STAR_MASS_MAXIMUM),
                age: rng.gen_range(STAR_AGE_MINIMUM..STAR_AGE_MAXIMUM),
            },
            planets: rng.gen_range(PLANET_NUMBER_MINIMUM..PLANET_NUMBER_MAXIMUM),
        }
    }

    fn chose_star_class() -> StarClass {
        let abundance = vec![0.8, 0.0828, 0.035, 0.02, 0.007,
                             0.001, 0.0001, 0.05, 0.004, 0.0001];
        let random_index = generate_weighted_random_number(&abundance);
        match random_index {
            0 => StarClass::M,
            1 => StarClass::K,
            2 => StarClass::G,
            3 => StarClass::F,
            4 => StarClass::A,
            5 => StarClass::B,
            6 => StarClass::O,
            7 => StarClass::WhiteDwarf,
            8 => StarClass::Giant,
            9 => StarClass::SuperGiant,
            _ => panic!("abundance[] has more indices than there are star classes!")
        }
    }
}

fn generate_weighted_random_number(weights: &[f64]) -> usize {
    let dist = WeightedIndex::new(weights).unwrap();
    let mut rng = thread_rng();
    dist.sample(&mut rng)
}

fn main() {
    let solar_system = PlanetarySystem::new();
    println!("There is an {} star that is {} million years old and weighs {} M☉. \
    It has {} planets.", solar_system.star.class, solar_system.star.age,
             solar_system.star.mass, solar_system.planets);
}