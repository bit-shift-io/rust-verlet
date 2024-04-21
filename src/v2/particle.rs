use cgmath::Vector2;
use sdl2::{gfx::primitives::DrawRenderer, pixels::Color};

use crate::sdl_system::SdlSystem;

use super::{position::Position, types::Vec2};

pub struct Particle {
    pub pos: Vec2,
    pub pos_prev: Vec2,
    pub pos_init: Vec2,
    pub force: Vec2,
    pub radius: f32,
    pub mass: f32,
    pub color: Color,
    pub is_pinned: bool,
    pub is_selected: bool,
}

impl Position for Particle {
    fn get_position(&self) -> Vec2 {
        self.pos
    }

    fn set_position(&mut self, pos: Vec2) {
        self.pos = pos;
    }
}

impl Particle {
    pub fn new(pos: Vec2, radius: f32, mass: f32, color: Color) -> Self {
        Self { pos, pos_prev: pos, pos_init: pos, radius, mass, color, force: Vector2::new(0f32, 0f32), is_pinned: false, is_selected: false }
    }

    pub fn draw(&self, sdl: &mut SdlSystem) {
        let pos_x = i16::try_from(self.pos[0].round() as i32).unwrap();
        let pos_y = i16::try_from(self.pos[1].round() as i32).unwrap();
        let r = i16::try_from(self.radius as i32).unwrap();
        sdl.canvas.filled_circle(pos_x, pos_y, r, self.color).unwrap();
    }

    pub fn update_position(&mut self, dt: f32) {
        let velocity: Vec2 = self.pos - self.pos_prev;
        let acceleration: Vec2 = self.force / self.mass;
        self.pos_prev = self.pos;
        self.pos = self.pos + velocity + acceleration * dt * dt;
    }

    pub fn set_force(&mut self, force: Vec2) {
        self.force = force;
    }

    pub fn add_force(&mut self, force: Vec2) {
        self.force += force;
    }

    pub fn acceleration_to_force(&self, acc: Vec2) -> Vec2 {
        acc * self.mass
    }
/* 
    pub fn accelerate(&mut self, acc: Vec2) {
        let force = acc * self.mass;
        self.force = force;
    }

    pub fn add_acceleration(&mut self, acc: Vec2) {
        let force = acc * self.mass;
        self.force += force;
    }*/
}