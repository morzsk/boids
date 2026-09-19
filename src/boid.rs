use macroquad::prelude::*;
use macroquad::rand::gen_range;

use crate::config::Config;

#[inline]
fn sq(x: f32) -> f32 {
    x * x
}

#[derive(Clone, Copy)]
pub struct Boid {
    position: Vec2,
    velocity: Vec2,
}

impl Boid {
    fn new(position: Vec2, velocity: Vec2) -> Self {
        Boid { position, velocity }
    }

    // we use reynold approach
    fn steer_towards(&self, direction: Vec2, config: &Config) -> Vec2 {
        if direction == Vec2::ZERO {
            return Vec2::ZERO;
        }
        let desired = direction.normalize_or_zero() * config.max_speed;
        (desired - self.velocity).clamp_length_max(config.max_force)
    }

    pub fn step(&self, boids: &[Boid], config: &Config) -> Boid {
        let neighbors: Vec<&Boid> = boids
            .iter()
            .filter(|other| {
                let distance_squared = self.position.distance_squared(other.position);
                distance_squared > 0.0 && distance_squared < sq(config.perception_radius)
            })
            .collect();

        let separation: Vec2 = neighbors
            .iter()
            .map(|other| {
                let away = self.position - other.position;
                away / away.length_squared()
            })
            .sum();

        let alignment: Vec2 = neighbors.iter().map(|other| other.velocity).sum();

        let cohesion: Vec2 = if neighbors.is_empty() {
            Vec2::ZERO
        } else {
            let center =
                neighbors.iter().map(|other| other.position).sum::<Vec2>() / neighbors.len() as f32;
            center - self.position
        };

        let steer = self.steer_towards(separation, config) * config.separation_weight
            + self.steer_towards(alignment, config) * config.alignment_weight
            + self.steer_towards(cohesion, config) * config.cohesion_weight;

        let velocity = (self.velocity + steer).clamp_length_max(config.max_speed);

        let mut position = self.position + velocity;
        position.x = position.x.rem_euclid(screen_width());
        position.y = position.y.rem_euclid(screen_height());

        Boid { position, velocity }
    }

    pub fn draw(&self, debug: bool, config: &Config) {
        let dir = self.velocity.normalize_or_zero();
        let perp = vec2(-dir.y, dir.x);

        let nose = self.position + dir * config.size_radius;
        let left = self.position - dir * config.size_radius + perp * config.size_radius * 0.6;
        let right = self.position - dir * config.size_radius - perp * config.size_radius * 0.6;

        draw_triangle(nose, left, right, WHITE);

        if !debug {
            return;
        }

        // perception radius
        draw_circle_lines(
            self.position.x,
            self.position.y,
            config.perception_radius,
            1.0,
            GRAY,
        );

        // velocity (green)
        let vel_end = self.position + dir * config.size_radius * 2.0;
        draw_line(
            self.position.x,
            self.position.y,
            vel_end.x,
            vel_end.y,
            2.0,
            GREEN,
        );

        // perpendicular (red)
        let perp_end = self.position + perp * config.size_radius * 2.0;
        draw_line(
            self.position.x,
            self.position.y,
            perp_end.x,
            perp_end.y,
            2.0,
            RED,
        );
    }
}

pub fn spawn_boid() -> Boid {
    Boid::new(
        vec2(
            gen_range(0.0, screen_width()),
            gen_range(0.0, screen_height()),
        ),
        vec2(gen_range(-1.0, 1.0), gen_range(-1.0, 1.0)),
    )
}

pub fn spawn_boids(count: usize) -> Vec<Boid> {
    (0..count).map(|_| spawn_boid()).collect()
}
