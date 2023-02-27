use strum_macros::Display;
use rand::prelude::*;
use rand::distributions::WeightedIndex;
use rand_derive2::RandGen;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::hash::Hash;

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

// TODO: Remove and redesign Giant and SuperGiant, as they aren't classes and are found in
// every class.
#[derive(Display, Hash, Eq, PartialEq)]
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
    pub temperature: u16,
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

    static ref STAR_TEMPERATURES: HashMap<StarClass, Vec<u16>> = HashMap::from([
        (StarClass::M, vec![3600, 3400, 3200, 3100, 2900, 2700, 2600, 2400, 2200, 2000]),
        (StarClass::K, vec![4500, 4400, 4300, 4200, 4100, 4000, 3900, 3800, 3800, 3700]),
        (StarClass::G, vec![5800, 5600, 5500, 5200, 5100, 5000, 5000, 5000, 4900, 4600]),
        (StarClass::F, vec![7700, 7600, 7400, 7200, 7000, 6900, 6600, 6500, 6300, 6100]),
        (StarClass::A, vec![9900, 9700, 9500, 9200, 9000, 8700, 8600, 8300, 8100, 7900]),
        (StarClass::B, vec![26500, 24900, 23500, 22000, 20500, 19000, 17100, 15300, 13500, 11700]),
        (StarClass::O, vec![50000, 47700, 45400, 43100, 40800, 39000, 36100, 33700, 31300, 28900]),
    ]);
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
                age: rng.gen_range(LOW_MASS_STAR_AGE_MINIMUM..=M_STAR_AGE_MAXIMUM),
                temperature: STAR_TEMPERATURES[&StarClass::M][rng.gen_range(0..=9)],
            },
            StarClass::K =>         Star {
                class: StarClass::K,
                color: StarColor::Orange,
                mass: Self::generate_star_mass(K_STAR_MASS_MINIMUM,
                                               K_STAR_MASS_MAXIMUM, &mut rng),
                age: rng.gen_range(LOW_MASS_STAR_AGE_MINIMUM..=K_STAR_AGE_MAXIMUM),
                temperature: STAR_TEMPERATURES[&StarClass::K][rng.gen_range(0..=9)],
            },
            StarClass::G => Star {
                class: StarClass::G,
                color: StarColor::Yellow,
                mass: Self::generate_star_mass(G_STAR_MASS_MINIMUM,
                                               G_STAR_MASS_MAXIMUM, &mut rng),
                age: rng.gen_range(LOW_MASS_STAR_AGE_MINIMUM..=G_STAR_AGE_MAXIMUM),
                temperature: STAR_TEMPERATURES[&StarClass::G][rng.gen_range(0..=9)],
            },
            StarClass::F => Star {
                class: StarClass::F,
                color: StarColor::White,
                mass: Self::generate_star_mass(F_STAR_MASS_MINIMUM,
                                               F_STAR_MASS_MAXIMUM, &mut rng),
                age: rng.gen_range(LOW_MASS_STAR_AGE_MINIMUM..=F_STAR_AGE_MAXIMUM),
                temperature: STAR_TEMPERATURES[&StarClass::F][rng.gen_range(0..=9)],
            },
            StarClass::A => Star {
                class: StarClass::A,
                color: StarColor::White,
                mass: Self::generate_star_mass(A_STAR_MASS_MINIMUM,
                                               A_STAR_MASS_MAXIMUM, &mut rng),
                age: rng.gen_range(HIGH_MASS_STAR_AGE_MINIMUM..=A_STAR_AGE_MAXIMUM),
                temperature: STAR_TEMPERATURES[&StarClass::A][rng.gen_range(0..=9)],
            },
            StarClass::B => Star {
                class: StarClass::B,
                color: StarColor::Blue,
                mass: Self::generate_star_mass(B_STAR_MASS_MINIMUM,
                                               B_STAR_MASS_MAXIMUM, &mut rng),
                age: rng.gen_range(HIGH_MASS_STAR_AGE_MINIMUM..=B_STAR_AGE_MAXIMUM),
                temperature: STAR_TEMPERATURES[&StarClass::B][rng.gen_range(0..=9)],
            },
            StarClass::O => Star {
                class: StarClass::O,
                color: StarColor::Blue,
                mass: Self::generate_star_mass(O_STAR_MASS_MINIMUM,
                                               O_STAR_MASS_MAXIMUM, &mut rng),
                age: rng.gen_range(HIGH_MASS_STAR_AGE_MINIMUM..=O_STAR_AGE_MAXIMUM),
                temperature: STAR_TEMPERATURES[&StarClass::O][rng.gen_range(0..=9)],
            },
            StarClass::WhiteDwarf => Star {
                class: StarClass::WhiteDwarf,
                color: StarColor::White,
                mass: Self::generate_star_mass(WHITE_DWARF_STAR_MASS_MINIMUM,
                                               WHITE_DWARF_STAR_MASS_MAXIMUM, &mut rng),
                age: rng.gen_range(WHITE_DWARF_STAR_AGE_MINIMUM..=
                    WHITE_DWARF_STAR_AGE_MAXIMUM),
                temperature: 10000,
            },
            StarClass::Giant => Star {
                class: StarClass::Giant,
                color: random(),
                mass: Self::generate_star_mass(GIANT_STAR_MASS_MINIMUM,
                                               GIANT_STAR_MASS_MAXIMUM, &mut rng),
                age: rng.gen_range(GIANT_STAR_AGE_MINIMUM..=
                    GIANT_STAR_AGE_MAXIMUM),
                temperature: 4500,
            },
            StarClass::SuperGiant => Star {
                class: StarClass::SuperGiant,
                color: random(),
                mass: Self::generate_star_mass(SUPER_GIANT_STAR_MASS_MINIMUM,
                                               SUPER_GIANT_STAR_MASS_MAXIMUM, &mut rng),
                age: rng.gen_range(SUPER_GIANT_STAR_AGE_MINIMUM..=
                    SUPER_GIANT_STAR_AGE_MAXIMUM),
                temperature: STAR_TEMPERATURES[&StarClass::B][rng.gen_range(0..=9)],
                //Although supergiants exist in every class from O to M, the majority are
                // spectral type B, more than at all other spectral classes combined.
            },
        }
    }

    fn generate_weighted_random_number(weights: &[f64], rng: &mut impl Rng) -> usize {
        let dist = WeightedIndex::new(weights).unwrap();
        dist.sample(rng)
    }

    fn generate_star_mass(min: f64, max: f64, rng: &mut impl Rng) -> f64 {
        (rng.gen_range(min..=max) * 100.0).round() / 100.0
    }
}