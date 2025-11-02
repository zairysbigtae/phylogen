use raylib::{color::Color, math::Vector2, prelude::{RaylibDraw, RaylibDrawHandle}};

use super::{HasNutrients, Nutrients};

#[derive(Debug, Clone, Copy)]
pub struct NutrientNode {
    pub nutrients: Nutrients,

    pub pos: Vector2,
    pub size: Vector2,
    pub consumed: bool,
}

impl HasNutrients for NutrientNode {
    fn get_pos(&self) -> Vector2 {
        self.pos
    }

    fn get_size(&self) -> Vector2 {
        self.size
    }

    fn get_nutrients(&self) -> Nutrients {
        self.nutrients
    }

    fn get_consumed(&mut self) {
        self.consumed = true;
    }
}

impl NutrientNode {
    pub fn new(pos: Vector2) -> Self {
        let nutrients = Nutrients::new();
        let size = (nutrients.glucose / nutrients.max_glucose) + (nutrients.lipids / nutrients.max_lipids);

        Self {
            nutrients,

            pos,
            size: Vector2::new(size, size) * 3.0,
            consumed: false,
        }
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        d.draw_rectangle_v(self.pos, self.size, Color::WHITE);
    }
}
