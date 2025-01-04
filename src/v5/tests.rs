

#[cfg(test)]
mod tests {
    use bevy::math::vec2;

    use crate::v5::spatial_hash_particle_solver::SpatialHashParticleSolver;
    use crate::v5::naive_particle_solver::NaiveParticleSolver;
    use crate::v5::particle::Particle;
    use crate::v5::particle_vec::SharedParticleVec;

    #[test]
    fn naive_particle_solver() {
        let mut solver = NaiveParticleSolver::default();
        let shared_particle_vec = SharedParticleVec::default();

        let (ph_1, ph_2) = {
            // particle_vec get released at the closing bracket
            let mut particle_vec = shared_particle_vec.as_ref().write().unwrap();
            let ph_1 = particle_vec.add(*Particle::default().set_position(vec2(0.9, 0.0)));
            let ph_2 = particle_vec.add(*Particle::default().set_static(true));
            (ph_1, ph_2)
        };

        solver.bind(&shared_particle_vec);
        solver.solve_collisions();

        let particle_vec = shared_particle_vec.as_ref().write().unwrap();
        let p_1 = particle_vec.get(ph_1).unwrap();
        let p_2 = particle_vec.get(ph_2).unwrap();

        assert_eq!(p_1.pos, vec2(1.0, 0.0));
        assert_eq!(p_2.pos, vec2(0.0, 0.0));
    }


    #[test]
    fn spatial_hash_particle_solver() {
        let mut solver = SpatialHashParticleSolver::default();
        let shared_particle_vec = SharedParticleVec::default();

        let (ph_1, ph_2) = {
            // particle_vec get released at the closing bracket
            let mut particle_vec = shared_particle_vec.as_ref().write().unwrap();
            let ph_1 = particle_vec.add(*Particle::default().set_position(vec2(0.9, 0.0)));
            let ph_2 = particle_vec.add(*Particle::default().set_static(true));
            (ph_1, ph_2)
        };

        solver.bind(&shared_particle_vec);
        solver.solve_collisions();
    
        let particle_vec = shared_particle_vec.as_ref().write().unwrap();
        let p_1 = particle_vec.get(ph_1).unwrap();
        let p_2 = particle_vec.get(ph_2).unwrap();

        assert_eq!(p_1.pos, vec2(1.0, 0.0));
        assert_eq!(p_2.pos, vec2(0.0, 0.0));
    }

}
