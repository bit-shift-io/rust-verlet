use super::{particle::Particle, particle_data::ParticleData, particle_handle::ParticleHandle, spatial_hash_simd_particle_solver::SpatialHashSimdParticleSolver};


pub struct ParticleSystem {
    particle_data: ParticleData,
    solver: SpatialHashSimdParticleSolver,
}

impl ParticleSystem {

    pub fn add_particles(&mut self, particles: &Vec<Particle>) -> Vec<ParticleHandle>{
        let mut handles = self.particle_data.add_particles(particles);
        self.solver.notify_particle_data_changed(&mut self.particle_data);
        handles
    }

    pub fn solve_collisions(&mut self) {
        self.solver.solve_collisions(&mut self.particle_data);
    }
}

impl Default for ParticleSystem {
    fn default() -> Self {
        Self { 
            particle_data: ParticleData::default(),
            solver: SpatialHashSimdParticleSolver::default(),
        }
    }
}