use super::{particle::Particle, particle_handle::ParticleHandle, particle_vec::ParticleVec};

pub struct ParticleData {
    pub static_particles: ParticleVec,
    pub dynamic_particles: ParticleVec,
    pub disabled_particles: ParticleVec,
    //pub particle_handles: ParticleHandle // todo: map from ParticleHandle to Particle
}

impl Default for ParticleData {
    fn default() -> Self {
        Self { 
            static_particles: ParticleVec::default(),
            dynamic_particles: ParticleVec::default(),
            disabled_particles: ParticleVec::default(),
        }
    }
}

impl ParticleData {

    pub fn add_particles(&mut self, particles: &Vec<Particle>) -> Vec<ParticleHandle> {
        // todo: handles support
        let mut handles = Vec::new();
        for p in particles {
            if !p.is_enabled {
                self.disabled_particles.add(*p);
            }
            else if p.is_static {
                self.static_particles.add(*p);
            }
            else {
                self.dynamic_particles.add(*p);
            }
        }
        handles
    }
}