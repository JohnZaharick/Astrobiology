enum Size {
    SingleCell,
    MultiCellular,
}

enum Organization {
    Modular,
    Unitary,
}

enum Symmetry {
    Asymmetrical,
    Spherical,
    Radial,
    Bilateral,
}

enum Solvent {
    Water,
    Ammonia,
}

enum Structure {
    CarbonHydrogen,
    Oxocarbon,
    Silicone,
}

enum Metabolism {
    Aerobic,
    Anaerobic,
}

pub struct Organism {
    size: Size,
    organization: Organization,
    symmetry: Symmetry,
    solvent: Solvent,
    structure: Structure,
    metabolism: Metabolism,
}

impl Organism {
    pub fn new () -> Organism {
        Organism {
            size: Size::SingleCell,
            organization: Organization::Modular,
            symmetry: Symmetry::Asymmetrical,
            solvent: Solvent::Water,
            structure: Structure::CarbonHydrogen,
            metabolism: Metabolism::Anaerobic,
        }
    }
}