pub mod nutrient_node;

use std::ops::AddAssign;

use rand::random_range;
use raylib::math::Vector2;

pub trait HasNutrients {
    fn get_pos(&self) -> Vector2;
    fn get_size(&self) -> Vector2;
    fn get_nutrients(&self) -> Nutrients;
    fn consume(&mut self);
}

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
            glucose: random_range(0.0..40.0),
            amino_acids: random_range(10.0..200.0),
            lipids: random_range(10.0..100.0),
            max_glucose: random_range(150.0..500.0),
            max_amino_acids: random_range(5.0..500.0),
            max_lipids: random_range(150.0..500.0),
        }
    }
}

impl AddAssign for Nutrients {
    fn add_assign(&mut self, rhs: Self) {
        self.glucose = self.glucose + rhs.glucose;
        self.amino_acids = self.amino_acids + rhs.amino_acids;
        self.lipids = self.lipids + rhs.lipids;
    }
}
