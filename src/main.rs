use macroquad::miniquad::date;
use macroquad::prelude::*;
use macroquad::rand::{gen_range, srand};

#[inline]
fn sq(x: f32) -> f32 {
    x * x
}

const RADIUS: f32 = 10.0;
const PERCEPTION_RADIUS: f32 = 50.0;
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

            // separation
            let away = self.position - other.position;
            let distance_squared = away.length_squared();
            if distance_squared > 0.0 && distance_squared < sq(PERCEPTION_RADIUS) {
                steer += away / distance_squared;
            }

            // TODO: alignment : steer toward the average heading of neighbors
            // TODO: cohesion  : steer toward the average position of neighbors
        }

        let mut position = self.position + self.velocity;
        position.x = position.x.rem_euclid(screen_width());
        position.y = position.y.rem_euclid(screen_height());

        Boid {
            position,
            velocity: self.velocity + steer,
            ..*self
        }
    }

    fn draw(&self, debug: bool) {
        let dir = self.velocity.normalize_or_zero();
        let perp = vec2(-dir.y, dir.x);

        let nose = self.position + dir * self.radius;
        let left = self.position - dir * self.radius + perp * self.radius * 0.6;
        let right = self.position - dir * self.radius - perp * self.radius * 0.6;

        draw_triangle(nose, left, right, WHITE);

        if !debug {
            return;
        }

        // perception radius
        draw_circle_lines(
            self.position.x,
            self.position.y,
            PERCEPTION_RADIUS,
            1.0,
            GRAY,
        );

        // velocity (green)
        let vel_end = self.position + dir * self.radius * 2.0;
        draw_line(
            self.position.x,
            self.position.y,
            vel_end.x,
            vel_end.y,
            2.0,
            GREEN,
        );

        // perpendicular (red)
        let perp_end = self.position + perp * self.radius * 2.0;
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

    let mut debug = false;

    loop {
        clear_background(BLACK);

        if is_key_pressed(KeyCode::D) {
            debug = !debug;
        }

        boids = boids.iter().map(|boid| boid.step(&boids)).collect();

        for boid in &boids {
            boid.draw(debug);
        }

        next_frame().await
    }
}
