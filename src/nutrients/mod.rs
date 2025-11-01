pub mod nutrient_node;

use rand::random_range;

#[derive(Default, Debug, Clone, Copy)]
pub struct Nutrients {
    pub glucose: f32,
    pub amino_acids: f32,
    pub lipids: f32,
    pub max_glucose: f32,
    pub max_amino_acids: f32,
    pub max_lipids: f32,
}

impl Nutrients {
    pub fn new() -> Self {
        Self {
            glucose: random_range(0.0..3.0),
            amino_acids: random_range(0.0..2.0),
            lipids: random_range(0.0..1.0),
            max_glucose: random_range(5.0..10.0),
            max_amino_acids: random_range(5.0..6.0),
            max_lipids: random_range(3.0..5.0),
        }
    }
}
