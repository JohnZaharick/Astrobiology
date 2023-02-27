use strum_macros::Display;
use rand::prelude::*;
use rand::distributions::WeightedIndex;
use rand_derive2::RandGen;
use lazy_static::lazy_static;

// TODO: Adjust for exclusive max value in rng range.

// In millions of years
const LOW_MASS_STAR_AGE_MINIMUM: u16 = 10;
const HIGH_MASS_STAR_AGE_MINIMUM: u16 = 1;
const WHITE_DWARF_STAR_AGE_MINIMUM: u16 = 1520; // evolved from A Star and Giant Star
const GIANT_STAR_AGE_MINIMUM: u16 = 400; // evolved from A Star
const SUPER_GIANT_STAR_AGE_MINIMUM: u16 = 8; // evolved from O Star

// M stars have a maximum age of 10 trillion years
// K stars have a maximum age of 15 billion years
// The simulated universe is only 13 billion years old, however.
const M_STAR_AGE_MAXIMUM: u16 = 13000;
const K_STAR_AGE_MAXIMUM: u16 = 13000;
const G_STAR_AGE_MAXIMUM: u16 = 10000;
const F_STAR_AGE_MAXIMUM: u16 = 5000;
const A_STAR_AGE_MAXIMUM: u16 = 400;
const B_STAR_AGE_MAXIMUM: u16 = 100;
const O_STAR_AGE_MAXIMUM: u16 = 8;
const WHITE_DWARF_STAR_AGE_MAXIMUM: u16 = 13000;
const GIANT_STAR_AGE_MAXIMUM: u16 = 13000;
const SUPER_GIANT_STAR_AGE_MAXIMUM: u16 = 150; // 50 MY lifespan + B star lifespan

