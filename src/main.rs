use phylogen::{cell::{environment::Environment, property::CellProperties, Cell}, nutrients::{nutrient_node::NutrientNode, Nutrients}, organelle::{Organelle, OrganelleKind}};
use rand::random_range;
use raylib::prelude::*;
use cs_utils::drain_filter;

fn main() {
    let (mut rl, thread) = raylib::init()
        .title("Phylogen")
        .build();

    rl.set_target_fps(60);

    let mut organelles = vec![];
    for _ in 0..1 {
        organelles.push(Organelle::new(OrganelleKind::Mitochondria));
        organelles.push(Organelle::new(OrganelleKind::Chloroplast));
    }

    let size = Vector2::new(5.0, 5.0);
    let area = size.x * size.y;

    let mut cells = vec![];
    for _ in 0..1 {
        let cell = Cell {
            atp: 1000.0,
            max_atp: 2160.0,
            o2_level: 21.0,
            max_o2_level: 21.0 + organelles.len() as f32 * 0.001,
            properties: CellProperties {
                metabolism_rate: 0.001,
                respiration_rate: 0.01,
                ..Default::default()
            },
            organelles: organelles.clone(),
            pos: Vector2::new(random_range(0.0..300.0), random_range(0.0..300.0)),
            size,
            nutrients: Nutrients::new(),
            velocity: Vector2::new(0.05, 0.0),
            consumed: false,
            nearest_food: None,
            nearest_food_dist_sq: f32::MAX,
            max_photosynthesis_rate: 0.5,
            env: Environment {
                env_o2_level: 21.0,
                light_intensity: 150.0,
                co2_concentration_level: 400.0,
            },
        };

        cells.push(cell);
    }

    let mut nutrient_nodes = vec![];
    for _ in 0..1 {
        let nutrient_node= NutrientNode::new(Vector2::new(random_range(0.0..300.0), random_range(0.0..300.0)));
        nutrient_nodes.push(nutrient_node);
    }

    while !rl.window_should_close() {
        let fps = rl.get_fps();
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::BLACK);

        for cell in &mut cells {
            cell.update();
            for node in nutrient_nodes.iter().cloned() {
                cell.find_food(node);
                cell.extract_glucose_from_light();
                node.draw(&mut d);
            }

            for node in &mut nutrient_nodes {
                cell.eat(node);
            }
        }

        nutrient_nodes.retain(|node| !node.consumed);

        for cell in &cells {
            cell.draw(&mut d);
        // }

            d.draw_text(
                &format!("Cell's ATP: {}", cell.atp),
                20, 20,
                10,
                Color::WHITE,
            );
            d.draw_text(
                &format!("Cell's glucose: {}", cell.nutrients.glucose),
                20, 40,
                10,
                Color::WHITE,
            );
            d.draw_text(
                &format!("Cell's O2 level: {}", cell.o2_level),
                20, 60,
                10,
                Color::WHITE,
            );
            d.draw_text(
                &format!("Cell's pos: {:?}", cell.pos),
                20, 80,
                10,
                Color::WHITE,
            );
        }

        d.draw_text(&fps.to_string(), 600, 20, 16, Color::GREEN);

        // println!("Cell's ATP: {}", cell.atp);
        // println!("Cell's glucose: {}", cell.nutrients.glucose);
        // println!("Cell's oxygen level: {}", cell.o2_level);
        // println!("Cell's position: {:?}", cell.pos);
    }
}
