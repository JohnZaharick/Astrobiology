use strum_macros::Display;
use rand::prelude::*;
use rand::distributions::WeightedIndex;
use rand_derive2::RandGen;

// TODO: Make unique age and mass ranges for each StarClass.

// In millions of years
const STAR_AGE_MINIMUM: u16 = 1;
const STAR_AGE_MAXIMUM: u16 = 13000;

// In stellar mass (M☉)
const STAR_MASS_MINIMUM: u8 = 1;
const STAR_MASS_MAXIMUM: u8 = 150;

#[derive(Display)]
pub enum StarClass {
    O, B, A, F, G, K, M,
    WhiteDwarf,
    Giant,
    SuperGiant,
}

#[derive(Display, RandGen)]
pub enum StarColor {
    Red, Yellow, Orange, Blue, White
}

pub struct Star {
    pub class: StarClass,
    pub color: StarColor,
    pub mass: u8,
    pub age: u16,
}

impl Star {
    pub fn new() -> Star {
        let mut rng = rand::thread_rng();
        let abundance = vec![0.8, 0.0828, 0.035, 0.02, 0.007,
                             0.001, 0.0001, 0.05, 0.004, 0.0001];
        let random_index = Self::generate_weighted_random_number(&abundance);
        match random_index {
            0 =>         Star {
                class: StarClass::M,
                color: StarColor::Red,
                mass: rng.gen_range(STAR_MASS_MINIMUM..STAR_MASS_MAXIMUM),
                age: rng.gen_range(STAR_AGE_MINIMUM..STAR_AGE_MAXIMUM),
            },
            1 =>         Star {
                class: StarClass::K,
                color: StarColor::Orange,
                mass: rng.gen_range(STAR_MASS_MINIMUM..STAR_MASS_MAXIMUM),
                age: rng.gen_range(STAR_AGE_MINIMUM..STAR_AGE_MAXIMUM),
            },
            2 => Star {
                class: StarClass::G,
                color: StarColor::Yellow,
                mass: rng.gen_range(STAR_MASS_MINIMUM..STAR_MASS_MAXIMUM),
                age: rng.gen_range(STAR_AGE_MINIMUM..STAR_AGE_MAXIMUM),
            },
            3 => Star {
                class: StarClass::F,
                color: StarColor::White,
                mass: rng.gen_range(STAR_MASS_MINIMUM..STAR_MASS_MAXIMUM),
                age: rng.gen_range(STAR_AGE_MINIMUM..STAR_AGE_MAXIMUM),
            },
            4 => Star {
                class: StarClass::A,
                color: StarColor::White,
                mass: rng.gen_range(STAR_MASS_MINIMUM..STAR_MASS_MAXIMUM),
                age: rng.gen_range(STAR_AGE_MINIMUM..STAR_AGE_MAXIMUM),
            },
            5 => Star {
                class: StarClass::B,
                color: StarColor::Blue,
                mass: rng.gen_range(STAR_MASS_MINIMUM..STAR_MASS_MAXIMUM),
                age: rng.gen_range(STAR_AGE_MINIMUM..STAR_AGE_MAXIMUM),
            },
            6 => Star {
                class: StarClass::O,
                color: StarColor::Blue,
                mass: rng.gen_range(STAR_MASS_MINIMUM..STAR_MASS_MAXIMUM),
                age: rng.gen_range(STAR_AGE_MINIMUM..STAR_AGE_MAXIMUM),
            },
            7 => Star {
                class: StarClass::WhiteDwarf,
                color: StarColor::White,
                mass: rng.gen_range(STAR_MASS_MINIMUM..STAR_MASS_MAXIMUM),
                age: rng.gen_range(STAR_AGE_MINIMUM..STAR_AGE_MAXIMUM),
            },
            8 => Star {
                class: StarClass::Giant,
                color: random(),
                mass: rng.gen_range(STAR_MASS_MINIMUM..STAR_MASS_MAXIMUM),
                age: rng.gen_range(STAR_AGE_MINIMUM..STAR_AGE_MAXIMUM),
            },
            9 => Star {
                class: StarClass::SuperGiant,
                color: random(),
                mass: rng.gen_range(STAR_MASS_MINIMUM..STAR_MASS_MAXIMUM),
                age: rng.gen_range(STAR_AGE_MINIMUM..STAR_AGE_MAXIMUM),
            },
            _ => panic!("abundance[] has more indices than there are star classes!")
        }
    }

    fn generate_weighted_random_number(weights: &[f64]) -> usize {
        let dist = WeightedIndex::new(weights).unwrap();
        let mut rng = thread_rng();
        dist.sample(&mut rng)
    }
}