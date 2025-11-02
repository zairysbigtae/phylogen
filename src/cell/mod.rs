pub mod property;

use property::{CellProperties, BASE_RESPIRATION_RATE};
use raylib::prelude::*;

use crate::{nutrients::{nutrient_node::NutrientNode, HasNutrients, Nutrients}, organelle::{Organelle, OrganelleKind}, utils::rects_collide};

#[derive(Default)]
pub struct Cell {
    pub atp: f32,
    pub max_atp: f32,
    pub o2_level: f32,
    pub env_o2_level: f32,
    pub max_o2_level: f32,

    pub nutrients: Nutrients,

    pub properties: CellProperties,
    pub organelles: Vec<Organelle>,

    pub pos: Vector2,
    pub size: Vector2,
    pub velocity: Vector2,
    //pub mass: f32,

    pub nearest_food: Option<Box<dyn HasNutrients>>,
    pub nearest_food_dist_sq: f32,

    pub consumed: bool,
}

impl HasNutrients for Cell {
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

impl Cell {
    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        let color = if (self.atp / self.max_atp) <= 0.3  {
            Color::RED
        } else {
            Color::GREENYELLOW
        };

        // the uhh body?
        d.draw_rectangle_v(self.pos, self.size, color);

        // organelles hehe
        for organelle in &self.organelles {
            d.draw_ellipse(organelle.pos.x as i32, organelle.pos.y as i32, organelle.size.x, organelle.size.y, Color::SALMON);
        }
    }

    pub fn update(&mut self) {
        self.generate_atp();
        self.breathe();
        self.glide();

        self.atp = self.atp.max(0.0);
    }

    pub fn find_food<T: HasNutrients + Clone + 'static>(&mut self, food: T) {
        let dx = food.get_pos().x - self.pos.x;
        let dy = food.get_pos().y - self.pos.y;
        let food_dist_sq = dx*dx + dy*dy;

        if food_dist_sq < self.nearest_food_dist_sq {
            self.nearest_food_dist_sq = food_dist_sq;
            self.nearest_food = Some(Box::new(food));
        }

        if let Some(food) = &self.nearest_food {
            let dx = food.get_pos().x - self.pos.x;
            let dy = food.get_pos().y - self.pos.y;
            let food_dist = (dx*dx + dy*dy).sqrt();

            if food_dist > 0.0 {
                let dir_x = dx / food_dist;
                let dir_y = dy / food_dist;

                let speed = self.velocity.length();
                let stop_dist = 20.0;
                // let move_speed = if food_dist < stop_dist {
                //     speed * (speed + (food_dist / stop_dist))
                // } else {
                //     speed
                // };
                let move_speed = speed;

                self.velocity.x = dir_x * move_speed;
                self.velocity.y = dir_y * move_speed;
            }

            // println!("food");
        }
    }

    fn glide(&mut self) {
        self.pos += self.velocity;

        let base = 0.2; // base maintenance cost
        let speed = self.velocity.length();
        self.atp -= base * speed;
    }

    fn breathe(&mut self) {
        let env_o2_factor = self.env_o2_level / 21.0; // 0 - 1
        let intake = (self.max_o2_level - self.o2_level) * env_o2_factor * self.properties.respiration_rate;
        self.o2_level = (self.o2_level + intake).min(self.max_o2_level);
    }

    pub fn eat<T: HasNutrients>(&mut self, other: &mut T) {
        if rects_collide(self.pos.x, self.pos.y, self.size.x, self.size.y, other.get_pos().x, other.get_pos().y, other.get_size().x, other.get_size().y) {
            self.nutrients += other.get_nutrients();
            self.atp -= 2.0;

            self.nearest_food_dist_sq = f32::MAX;
            self.nearest_food = None;

            other.get_consumed();
        }
    }

    fn generate_atp(&mut self) {
        for organelle in &self.organelles {
            if organelle.organelle == OrganelleKind::Mitochondria {
                let burn_rate = 10.0 * (self.nutrients.glucose / self.nutrients.max_glucose).clamp(0.0, 1.0);
                let max_burn = self.properties.metabolism_rate * organelle.efficiency * organelle.activity_level * burn_rate;
                let glucose_used = self.nutrients.glucose.min(max_burn);

                if glucose_used <= 0.0
                || self.atp >= self.max_atp {
                    continue; // nothing to burn
                }

                let env_o2_factor = self.env_o2_level / 21.0; // range 0-1

                let min_o2_level = glucose_used * 6.0;
                let o2_used = self.o2_level.min(min_o2_level * env_o2_factor); // ~6 O2 per glucose
                self.o2_level -= o2_used;

                let respiration_boost = (o2_used / (min_o2_level * env_o2_factor)).powf(0.8);
                self.properties.respiration_rate = BASE_RESPIRATION_RATE * (0.5 + respiration_boost * 0.5); // the 0.5 makes it so that it doesnt shut off

                let o2_factor = o2_used / min_o2_level;
                let glucose_burned = glucose_used * o2_factor.max(0.05);
                self.nutrients.glucose -= glucose_burned;

                // atp yield
                let anaerobic_yield = 2.0 * glucose_used * (1.0 - o2_factor);
                let atp_yield = (36.0 * glucose_burned + anaerobic_yield) * organelle.efficiency;
                self.atp += atp_yield;

                // maintenance cost
                self.atp -= 0.0001 * self.properties.metabolism_rate * self.atp;
            }
        }
    }

    // 1 glucose = 10**6 molecules
}

#[cfg(test)]
mod tests {
    use raylib::math::Vector2;

    use crate::organelle::{Organelle, OrganelleKind};
    use crate::cell::property::BASE_RESPIRATION_RATE;

    use super::Cell;

    #[test]
    fn generate_energy_test() {
        let mut cell = Cell::default();
        cell.nutrients.glucose = 10.1;
        cell.properties.metabolism_rate = 0.01;
        cell.o2_level = 21.0;

        let area = cell.size.x * cell.size.y;
        let size_ratio = area / (area + 1.0);

        cell.nutrients.max_glucose = area * 0.1 + cell.organelles.len() as f32 * 0.0001;
        cell.properties.respiration_rate = BASE_RESPIRATION_RATE * cell.properties.metabolism_rate * size_ratio;

        cell.env_o2_level = 0.0;
        cell.max_o2_level = 21.0 + cell.organelles.len() as f32 * 0.001;
        cell.max_atp = 2160.0;

        cell.velocity = Vector2::new(0.05, 0.0);

        for _ in 0..1 {
            cell.organelles.push(Organelle::new(OrganelleKind::Mitochondria));
        }

        for tick in 0..10 {
            cell.generate_atp();
            cell.breathe();

            println!("Cell's ATP: {}", cell.atp);
            println!("Cell's glucose: {}", cell.nutrients.glucose);
            println!("Cell's oxygen level: {}", cell.o2_level);
        }
    }
}
