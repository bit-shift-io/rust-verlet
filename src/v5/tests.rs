

#[cfg(test)]
mod tests {
    use bevy::math::{bounding::Aabb2d, vec2};
    use bevy::prelude::default;
    use bevy::utils::HashSet;

    use crate::v5::naive_particle_solver::NaiveParticleSolver;
    use crate::v5::particle::Particle;
    use crate::v5::particle_vec::{ParticleVec, SharedParticleVec};

    use super::*;

    const TILE_SIZE: usize = 1;

    #[test]
    fn naive_particle_solver() {
        let mut solver = NaiveParticleSolver::default();
        let shared_particle_vec = SharedParticleVec::default();

        let mut particle_vec = shared_particle_vec.as_ref().write().unwrap();
        let ph_1 = particle_vec.add(*Particle::default().set_position(vec2(1.0, 0.0)));
        let ph_2 = particle_vec.add(Particle::default());

        solver.bind(&shared_particle_vec);
        solver.solve_collisions();

        let p_1 = particle_vec.get(ph_1).unwrap();
        let p_2 = particle_vec.get(ph_2).unwrap();

        assert_eq!(p_1.pos, vec2(1.0, 0.0));
        assert_eq!(p_2.pos, vec2(0.0, 0.0));
    }

}
