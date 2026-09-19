use macroquad::miniquad::date;
use macroquad::prelude::*;
use macroquad::rand::srand;
use macroquad::ui::root_ui;

mod boid;
mod config;
mod ui;

use crate::boid::{spawn_boid, spawn_boids};
use crate::config::Config;
use crate::ui::{draw_controls, make_skin};

#[macroquad::main("MyGame")]
async fn main() {
    srand(date::now() as u64);

    next_frame().await;

    let mut debug = false;
    let mut show_controls = true;
    let mut config = Config::default();
    let mut boids = spawn_boids(config.boid_count as usize);

    let skin = make_skin();

    loop {
        clear_background(BLACK);
        root_ui().push_skin(&skin);

        if is_key_pressed(KeyCode::E) {
            debug = !debug;
        }
        if is_key_pressed(KeyCode::R) {
            boids = spawn_boids(config.boid_count as usize);
        }
        if is_key_pressed(KeyCode::T) {
            show_controls = !show_controls;
        }

        if show_controls {
            draw_controls(&mut config);
        }

        let target = config.boid_count as usize;
        while boids.len() < target {
            boids.push(spawn_boid());
        }
        boids.truncate(target);

        boids = boids
            .iter()
            .map(|boid| boid.step(&boids, &config))
            .collect();

        for boid in &boids {
            boid.draw(debug, &config);
        }

        next_frame().await
    }
}
