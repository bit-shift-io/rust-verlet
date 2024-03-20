use super::particle::Particle;
use super::stick::Stick;

pub struct Body<'a> {
    pub particles: Vec<Box<Particle>>,
    pub sticks: Vec<Box<Stick<'a>>>,
    pub collides_with_self: bool,
    // collision_group(s) ?
}

impl<'a> Body<'a> {
    pub fn new() -> Self {
        Self { particles: vec![], sticks: vec![], collides_with_self: false }
    }

    pub fn add_particle(&mut self, particle: Box<Particle>) {
        self.particles.push(particle);
    }

    pub fn add_stick(&mut self, stick: Box<Stick<'a>>) {
        self.sticks.push(stick);
    }
}