// In stellar mass (M☉)
const M_STAR_MASS_MINIMUM: f64 = 0.1;
const M_STAR_MASS_MAXIMUM: f64 = 0.45;
const K_STAR_MASS_MINIMUM: f64 = 0.45;
const K_STAR_MASS_MAXIMUM: f64 = 0.8;
const G_STAR_MASS_MINIMUM: f64 = 0.8;
const G_STAR_MASS_MAXIMUM: f64 = 1.0;
const F_STAR_MASS_MINIMUM: f64 = 1.0;
const F_STAR_MASS_MAXIMUM: f64 = 1.4;
const A_STAR_MASS_MINIMUM: f64 = 1.4;
const A_STAR_MASS_MAXIMUM: f64 = 2.0;
const B_STAR_MASS_MINIMUM: f64 = 2.1;
const B_STAR_MASS_MAXIMUM: f64 = 15.0;
const O_STAR_MASS_MINIMUM: f64 = 15.0;
const O_STAR_MASS_MAXIMUM: f64 = 315.0;
const WHITE_DWARF_STAR_MASS_MINIMUM: f64 = 0.5;
const WHITE_DWARF_STAR_MASS_MAXIMUM: f64 = 0.7;
const GIANT_STAR_MASS_MINIMUM: f64 = 0.8;
const GIANT_STAR_MASS_MAXIMUM: f64 = 2.0;
const SUPER_GIANT_STAR_MASS_MINIMUM: f64 = 11.0;
const SUPER_GIANT_STAR_MASS_MAXIMUM: f64 = 315.0;

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
    pub mass: f64,
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
        let random_index = Self::generate_weighted_random_number(&ABUNDANCE, &mut rng);

        match STARS[random_index].0 {
            StarClass::M =>         Star {
                class: StarClass::M,
                color: StarColor::Red,
                mass: Self::generate_star_mass(M_STAR_MASS_MINIMUM,
                                               M_STAR_MASS_MAXIMUM, &mut rng),
                age: rng.gen_range(LOW_MASS_STAR_AGE_MINIMUM..M_STAR_AGE_MAXIMUM),
            },
            StarClass::K =>         Star {
                class: StarClass::K,
                color: StarColor::Orange,
                mass: Self::generate_star_mass(K_STAR_MASS_MINIMUM,
                                               K_STAR_MASS_MAXIMUM, &mut rng),
                age: rng.gen_range(LOW_MASS_STAR_AGE_MINIMUM..K_STAR_AGE_MAXIMUM),
            },
            StarClass::G => Star {
                class: StarClass::G,
                color: StarColor::Yellow,
                mass: Self::generate_star_mass(G_STAR_MASS_MINIMUM,
                                               G_STAR_MASS_MAXIMUM, &mut rng),
                age: rng.gen_range(LOW_MASS_STAR_AGE_MINIMUM..G_STAR_AGE_MAXIMUM),
            },
            StarClass::F => Star {
                class: StarClass::F,
                color: StarColor::White,
                mass: Self::generate_star_mass(F_STAR_MASS_MINIMUM,
                                               F_STAR_MASS_MAXIMUM, &mut rng),
                age: rng.gen_range(LOW_MASS_STAR_AGE_MINIMUM..F_STAR_AGE_MAXIMUM),
            },
            StarClass::A => Star {
                class: StarClass::A,
                color: StarColor::White,
                mass: Self::generate_star_mass(A_STAR_MASS_MINIMUM,
                                               A_STAR_MASS_MAXIMUM, &mut rng),
                age: rng.gen_range(HIGH_MASS_STAR_AGE_MINIMUM..A_STAR_AGE_MAXIMUM),
            },
            StarClass::B => Star {
                class: StarClass::B,
                color: StarColor::Blue,
                mass: Self::generate_star_mass(B_STAR_MASS_MINIMUM,
                                               B_STAR_MASS_MAXIMUM, &mut rng),
                age: rng.gen_range(HIGH_MASS_STAR_AGE_MINIMUM..B_STAR_AGE_MAXIMUM),
            },
            StarClass::O => Star {
                class: StarClass::O,
                color: StarColor::Blue,
                mass: Self::generate_star_mass(O_STAR_MASS_MINIMUM,
                                               O_STAR_MASS_MAXIMUM, &mut rng),
                age: rng.gen_range(HIGH_MASS_STAR_AGE_MINIMUM..O_STAR_AGE_MAXIMUM),
            },
            StarClass::WhiteDwarf => Star {
                class: StarClass::WhiteDwarf,
                color: StarColor::White,
                mass: Self::generate_star_mass(WHITE_DWARF_STAR_MASS_MINIMUM,
                                               WHITE_DWARF_STAR_MASS_MAXIMUM, &mut rng),
                age: rng.gen_range(WHITE_DWARF_STAR_AGE_MINIMUM..
                    WHITE_DWARF_STAR_AGE_MAXIMUM),
            },
            StarClass::Giant => Star {
                class: StarClass::Giant,
                color: random(),
                mass: Self::generate_star_mass(GIANT_STAR_MASS_MINIMUM,
                                               GIANT_STAR_MASS_MAXIMUM, &mut rng),
                age: rng.gen_range(GIANT_STAR_AGE_MINIMUM..GIANT_STAR_AGE_MAXIMUM),
            },
            StarClass::SuperGiant => Star {
                class: StarClass::SuperGiant,
                color: random(),
                mass: Self::generate_star_mass(SUPER_GIANT_STAR_MASS_MINIMUM,
                                               SUPER_GIANT_STAR_MASS_MAXIMUM, &mut rng),
                age: rng.gen_range(SUPER_GIANT_STAR_AGE_MINIMUM..
                    SUPER_GIANT_STAR_AGE_MAXIMUM),
            },
        }
    }

    fn generate_weighted_random_number(weights: &[f64], rng: &mut impl Rng) -> usize {
        let dist = WeightedIndex::new(weights).unwrap();
        dist.sample(rng)
    }

    fn generate_star_mass(min: f64, max: f64, rng: &mut impl Rng) -> f64 {
        (rng.gen_range(min..max) * 100.0).round() / 100.0
    }
}