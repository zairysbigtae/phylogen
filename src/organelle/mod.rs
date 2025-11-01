#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OrganelleKind {
    Nucleus,     // control center, holds DNA
    Mitochondria, // powerhouse
    Ribosome,    // protein builder factories
    Lysosome,    // garbage disposal lol
    Chloroplast, // light -> sugar
    Vacuole,     // stores water and nutrients. plants have big, animals have small
    Cytoplasm,   // goo, makes organelles float around
    Cytoskeleton,// give shapes and helps movements
    Centrioles,  // animal cells, helps with cell division
}

#[derive(Debug, Clone, Copy)]
pub struct Organelle {
    pub organelle: OrganelleKind,

    pub efficiency: f32, // depends on if the organelle is damaged
    pub activity_level: f32,
    pub integrity: f32, // general health
}

impl Default for Organelle {
    fn default() -> Self {
        Self {
            organelle: OrganelleKind::Mitochondria,
            efficiency: 0.99,
            activity_level: 1.0,
            integrity: 1.0,
        }
    }
}

impl Organelle {
    pub fn new(organelle_kind: OrganelleKind) -> Self {
        Self {
            organelle: organelle_kind,
            efficiency: 0.99,
            activity_level: 1.0,
            integrity: 1.0,
        }
    }

}
