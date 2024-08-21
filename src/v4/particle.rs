use bevy::{color::Color, math::Vec2};

#[derive(Debug, Copy, Clone)]
pub struct Particle {
    pub pos: Vec2,
    pub radius: f32,
    pub mass: f32,
    pub is_static: bool,
    pub color: Color,
}

impl Particle {
    pub fn new(pos: Vec2, radius: f32, mass: f32, is_static: bool, color: Color) -> Self {
        Self { pos, radius, mass, is_static, color }
    }

    pub fn set_radius(&mut self, radius: f32) -> &mut Self {
        self.radius = radius;
        self
    }

    pub fn set_position(&mut self, pos: Vec2) -> &mut Self {
        self.pos = pos;
        self
    }

    pub fn set_static(&mut self, is_static: bool) -> &mut Self {
        self.is_static = is_static;
        self
    }
}

impl Default for Particle {
    fn default() -> Self {
        Self {
            pos: Vec2::new(0.0, 0.0),
            radius: 0.5,
            mass: 1.0,
            is_static: false,
            color: Color::WHITE,
        }
    }
}