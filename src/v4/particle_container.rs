use super::{particle::Particle, particle_handle::ParticleHandle};

pub struct ParticleContainer {
    pub particles: Vec<Particle>,
}

impl ParticleContainer {
    pub fn new() -> Self {
        Self {
            particles: vec![],
        }
    }

    pub fn add(&mut self, particle: Particle) -> ParticleHandle {
        let id = self.particles.len();
        self.particles.push(particle);
        ParticleHandle::new(id) 
    }
}