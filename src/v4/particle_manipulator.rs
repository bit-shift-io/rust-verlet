use bevy::math::Vec2;

use super::{particle_handle::ParticleHandle, particle_sim::ParticleSim};


/**
 * Utility class to help with particle manipulation.
 */
pub struct ParticleManipulator {

}

impl ParticleManipulator {
    pub fn new() -> Self {
        Self{}
    }

    pub fn add_rotational_force_around_point(&self, particle_sim: &mut ParticleSim, particle_handles: &Vec<ParticleHandle>, pos: Vec2, force_magnitude: f32) {
        let particle_container = particle_sim.particle_container.as_ref().write().unwrap();

        for particle_handle in particle_handles.iter() {
            let mut particle = particle_container.particles[particle_handle.id()];
            let delta = particle.pos - pos;
            let adjacent = Vec2::new(-delta[1], delta[0]); // compute a vector at 90 degress to delta

            let force = adjacent * force_magnitude;
            println!("add force: {}", force);
            particle.add_force(force);
        }
    }
}