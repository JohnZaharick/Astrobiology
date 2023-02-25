use strum_macros::Display;
use rand::prelude::*;
use rand::distributions::WeightedIndex;

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

pub struct Star {
    pub class: StarClass,
    pub mass: u8,
    pub age: u16,
}

impl Star {
    pub fn new () -> Star {
        let mut rng = rand::thread_rng();

        Star {
            class: Self::chose_star_class(),
            mass: rng.gen_range(STAR_MASS_MINIMUM..STAR_MASS_MAXIMUM),
            age: rng.gen_range(STAR_AGE_MINIMUM..STAR_AGE_MAXIMUM),
        }
    }

    fn chose_star_class() -> StarClass {
        let abundance = vec![0.8, 0.0828, 0.035, 0.02, 0.007,
                             0.001, 0.0001, 0.05, 0.004, 0.0001];
        let random_index = Self::generate_weighted_random_number(&abundance);
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

    fn generate_weighted_random_number(weights: &[f64]) -> usize {
        let dist = WeightedIndex::new(weights).unwrap();
        let mut rng = thread_rng();
        dist.sample(&mut rng)
    }
}