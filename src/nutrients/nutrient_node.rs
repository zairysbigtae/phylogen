use raylib::math::Vector2;

use super::Nutrients;

pub struct NutrientNode {
    pub nutrients: Nutrients,

    pub pos: Vector2,
    pub size: Vector2,
    pub consumed: bool,
}

impl NutrientNode {
    pub fn new(pos: Vector2) -> Self {
        let nutrients = Nutrients::new();
        let size = (nutrients.glucose / nutrients.max_glucose) + (nutrients.lipids / nutrients.max_lipids);

        Self {
            nutrients,

            pos,
            size: Vector2::new(size, size),
            consumed: false,
        }
    }
}
