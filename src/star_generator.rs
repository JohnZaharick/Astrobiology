use strum_macros::Display;
use rand::prelude::*;
use rand::distributions::WeightedIndex;
use rand_derive2::RandGen;
use lazy_static::lazy_static;

// TODO: Make unique age and mass ranges for each StarClass.

// In millions of years
const STAR_AGE_MINIMUM: u16 = 1;
const STAR_AGE_MAXIMUM: u16 = 13000;

// In stellar mass (M☉)
const STAR_MASS_MINIMUM: u8 = 1;
const STAR_MASS_MAXIMUM: u8 = 150;

const M_STAR_ABUNDANCE: f64 = 0.8;
const K_STAR_ABUNDANCE: f64 = 0.0828;
const G_STAR_ABUNDANCE: f64 = 0.035;
const F_STAR_ABUNDANCE: f64 = 0.02;
const A_STAR_ABUNDANCE: f64 = 0.007;
const B_STAR_ABUNDANCE: f64 = 0.001;
const O_STAR_ABUNDANCE: f64 = 0.0001;
const WHITE_DWARF_STAR_ABUNDANCE: f64 = 0.05;
const GIANT_STAR_ABUNDANCE: f64 = 0.004;
const SUPER_GIANT_STAR_ABUNDANCE: f64 = 0.0001;

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

lazy_static! {
    static ref STARS: Vec<(StarClass,f64)> = vec![
            (StarClass::M, M_STAR_ABUNDANCE),
            (StarClass::K, K_STAR_ABUNDANCE),
            (StarClass::G, G_STAR_ABUNDANCE),
            (StarClass::F, F_STAR_ABUNDANCE),
            (StarClass::A, A_STAR_ABUNDANCE),
            (StarClass::B, B_STAR_ABUNDANCE),
            (StarClass::O, O_STAR_ABUNDANCE),
            (StarClass::WhiteDwarf, WHITE_DWARF_STAR_ABUNDANCE),
            (StarClass::Giant, GIANT_STAR_ABUNDANCE),
            (StarClass::SuperGiant, SUPER_GIANT_STAR_ABUNDANCE),
            ];

    static ref ABUNDANCE: Vec<f64> = STARS.iter().map(|x| x.1).collect();
    }

impl Star {
    pub fn new() -> Star {
        let mut rng = rand::thread_rng();

        let random_index = Self::generate_weighted_random_number(&ABUNDANCE);
        match STARS[random_index].0 {
            StarClass::M =>         Star {
                class: StarClass::M,
                color: StarColor::Red,
                mass: rng.gen_range(STAR_MASS_MINIMUM..STAR_MASS_MAXIMUM),
                age: rng.gen_range(STAR_AGE_MINIMUM..STAR_AGE_MAXIMUM),
            },
            StarClass::K =>         Star {
                class: StarClass::K,
                color: StarColor::Orange,
                mass: rng.gen_range(STAR_MASS_MINIMUM..STAR_MASS_MAXIMUM),
                age: rng.gen_range(STAR_AGE_MINIMUM..STAR_AGE_MAXIMUM),
            },
            StarClass::G => Star {
                class: StarClass::G,
                color: StarColor::Yellow,
                mass: rng.gen_range(STAR_MASS_MINIMUM..STAR_MASS_MAXIMUM),
                age: rng.gen_range(STAR_AGE_MINIMUM..STAR_AGE_MAXIMUM),
            },
            StarClass::F => Star {
                class: StarClass::F,
                color: StarColor::White,
                mass: rng.gen_range(STAR_MASS_MINIMUM..STAR_MASS_MAXIMUM),
                age: rng.gen_range(STAR_AGE_MINIMUM..STAR_AGE_MAXIMUM),
            },
            StarClass::A => Star {
                class: StarClass::A,
                color: StarColor::White,
                mass: rng.gen_range(STAR_MASS_MINIMUM..STAR_MASS_MAXIMUM),
                age: rng.gen_range(STAR_AGE_MINIMUM..STAR_AGE_MAXIMUM),
            },
            StarClass::B => Star {
                class: StarClass::B,
                color: StarColor::Blue,
                mass: rng.gen_range(STAR_MASS_MINIMUM..STAR_MASS_MAXIMUM),
                age: rng.gen_range(STAR_AGE_MINIMUM..STAR_AGE_MAXIMUM),
            },
            StarClass::O => Star {
                class: StarClass::O,
                color: StarColor::Blue,
                mass: rng.gen_range(STAR_MASS_MINIMUM..STAR_MASS_MAXIMUM),
                age: rng.gen_range(STAR_AGE_MINIMUM..STAR_AGE_MAXIMUM),
            },
            StarClass::WhiteDwarf => Star {
                class: StarClass::WhiteDwarf,
                color: StarColor::White,
                mass: rng.gen_range(STAR_MASS_MINIMUM..STAR_MASS_MAXIMUM),
                age: rng.gen_range(STAR_AGE_MINIMUM..STAR_AGE_MAXIMUM),
            },
            StarClass::Giant => Star {
                class: StarClass::Giant,
                color: random(),
                mass: rng.gen_range(STAR_MASS_MINIMUM..STAR_MASS_MAXIMUM),
                age: rng.gen_range(STAR_AGE_MINIMUM..STAR_AGE_MAXIMUM),
            },
            StarClass::SuperGiant => Star {
                class: StarClass::SuperGiant,
                color: random(),
                mass: rng.gen_range(STAR_MASS_MINIMUM..STAR_MASS_MAXIMUM),
                age: rng.gen_range(STAR_AGE_MINIMUM..STAR_AGE_MAXIMUM),
            },
        }
    }

    fn generate_weighted_random_number(weights: &[f64]) -> usize {
        let dist = WeightedIndex::new(weights).unwrap();
        let mut rng = thread_rng();
        dist.sample(&mut rng)
    }
}