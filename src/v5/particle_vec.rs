use std::{simd::{f32x2, f32x4, i32x2}, sync::{Arc, RwLock}};

use std::simd::prelude::*;

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

    pub radius: Vec<f32x1>,
    pub mass: Vec<f32x1>,

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
        self.radius.push(f32x1::from_array([particle.radius]));
        self.mass.push(f32x1::from_array([particle.mass]));
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
            radius: self.radius[id][0], 
            mass: self.mass[id][0], 
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
        let delta_seconds_sqrd_f32x2 = f32x2::splat(delta_seconds_sqrd);//([delta_seconds_sqrd, delta_seconds_sqrd]);

        // todo: can we take 2x f32x2 and stuff into f32x4 to process 2 particles at once doubling the speed?
        let particle_count = self.len();
        for id in 0..particle_count {
            if self.is_static[id] || !self.is_enabled[id] {
                continue
            }

            let pos = self.pos[id];
            let pos_prev = self.pos_prev[id];

            let velocity = pos - pos_prev;
            let acceleration = self.force[id] / f32x2::splat(self.mass[id][0]); //from_array([self.mass[id], self.mass[id]]);

            //println!("accel {}, vel {}", acceleration, velocity);

            self.pos_prev[id] = pos;
            let new_pos = pos + velocity + acceleration * delta_seconds_sqrd_f32x2;
            
            debug_assert!(!new_pos[0].is_nan());
            debug_assert!(!new_pos[1].is_nan());

            self.pos[id] = new_pos;
        }
    }


    // attempt to try to use SIMD to process 2 particles at once. Its slower!
    pub fn update_positions_2(&mut self, delta_seconds: f32) {
        let delta_seconds_sqrd = delta_seconds * delta_seconds;
        let delta_seconds_sqrd_f32x4 = f32x4::splat(delta_seconds_sqrd);//([delta_seconds_sqrd, delta_seconds_sqrd]);

        // todo: can we take 2x f32x2 and stuff into f32x4 to process 2 particles at once doubling the speed?
        let particle_count = self.len();
        let mut ids = [0; 2];
        let mut id_offset = 0;

        
        for i in 0..particle_count {
            if self.is_static[i] || !self.is_enabled[i] {
                continue
            }

            ids[id_offset] = i;
            id_offset += 1;

            if (id_offset >= 2) {
                id_offset = 0;


                // zip the 2 particles given by ids[0] and ids[1] together
                let pos = f32x4::from_array([self.pos[ids[0]][0], self.pos[ids[0]][1], self.pos[ids[1]][0], self.pos[ids[1]][1]]);
                let pos_prev = f32x4::from_array([self.pos_prev[ids[0]][0], self.pos_prev[ids[0]][1], self.pos_prev[ids[1]][0], self.pos_prev[ids[1]][1]]);

                let force = f32x4::from_array([self.force[ids[0]][0], self.force[ids[0]][1], self.force[ids[1]][0], self.force[ids[1]][1]]);
                let mass = f32x4::from_array([self.mass[ids[0]][0], self.mass[ids[0]][0], self.mass[ids[1]][0], self.mass[ids[1]][0] ]);


                let velocity = pos - pos_prev;
                let acceleration = force / mass; //from_array([self.mass[id], self.mass[id]]);

                //println!("accel {}, vel {}", acceleration, velocity);

                self.pos_prev[ids[0]] = self.pos[ids[0]];
                self.pos_prev[ids[1]] = self.pos[ids[1]];

                let new_pos = pos + velocity + (acceleration * delta_seconds_sqrd_f32x4);
                
                //debug_assert!(!new_pos[0].is_nan());
                //debug_assert!(!new_pos[1].is_nan());

                self.pos[ids[0]] = f32x2::from_array([new_pos[0], new_pos[1]]);
                self.pos[ids[1]] = f32x2::from_array([new_pos[2], new_pos[3]]);
            }
        
        }
    }


    // attempt to process 2 particles at once
    pub fn update_positions_3(&mut self, delta_seconds: f32) {
        let delta_seconds_sqrd = delta_seconds * delta_seconds;
        let delta_seconds_sqrd_simd = f32x4::splat(delta_seconds_sqrd);//([delta_seconds_sqrd, delta_seconds_sqrd]);

        // todo: can we take 2x f32x2 and stuff into f32x4 to process 2 particles at once doubling the speed?
        let particle_count = self.len();

        // pointer to the start of the vector data
        let pos_ptr: *mut f32x4 = self.pos.as_mut_ptr() as *mut f32x4;
        let pos_prev_ptr: *mut f32x4 = self.pos_prev.as_ptr() as *mut f32x4;
        let force_ptr: *mut f32x4 = self.force.as_mut_ptr() as *mut f32x4;
        let mass_ptr: *mut f32x2 = self.mass.as_mut_ptr() as *mut f32x2;

        let chunks = particle_count / 2;

        // todo: handle any left over particles if there is an odd number of particles (as is done by the example urls below)!

        for i in 0..chunks as isize {

            // https://www.cs.brandeis.edu/~cs146a/rust/rustbyexample-02-21-2015/simd.html
            // it might be better somehow seperate static and enabled particles?
            //
            // this will then allow parallel processing
            // or we can use a mask?
            // https://pythonspeed.com/articles/optimizing-with-simd/
            //

            /*
            if self.is_static[id] || !self.is_enabled[id] {
                continue
            }*/

            unsafe {
                let velocity = *pos_ptr.offset(i) - *pos_prev_ptr.offset(i);
                let acceleration = *force_ptr.offset(i) / f32x4::from_array([(*mass_ptr.offset(i))[0], (*mass_ptr.offset(i))[0], (*mass_ptr.offset(i))[1], (*mass_ptr.offset(i))[1]]); //from_array([self.mass[id], self.mass[id]]);

                *pos_prev_ptr.offset(i) = *pos_ptr.offset(i);
                
                let new_pos = *pos_ptr.offset(i) + velocity + acceleration * delta_seconds_sqrd_simd;
    
                debug_assert!(!new_pos[0].is_nan());
                debug_assert!(!new_pos[1].is_nan());
                debug_assert!(!new_pos[3].is_nan());
                debug_assert!(!new_pos[4].is_nan());

                *pos_ptr.offset(i) = new_pos;
            }


            /*
            let pos = self.pos[i];
            let pos_prev = self.pos_prev[i];

            let velocity = pos - pos_prev;
            let acceleration = self.force[i] / f32x2::splat(self.mass[i]); //from_array([self.mass[id], self.mass[id]]);

            //println!("accel {}, vel {}", acceleration, velocity);

            self.pos_prev[i] = pos;
            let new_pos = pos + velocity + acceleration * delta_seconds_sqrd_f32x2;
            
            debug_assert!(!new_pos[0].is_nan());
            debug_assert!(!new_pos[1].is_nan());

            self.pos[i] = new_pos;
            */
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


#[cfg(test)]
mod tests {

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn alignment() {
        println!("size of Vec: {}", std::mem::size_of::<Vec<f32x2>>());
        println!("align of Vec: {}", std::mem::align_of::<Vec<f32x2>>());

        println!("size of ParticleVec: {}", std::mem::size_of::<ParticleVec>());
        println!("align of ParticleVec: {}", std::mem::align_of::<ParticleVec>());

        assert!(align_of::<Vec<f32x2>>() >= align_of::<f32x2>());
        assert!(align_of::<ParticleVec>() >= align_of::<f32x2>());
    }
}