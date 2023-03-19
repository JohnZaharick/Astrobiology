use strum_macros::Display;

use crate::planet_generator;
use crate::planet_generator::Ocean;

#[derive(Display)]
pub enum Size {
    SingleCell,
    MultiCellular,
}

#[derive(Display)]
pub enum Organization {
    Modular,
    Unitary,
}

#[derive(Display)]
pub enum Symmetry {
    Asymmetrical,
    Spherical,
    Radial,
    Bilateral,
}

// Terrestrial life is made of C-H chains
// C-O chains will form in environments poor in H
// S-O chains will form in high temperatures that prevent C chains from forming
#[derive(Display)]
pub enum Structure {
    CarbonHydrogen,
    Oxocarbon,
    Silicone,
}

#[derive(Display)]
pub enum Solvent {
    Water,
    Ammonia,
}

#[derive(Display)]
pub enum Metabolism {
    Aerobic,
    Anaerobic,
}

pub struct Organism {
    pub size: Size,
    pub organization: Organization,
    pub symmetry: Symmetry,
    pub structure: Structure,
    pub solvent: Solvent,
    pub metabolism: Metabolism,
}

impl Organism {
    pub fn new (planet: &planet_generator::Planet) -> Organism {

        let solvent = match planet.ocean {
            Ocean::Water => { Solvent::Water }
            Ocean::Ammonia => { Solvent::Ammonia }
            Ocean::None => {
                panic!("Should not have been able to enter organism_generator without an ocean!") }
        };

        Organism {
            size: Size::SingleCell,
            organization: Organization::Modular,
            symmetry: Symmetry::Asymmetrical,
            structure: Structure::CarbonHydrogen,
            solvent,
            metabolism: Metabolism::Anaerobic,
        }
    }

    pub fn get_info(&self) -> String{
        format!("This organism is {} and {}, has {} symmetry, is made of {}, uses {} for a solvent,\
         and is {}.", &self.size, &self.organization,
                &self.symmetry, &self.structure, &self.solvent,
                &self.metabolism)
    }
}