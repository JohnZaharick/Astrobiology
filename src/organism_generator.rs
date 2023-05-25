use strum_macros::Display;
use rand::prelude::*;
use rand::distributions::{Alphanumeric, DistString};

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
    Siloxane,
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
    name: String,
    size: Size,
    organization: Organization,
    symmetry: Symmetry,
    structure: Structure,
    solvent: Solvent,
    metabolism: Metabolism,
}

impl Organism {
    pub fn new (planet: &planet_generator::Planet, seed: u64) -> Organism {

        let mut rng = StdRng::seed_from_u64(seed);

        let solvent = match planet.get_ocean() {
            Ocean::Water => { Solvent::Water }
            Ocean::Ammonia => { Solvent::Ammonia }
            Ocean::None => {
                panic!("Should not have been able to enter organism_generator without an ocean!") }
        };

        Organism {
            name: Alphanumeric.sample_string(&mut rng, 5),
            size: Size::SingleCell,
            organization: Organization::Modular,
            symmetry: Symmetry::Asymmetrical,
            structure: Structure::CarbonHydrogen,
            solvent,
            metabolism: Metabolism::Anaerobic,
        }
    }

    pub fn get_info(&self) -> String{
        format!("This organism is barcoded: {}. It is {} and {}, has {} symmetry, is made of {}, uses {} for a solvent, \
        and is {}.", &self.name, &self.size, &self.organization,
                &self.symmetry, &self.structure, &self.solvent,
                &self.metabolism)
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_size(&self) -> &Size {
        &self.size
    }

    pub fn get_organization(&self) -> &Organization {
        &self.organization
    }

    pub fn get_symmetry(&self) -> &Symmetry {
        &self.symmetry
    }

    pub fn get_structure(&self) -> &Structure {
        &self.structure
    }

    pub fn get_solvent(&self) -> &Solvent {
        &self.solvent
    }

    pub fn get_metabolism(&self) -> &Metabolism {
        &self.metabolism
    }
}