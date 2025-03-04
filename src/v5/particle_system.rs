use std::simd::f32x2;

use super::{particle::Particle, particle_data::ParticleData, particle_handle::ParticleHandle, particle_vec::ParticleVec, spatial_hash_simd_particle_solver::SpatialHashSimdParticleSolver};


// dt = last frame elapsed time
// desired_hertz = times per second
fn range_substeps(delta_seconds: f32, desired_hertz: f32) -> Vec<f32> {
    //let last_elapsed_secs = last_elapsed.as_secs_f32();
    let substeps: f32 = delta_seconds * desired_hertz as f32;
    let rounded_substeps = substeps.floor() as usize;
    let increment = 1.0 / desired_hertz;

    //println!("increment {}, rounded_substeps {}, delta_seconds {}", increment, rounded_substeps, delta_seconds);
    vec![increment; rounded_substeps]
}


pub struct ParticleSystem {
    pub particle_data: ParticleData,
    pub solver: SpatialHashSimdParticleSolver,
    desired_hertz: f32,
    gravity: f32x2
}

impl ParticleSystem {

    pub fn add_particles(&mut self, particles: &Vec<Particle>) -> Vec<ParticleHandle>{
        let mut handles = self.particle_data.add_particles(particles);
        self.solver.notify_particle_data_changed(&mut self.particle_data);
        handles
    }

    pub fn solve_collisions(&mut self) {
        //self.solver.solve_collisions(&mut self.particle_data);
        self.solver.solve_collisions_6(&mut self.particle_data);
    }

    pub fn pre_update(&mut self) {
        self.particle_data.dynamic_particles.reset_forces(self.gravity);
    }

    pub fn update(&mut self, delta_seconds: f32) {
        // disable sub steps for now, so we can see each frame
        //self.update_step(delta_seconds);
 
        let range = range_substeps(delta_seconds, self.desired_hertz);
        
        /*
        if range.len() > 0 {
            println!("update step: {}, # substeps: {}, sub step: {}", delta_seconds, range.len(), range[0]);
        }*/
        
        if range.len() > 5 {
            println!("[ParticleSystem.update] frame rate too low. Dropping some physics frames.");
            return;
        }

        for sub_dt in range.iter() {
            self.update_step(*sub_dt);
        }
    }

    pub fn update_step(&mut self, delta_seconds: f32) {
        //println!("delta_seconds: {}", delta_seconds);

        self.solve_collisions();

        self.particle_data.dynamic_particles.update_positions_3(delta_seconds);
        //self.particle_data.dynamic_particles.update_positions(delta_seconds);
        /* 
        self.constraint_solver.update_constraints(delta_seconds);
        self.particle_solver.update_particle_positions(delta_seconds);
        self.constraint_solver.post_update_constraints(delta_seconds);
        */
    }
}

impl Default for ParticleSystem {
    fn default() -> Self {
        Self {
            particle_data: ParticleData::default(),
            solver: SpatialHashSimdParticleSolver::default(),
            desired_hertz: 240.0,
            gravity: f32x2::from_array([0.0, -9.8]),
        }
    }
}