use phylogen::{cell::{property::CellProperties, Cell}, nutrients::{nutrient_node::NutrientNode, Nutrients}, organelle::{Organelle, OrganelleKind}};
use rand::random_range;
use raylib::prelude::*;

fn main() {
    let (mut rl, thread) = raylib::init()
        .title("Phylogen")
        .build();

    let mut organelles = vec![];
    for _ in 0..200 {
        organelles.push(Organelle::new(OrganelleKind::Mitochondria));
    }

    let size = Vector2::new(10.0, 10.0);
    let area = size.x * size.y;
    let mut cell = Cell {
        atp: 1000.0,
        max_atp: 2160.0,
        o2_level: 21.0,
        max_o2_level: 21.0 + organelles.len() as f32 * 0.001,
        env_o2_level: 21.0,
        properties: CellProperties {
            metabolism_rate: 0.001,
            respiration_rate: 0.01,
            ..Default::default()
        },
        organelles: organelles.clone(),
        pos: Vector2::new(100.0, 100.0),
        size,
        nutrients: Nutrients::new(),
        velocity: Vector2::new(0.05, 0.0),
        consumed: false,
        nearest_food: None,
        nearest_food_dist_sq: f32::MAX,
    };
    let mut nutrient_nodes = vec![];

    for _ in 0..100 {
        let nutrient_node= NutrientNode::new(Vector2::new(random_range(0.0..500.0), random_range(0.0..500.0)));
        nutrient_nodes.push(nutrient_node);
    }

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::BLACK);

        cell.draw(&mut d);
        cell.update();

        for nutrient_node in &mut nutrient_nodes {
            let temp_nutrient_node = nutrient_node.clone();

            cell.find_food(&temp_nutrient_node);
            // cell.eat(nutrient_node);
            nutrient_node.draw(&mut d);
        }

        nutrient_nodes.retain(|node| !node.consumed);

        println!("Cell's ATP: {}", cell.atp);
        println!("Cell's glucose: {}", cell.nutrients.glucose);
        println!("Cell's oxygen level: {}", cell.o2_level);
        println!("Cell's position: {:?}", cell.pos);
    }
}
