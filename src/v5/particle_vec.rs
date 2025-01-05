use std::sync::{Arc, RwLock};

use super::{particle::Particle, particle_handle::ParticleHandle};
use bevy::{color::Color, math::{vec2, Vec2}};


/* 
pub trait ParticleAdd {
    /// Add a particle to this particle vector.
    fn add(&mut self, particle: Particle) -> ParticleHandle;

    /// Add multiple particles to this particle vector.
    fn add_vec(&mut self, particles: &Vec<Particle>) -> Vec<ParticleHandle>;
}*/

/* 
// https://www.cs.brandeis.edu/~cs146a/rust/rustbyexample-02-21-2015/simd.html

#[derive(Clone)]
pub struct SharedParticleVec(Arc<RwLock<ParticleVec>>);

impl Default for SharedParticleVec {
    fn default() -> Self { 
        SharedParticleVec(Arc::new(RwLock::new(ParticleVec::default())))
    }
}

impl AsRef<ParticleVec> for SharedParticleVec {
    fn as_ref(&self) -> &ParticleVec {
        // Return a reference to the inner value
        &self.0
    }
}*/

pub type SharedParticleVec = Arc<RwLock<ParticleVec>>;


pub struct ParticleVec {
    pub pos_x: Vec<f32>,
    pub pos_y: Vec<f32>,

    pub pos_prev_x: Vec<f32>,
    pub pos_prev_y: Vec<f32>,

    pub radius: Vec<f32>,
    pub mass: Vec<f32>,

    pub is_static: Vec<bool>,
    pub color: Vec<Color>,
    pub is_enabled: Vec<bool>,

    pub force: Vec<Vec2>, // should this be here? when we apply a force can we not just move the pos?
}

impl ParticleVec {
    /// Add a particle to this particle vector.
    pub fn add(&mut self, particle: Particle) -> ParticleHandle {
        let id = self.len();

        self.pos_x.push(particle.pos.x);
        self.pos_y.push(particle.pos.y);
        self.pos_prev_x.push(particle.pos_prev.x);
        self.pos_prev_y.push(particle.pos_prev.y);
        self.radius.push(particle.radius);
        self.mass.push(particle.mass);
        self.is_static.push(particle.is_static);
        self.color.push(particle.color);
        self.is_enabled.push(particle.is_enabled);
        self.force.push(particle.force);

        ParticleHandle::new(id) 
    }

    pub fn add_vec(&mut self, particles: &Vec<Particle>) -> Vec<ParticleHandle> {
        let mut handles = Vec::new();
        for p in particles {
            handles.push(self.add(*p));
        }
        handles
    }
}

impl ParticleVec {

    /// Get the particle that the particle_handle refers to.
    pub fn get(&self, particle_handle: ParticleHandle) -> Option<Particle> {
        let id = particle_handle.id();
        if id >= self.len() {
            return None;
        }

        Some(Particle { 
            pos: vec2(self.pos_x[id], self.pos_y[id]), 
            pos_prev: vec2(self.pos_prev_x[id], self.pos_prev_y[id]), 
            radius: self.radius[id], 
            mass: self.mass[id], 
            is_static: self.is_static[id], 
            color: self.color[id], 
            is_enabled: self.is_enabled[id], 
            force: self.force[id]
        })
    }

    pub fn len(&self) -> usize {
        self.pos_x.len()
    }


    pub fn update_positions(&mut self, delta_seconds: f32) {
        let particle_count = self.len();
        for id in 0..particle_count {
            /*
            if i == 65 {
                println!("65!");
            }*/
            if self.is_static[id] || !self.is_enabled[id] {
                continue
            }

            let pos = vec2(self.pos_x[id], self.pos_y[id]);
            let pos_prev = vec2(self.pos_prev_x[id], self.pos_prev_y[id]);

            let velocity: Vec2 = pos - pos_prev;
            let acceleration: Vec2 = self.force[id] / self.mass[id];

            //println!("accel {}, vel {}", acceleration, velocity);

            self.pos_prev_x[id] = pos.x;
            self.pos_prev_y[id] = pos.y;

            let new_pos = pos + velocity + acceleration * delta_seconds * delta_seconds;
            debug_assert!(!new_pos.x.is_nan());
            debug_assert!(!new_pos.y.is_nan());

            self.pos_x[id] = new_pos.x;
            self.pos_y[id] = new_pos.y;
        }
    }
}

impl Default for ParticleVec {
    fn default() -> Self { 
        Self {
            pos_x: vec![],
            pos_y: vec![],

            pos_prev_x: vec![],
            pos_prev_y: vec![],

            radius: vec![],
            mass: vec![],

            is_static: vec![],
            color: vec![],
            is_enabled: vec![],

            force: vec![],
        }
    }
}