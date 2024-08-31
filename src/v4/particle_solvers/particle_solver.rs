use std::{cell::RefCell, rc::Rc, sync::{Arc, RwLock}};

use bevy::math::Vec2;

use crate::v4::particle_container::ParticleContainer;

/// Compute which particle should move by how much if a and or b is static
#[inline(always)]
pub fn compute_movement_weight(a_is_static: bool, b_is_static: bool) -> (f32, f32) {
    // movement weight is used to stop static objects being moved
    let a_movement_weight = if a_is_static { 0.0f32 } else if b_is_static { 1.0f32 } else { 0.5f32 };
    let b_movement_weight = 1.0f32 - a_movement_weight;
    (a_movement_weight, b_movement_weight)
}

#[inline(always)]
pub fn update_particle_positions(particle_container: &mut ParticleContainer, delta_seconds: f32) {
    let mut i = 0;
    for particle in particle_container.particles.iter_mut() {
        /*
        if i == 65 {
            println!("65!");
        }*/
        if particle.is_static || !particle.is_enabled {
            continue
        }

        let velocity: Vec2 = particle.pos - particle.pos_prev;
        let acceleration: Vec2 = particle.force / particle.mass;

        //println!("accel {}, vel {}", acceleration, velocity);

        particle.pos_prev = particle.pos;
        particle.pos = particle.pos + velocity + acceleration * delta_seconds * delta_seconds;

        i += 1;
    }
}

pub trait ParticleSolver {
    fn attach_to_particle_container(&mut self, particle_container: &Arc<RwLock<ParticleContainer>>);

    fn notify_particle_container_changed(&mut self);

    fn reset_metrics(&mut self);

    fn update_particle_positions(&mut self, delta_seconds: f32);
/* 
    fn update_positions(&mut self, particle_accelerator: &mut ParticleContainer, dt: f32);
    */
    fn solve_collisions(&mut self);
    /*
    fn reset_forces(&mut self, particle_accelerator: &mut ParticleContainer, gravity: Vec2);
    */

    fn get_metrics(&self) -> &ParticleSolverMetrics;
}

#[derive(Default)]
pub struct ParticleSolverMetrics {
    pub num_collision_checks: usize
}