use macroquad::miniquad::date;
use macroquad::prelude::*;
use macroquad::rand::{gen_range, srand};

const RADIUS: f32 = 10.0;
const BOID_COUNT_MIN: usize = 20;
const BOID_COUNT_MAX: usize = 50;

#[derive(Clone, Copy)]
struct Boid {
    position: Vec2,
    velocity: Vec2,
    radius: f32,
}

impl Boid {
    fn new(position: Vec2, velocity: Vec2) -> Self {
        Boid {
            position,
            velocity,
            radius: RADIUS,
        }
    }

    fn step(&self, boids: &[Boid]) -> Boid {
        let mut steer = Vec2::ZERO;

        for other in boids {
            if other.position == self.position {
                continue;
            }

            // TODO: separation: steer away from nearby boids
            // TODO: alignment : steer toward the average heading of neighbors
            // TODO: cohesion  : steer toward the average position of neighbors
            let _ = other;
        }

        Boid {
            position: self.position + self.velocity,
            velocity: self.velocity + steer,
            ..*self
        }
    }

    fn draw(&self) {
        let dir = self.velocity.normalize_or_zero();
        let perp = vec2(-dir.y, dir.x);

        let nose = self.position + dir * self.radius;
        let left = self.position - dir * self.radius + perp * self.radius * 0.6;
        let right = self.position - dir * self.radius - perp * self.radius * 0.6;

        draw_triangle(nose, left, right, WHITE);
    }
}

#[macroquad::main("MyGame")]
async fn main() {
    srand(date::now() as u64);

    next_frame().await;

    let count = gen_range(BOID_COUNT_MIN, BOID_COUNT_MAX);
    let mut boids: Vec<Boid> = (0..count)
        .map(|_| {
            Boid::new(
                vec2(
                    gen_range(0.0, screen_width()),
                    gen_range(0.0, screen_height()),
                ),
                vec2(gen_range(-1.0, 1.0), gen_range(-1.0, 1.0)),
            )
        })
        .collect();

    loop {
        clear_background(BLACK);

        boids = boids.iter().map(|boid| boid.step(&boids)).collect();

        for boid in &boids {
            boid.draw();
        }

        next_frame().await
    }
}
