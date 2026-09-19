use macroquad::prelude::*;
use macroquad::ui::{Skin, hash, root_ui};

use crate::config::{BOID_COUNT_MAX, BOID_COUNT_MIN, Config};

pub fn make_skin() -> Skin {
    let label_style = root_ui()
        .style_builder()
        .text_color(Color::new(0.9, 0.9, 0.95, 1.0))
        .font_size(18)
        .build();
    let window_style = root_ui()
        .style_builder()
        .color(Color::new(0.08, 0.09, 0.12, 0.85))
        .build();
    Skin {
        label_style,
        window_style,
        ..root_ui().default_skin()
    }
}

pub fn draw_controls(config: &mut Config) {
    root_ui().window(hash!(), vec2(10.0, 10.0), vec2(260.0, 260.0), |ui| {
        ui.slider(
            hash!(),
            "count",
            BOID_COUNT_MIN as f32..BOID_COUNT_MAX as f32,
            &mut config.boid_count,
        );
        ui.slider(
            hash!(),
            "separation",
            0.0..3.0,
            &mut config.separation_weight,
        );
        ui.slider(hash!(), "alignment", 0.0..3.0, &mut config.alignment_weight);
        ui.slider(hash!(), "cohesion", 0.0..3.0, &mut config.cohesion_weight);
        ui.slider(
            hash!(),
            "perception",
            10.0..200.0,
            &mut config.perception_radius,
        );
        ui.slider(hash!(), "size", 1.0..30.0, &mut config.size_radius);
        ui.slider(hash!(), "max force", 0.0..1.0, &mut config.max_force);
        ui.slider(hash!(), "max speed", 0.5..10.0, &mut config.max_speed);
    });
}
