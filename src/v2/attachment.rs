use std::{cell::RefCell, rc::Rc};

use cgmath::Vector2;
use sdl2::{gfx::primitives::DrawRenderer, pixels::Color};

use crate::sdl_system::SdlSystem;

use super::{particle::Particle, position::Position};

pub struct ParticleAttachment { 
    particle: Rc<RefCell<Particle>>,
    weight: f32,
}

impl ParticleAttachment {
    pub fn new(particle: &Rc<RefCell<Particle>>, weight: f32) -> Self {
        Self { particle: particle.clone(), weight }
    }
}

// An Attachment has a set of weighted particles
// such that when updated, computes a virtual position
// from which you can attach things too. This set is called incomming_particles
//
// An Attachment also has a set of weighted particles to 'push' for when this attachment itself is moved. This set is called outgoing_particles
//
// todo: consider if this is just a 'constraint', and can be just lumped in with sticks?
pub struct Attachment {
    pub incoming_particle_attachments: Vec<ParticleAttachment>,
    pub outgoing_particle_attachments: Vec<ParticleAttachment>,
    pub pos: Vector2<f32>,
}

impl Position for Attachment {
    fn get_position(&self) -> Vector2<f32> {
        self.pos
    }

    fn set_position(&mut self, pos: Vector2<f32>) {
        let pos_delta = pos - self.pos;
        for particle_attachment in self.outgoing_particle_attachments.iter() {
            let mut p = particle_attachment.particle.as_ref().borrow_mut();
            p.pos += pos_delta * particle_attachment.weight;
        }

        self.pos = pos;
    }
}

impl Attachment {
    pub fn new() -> Self {
        Self { incoming_particle_attachments: vec![], outgoing_particle_attachments: vec![], pos: Vector2::new(0f32, 0f32) }
    }

    pub fn add_incoming_particle(&mut self, particle: &Rc<RefCell<Particle>>, weight: f32) {
        self.incoming_particle_attachments.push(ParticleAttachment::new(particle, weight));
    }

    pub fn add_outgoing_particle(&mut self, particle: &Rc<RefCell<Particle>>, weight: f32) {
        self.outgoing_particle_attachments.push(ParticleAttachment::new(particle, weight));
    }

    pub fn add_outgoing_particles(&mut self, particles: &Vec<Rc<RefCell<Particle>>>, weight: f32) {
        for particle in particles.iter() {
            self.add_outgoing_particle(particle, weight);
        }
    }

    pub fn add_even_distribution_of_incoming_particles(&mut self, particles: &Vec<Rc<RefCell<Particle>>>, weight: f32, num_particles_to_add: usize) {
        // todo: what if we cant get an even distribution?
        let i = particles.len() / num_particles_to_add;
        for pn in 0..num_particles_to_add { //particles.iter() {
            let p_idx = pn * i;
            let particle = &particles[p_idx];
            self.add_incoming_particle(particle, weight);
        }
    }

    pub fn update(&mut self, _dt: f32) {
        self.pos = Vector2::new(0f32, 0f32);
        for particle_attachment in self.incoming_particle_attachments.iter() {
            let p = particle_attachment.particle.as_ref().borrow_mut();
            self.pos += p.pos * particle_attachment.weight;
        }
        self.pos /= self.incoming_particle_attachments.len() as f32;
    }

    pub fn draw(&self, sdl: &mut SdlSystem) {
        let pos_x = i16::try_from(self.pos[0].round() as i32).unwrap();
        let pos_y = i16::try_from(self.pos[1].round() as i32).unwrap();
        let r = 4; //i16::try_from(self.radius as i32).unwrap();

        let color = Color::RGB(255, 0, 0);
        sdl.canvas.filled_circle(pos_x, pos_y, r, color).unwrap();
    }

}