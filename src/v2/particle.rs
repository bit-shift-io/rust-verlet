use cgmath::Vector2;
use sdl2::pixels::Color;

pub struct Particle {
    pub pos: Vector2<f32>,
    pub pos_prev: Vector2<f32>,
    pub pos_init: Vector2<f32>,
    pub force: Vector2<f32>,
    pub radius: f32,
    pub mass: f32,
    pub color: Color,
    pub is_pinned: bool,
    pub is_selected: bool,
}

impl Particle {
    pub fn new(pos: Vector2<f32>, radius: f32, mass: f32, color: Color) -> Self {
        Self { pos, pos_prev: pos, pos_init: pos, radius, mass, color, force: Vector2::new(0f32, 0f32), is_pinned: false, is_selected: false }
    }
}