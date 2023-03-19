use strum_macros::Display;

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
    pub fn new () -> Organism {
        Organism {
            size: Size::SingleCell,
            organization: Organization::Modular,
            symmetry: Symmetry::Asymmetrical,
            structure: Structure::CarbonHydrogen,
            solvent: Solvent::Water,
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