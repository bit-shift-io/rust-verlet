use std::{simd::f32x2, sync::{Arc, RwLock}};

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
    pub pos: Vec<f32x2>,
    pub pos_prev: Vec<f32x2>,

    pub radius: Vec<f32>,
    pub mass: Vec<f32>,

    pub is_static: Vec<bool>,
    pub color: Vec<Color>,
    pub is_enabled: Vec<bool>,

    pub force: Vec<f32x2>, // should this be here? when we apply a force can we not just move the pos?
}

impl ParticleVec {
    /// Add a particle to this particle vector.
    pub fn add(&mut self, particle: Particle) -> ParticleHandle {
        let id = self.len();

        self.pos.push(f32x2::from_array([particle.pos.x, particle.pos.y]));
        self.pos_prev.push(f32x2::from_array([particle.pos_prev.x, particle.pos_prev.y]));
        self.radius.push(particle.radius);
        self.mass.push(particle.mass);
        self.is_static.push(particle.is_static);
        self.color.push(particle.color);
        self.is_enabled.push(particle.is_enabled);
        self.force.push(f32x2::from_array([particle.force.x, particle.force.y]));

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

    #[inline(always)]
    pub fn get_pos_vec2(&self, id: usize) -> Vec2 {
        let pos = self.pos[id].as_array();
        vec2(pos[0], pos[1])
    }

    #[inline(always)]
    pub fn set_pos_from_vec2(&mut self, id: usize, pos: &Vec2) {
        self.pos[id] = f32x2::from_array([pos.x, pos.y]);
    }

    /// Get the particle that the particle_handle refers to.
    pub fn get(&self, particle_handle: ParticleHandle) -> Option<Particle> {
        let id = particle_handle.id();
        if id >= self.len() {
            return None;
        }

        let pos = self.pos[id].as_array();
        let pos_prev = self.pos_prev[id].as_array();
        let force = self.force[id].as_array();

        Some(Particle { 
            pos: vec2(pos[0], pos[1]), 
            pos_prev: vec2(pos_prev[0], pos_prev[1]), 
            radius: self.radius[id], 
            mass: self.mass[id], 
            is_static: self.is_static[id], 
            color: self.color[id], 
            is_enabled: self.is_enabled[id], 
            force: vec2(force[0], force[1]), 
        })
    }

    pub fn len(&self) -> usize {
        self.pos.len()
    }

    pub fn update_positions(&mut self, delta_seconds: f32) {
        let delta_seconds_sqrd = delta_seconds * delta_seconds;
        let delta_seconds_sqrd_f32x2 = f32x2::from_array([delta_seconds_sqrd, delta_seconds_sqrd]);

        // todo: can we take 2x f32x2 and stuff into f32x4 to process 2 particles at once doubling the speed?
        let particle_count = self.len();
        for id in 0..particle_count {
            if self.is_static[id] || !self.is_enabled[id] {
                continue
            }

            let pos = self.pos[id];
            let pos_prev = self.pos_prev[id];

            let velocity = pos - pos_prev;
            let acceleration = self.force[id] / f32x2::from_array([self.mass[id], self.mass[id]]);

            //println!("accel {}, vel {}", acceleration, velocity);

            self.pos_prev[id] = pos;
            let new_pos = pos + velocity + acceleration * delta_seconds_sqrd_f32x2;
            
            debug_assert!(!new_pos[0].is_nan());
            debug_assert!(!new_pos[1].is_nan());

            self.pos[id] = new_pos;
        }
    }
}

impl Default for ParticleVec {
    fn default() -> Self { 
        Self {
            pos: vec![],
            pos_prev: vec![],

            radius: vec![],
            mass: vec![],

            is_static: vec![],
            color: vec![],
            is_enabled: vec![],

            force: vec![],
        }
    }
}