use cgmath::Vector2;
use sdl2::{gfx::primitives::DrawRenderer, pixels::Color};

use crate::sdl_system::SdlSystem;

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

    pub fn draw(&self, sdl: &mut SdlSystem) {
        let pos_x = i16::try_from(self.pos[0].round() as i32).unwrap();
        let pos_y = i16::try_from(self.pos[1].round() as i32).unwrap();
        let r = i16::try_from(self.radius as i32).unwrap();
        sdl.canvas.filled_circle(pos_x, pos_y, r, self.color).unwrap();
    }

    pub fn update_position(&mut self, dt: f32) {
        let velocity: Vector2<f32> = self.pos - self.pos_prev;
        let acceleration: Vector2<f32> = self.force / self.mass;
        self.pos_prev = self.pos;
        self.pos = self.pos + velocity + acceleration * dt * dt;
    }

    pub fn accelerate(&mut self, acc: Vector2<f32>) {
        let force = acc * self.mass;
        self.force = force;
    }
}