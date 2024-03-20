use cgmath::InnerSpace;

use super::particle::Particle;

pub struct Stick<'a> {
    pub length: f32,
    pub p1: &'a Box<Particle>,
    pub p2: &'a Box<Particle>,
    //pub color: Color,
}

impl<'a> Stick<'a> {
    pub fn new(p1: &'a Box<Particle>, p2: &'a Box<Particle>) -> Self {
        let length = (p1.pos - p2.pos).magnitude();
        Self { length, p1, p2 }
    }
}