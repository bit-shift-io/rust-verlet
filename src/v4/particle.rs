use bevy::{color::Color, math::{bounding::Aabb2d, vec2, Vec2}};

#[derive(Debug, Copy, Clone)]
pub struct Particle {
    pub pos: Vec2,
    pub pos_prev: Vec2,
    pub radius: f32,
    pub mass: f32,
    pub is_static: bool,
    pub color: Color,
    pub is_enabled: bool,

    pub force: Vec2, // should this be here? when we apply a force can we not just move the pos?
}

impl Particle {
    pub fn new(pos: Vec2, radius: f32, mass: f32, is_static: bool, color: Color) -> Self {
        Self { pos, pos_prev: pos, radius, mass, is_static, color, is_enabled: true, force: vec2(0.0, 0.0) }
    }

    pub fn set_radius(&mut self, radius: f32) -> &mut Self {
        self.radius = radius;
        self
    }

    pub fn set_mass(&mut self, mass: f32) -> &mut Self {
        self.mass = mass;
        self
    }

    pub fn set_position(&mut self, pos: Vec2) -> &mut Self {
        self.pos = pos;
        self.pos_prev = pos;
        self
    }

    pub fn set_static(&mut self, is_static: bool) -> &mut Self {
        self.is_static = is_static;
        self
    }

    pub fn get_aabb(&self) -> Aabb2d {
        Aabb2d {
            min: self.pos - vec2(self.radius, self.radius),
            max: self.pos + vec2(self.radius, self.radius),
        }
    }


    pub fn acceleration_to_force(acc: Vec2, mass: f32) -> Vec2 {
        acc * mass
    }

    pub fn set_force_based_on_acceleration(&mut self, acceleration: Vec2) -> &mut Self{
        self.force = acceleration * self.mass;
        self
    }

}

impl Default for Particle {
    fn default() -> Self {
        Self {
            pos: vec2(0.0, 0.0),
            pos_prev: vec2(0.0, 0.0),
            radius: 0.5,
            mass: 1.0,
            is_static: false,
            color: Color::WHITE,
            is_enabled: true,
            force: vec2(0.0, 0.0),
        }
    }
}