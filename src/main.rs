use phylogen::{cell::{property::CellProperties, Cell}, organelle::{Organelle, OrganelleKind}};
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
        glucose: 100.0,
        properties: CellProperties {
            metabolism_rate: 0.001,
            respiration_rate: 0.01,
            ..Default::default()
        },
        organelles: organelles.clone(),
        pos: Vector2::new(100.0, 100.0),
        size,
        max_glucose: area * 0.1 + organelles.len() as f32 * 0.0001,
        velocity: Vector2::new(0.05, 0.0),
    };

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::BLACK);

        cell.draw(&mut d);
        cell.update();

        println!("Cell's ATP: {}", cell.atp);
        println!("Cell's glucose: {}", cell.glucose);
        println!("Cell's oxygen level: {}", cell.o2_level);
    }
}
