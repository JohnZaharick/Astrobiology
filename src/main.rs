use strum_macros::Display;
use rand::prelude::*;
use rand::distributions::WeightedIndex;

#[derive(Display)]
enum StarClass {
    O, B, A, F, G, K, M,
    WhiteDwarf,
    Giant,
    SuperGiant,
}

struct PlanetarySystem {
    star: StarClass,
    planets: i8,
}

impl PlanetarySystem {
    fn new() -> Self {
        PlanetarySystem {
            star: Self::chose_star_class(),
            planets: 8,
        }
    }

    fn chose_star_class() -> StarClass {
        let abundance = vec![0.8, 0.0828, 0.035, 0.02, 0.007, 0.001, 0.0001, 0.05, 0.004, 0.0001];
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
    println!("There is an {} star that has {} planets.", solar_system.star, solar_system.planets);
}