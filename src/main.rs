use macroquad::miniquad::date;
use macroquad::prelude::*;
use macroquad::rand::{gen_range, srand};

#[inline]
fn sq(x: f32) -> f32 {
    x * x
}

const RADIUS: f32 = 10.0;
const PERCEPTION_RADIUS: f32 = 50.0;
const SEPARATION_WEIGHT: f32 = 0.5;
const ALIGNMENT_WEIGHT: f32 = 1.0;
const MAX_SPEED: f32 = 3.0;
const MAX_FORCE: f32 = 0.1;
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

    // we use reynold approach
    fn steer_towards(&self, direction: Vec2) -> Vec2 {
        if direction == Vec2::ZERO {
            return Vec2::ZERO;
        }
        let desired = direction.normalize_or_zero() * MAX_SPEED;
        (desired - self.velocity).clamp_length_max(MAX_FORCE)
    }

    fn step(&self, boids: &[Boid]) -> Boid {
        let neighbors: Vec<&Boid> = boids
            .iter()
            .filter(|other| {
                let distance_squared = self.position.distance_squared(other.position);
                distance_squared > 0.0 && distance_squared < sq(PERCEPTION_RADIUS)
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

        // TODO: cohesion  : steer toward the average position of neighbors

        let steer = self.steer_towards(separation) * SEPARATION_WEIGHT
            + self.steer_towards(alignment) * ALIGNMENT_WEIGHT;

        let velocity = (self.velocity + steer).clamp_length_max(MAX_SPEED);

        let mut position = self.position + velocity;
        position.x = position.x.rem_euclid(screen_width());
        position.y = position.y.rem_euclid(screen_height());

        Boid {
            position,
            velocity,
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
