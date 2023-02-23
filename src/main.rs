use strum_macros::Display;

#[derive(Display)]
enum StarClass {
    O, B, A, F, G, K, M
}

struct PlanetarySystem {
    star: StarClass,
    planets: i8,
}

impl PlanetarySystem {
    fn new() -> Self {
        PlanetarySystem {
            star: StarClass::M,
            planets: 8,
        }
    }
}

fn main() {
    let solar_system = PlanetarySystem::new();
    println!("There is an {} star that has {} planets.", solar_system.star, solar_system.planets);
}