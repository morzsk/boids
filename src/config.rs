pub const SIZE_RADIUS: f32 = 10.0;
pub const PERCEPTION_RADIUS: f32 = 50.0;
pub const SEPARATION_WEIGHT: f32 = 1.0;
pub const ALIGNMENT_WEIGHT: f32 = 1.25;
pub const COHESION_WEIGHT: f32 = 0.75;
pub const MAX_SPEED: f32 = 3.0;
pub const MAX_FORCE: f32 = 0.1;
pub const BOID_COUNT_MIN: usize = 0;
pub const BOID_COUNT_MAX: usize = 1000;
pub const BOID_COUNT_DEFAULT: usize = 200;

pub struct Config {
    pub boid_count: f32,
    pub separation_weight: f32,
    pub alignment_weight: f32,
    pub cohesion_weight: f32,
    pub perception_radius: f32,
    pub size_radius: f32,
    pub max_speed: f32,
    pub max_force: f32,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            boid_count: BOID_COUNT_DEFAULT as f32,
            separation_weight: SEPARATION_WEIGHT,
            alignment_weight: ALIGNMENT_WEIGHT,
            cohesion_weight: COHESION_WEIGHT,
            perception_radius: PERCEPTION_RADIUS,
            size_radius: SIZE_RADIUS,
            max_speed: MAX_SPEED,
            max_force: MAX_FORCE,
        }
    }
}
