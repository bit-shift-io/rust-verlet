use sdl2::pixels::Color;
use sdl2::rect::Point;

use crate::sdl_system::SdlSystem;

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

    pub fn update(&mut self, dt: f32) {
        /* 
        self.apply_gravity();
        self.apply_containment_constraint();
        self.solve_collisions(sub_dt);
        self.update_positions(sub_dt);
        */
    }

    pub fn draw(&self, sdl: &mut SdlSystem) {
        // draw particles
        for particle in self.particles.iter() {
            particle.draw(sdl);
        }

        // draw stick constraints
        for stick in self.sticks.iter() {
            stick.draw(sdl);
        }
    }
}