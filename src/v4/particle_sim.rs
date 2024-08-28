use std::{cell::RefCell, rc::Rc};

use bevy::math::Vec2;

use super::{particle_container::ParticleContainer, particle_solvers::particle_solver::ParticleSolver};



pub struct ParticleSim {
    particle_container: Rc<RefCell<ParticleContainer>>,
    particle_solver: Box<dyn ParticleSolver>,
    desired_hertz: f32,
    gravity: Vec2
}

impl ParticleSim {
    pub fn new(particle_container: &Rc<RefCell<ParticleContainer>>, mut particle_solver: Box<dyn ParticleSolver>) -> Self {
        particle_solver.as_mut().attach_to_particle_container(particle_container);
        Self {
            particle_container: particle_container.clone(),
            particle_solver,
            desired_hertz: 240.0,
            gravity: Vec2::new(0.0, -9.8),
        }
    }


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

    pub fn update(&mut self, delta_seconds: f32) {
        for sub_dt in Self::range_substeps(delta_seconds, self.desired_hertz).iter() {
            self.particle_solver.solve_collisions();
            /*
            collider.update_constraints(&mut car_scene.particle_accelerator, *sub_dt);
            collider.update_positions(&mut car_scene.particle_accelerator, *sub_dt);
            collider.post_update_constraints(&mut car_scene.particle_accelerator, *sub_dt);
            */
        }
    }
}


#[cfg(test)]
mod tests {
    use bevy::math::{vec2, Vec2};

    use crate::v4::{particle::Particle, particle_solvers::{naive_particle_solver::NaiveParticleSolver, spatial_hash_particle_solver::SpatialHashParticleSolver}};

    use super::*;

    #[test]
    fn naive_particle_solver_particle_sim() {
        // create a particle sim
        let particle_container = Rc::new(RefCell::new(ParticleContainer::new()));
        let particle_solver = Box::new(NaiveParticleSolver::new());
        let mut sim = ParticleSim::new(&particle_container, particle_solver);

        // add 2 particles so collision code can run
        particle_container.as_ref().borrow_mut().add(Particle::default());
        particle_container.as_ref().borrow_mut().add(*Particle::default().set_position(vec2(0.01, 0.0)));

        // step the simulation 1 second forward in time
        sim.update(1.0);
    }


    #[test]
    fn spatial_hash_particle_solver_particle_sim() {
        // create a particle sim
        let particle_container = Rc::new(RefCell::new(ParticleContainer::new()));
        let particle_solver = Box::new(SpatialHashParticleSolver::new());
        let mut sim = ParticleSim::new(&particle_container, particle_solver);

        // add 2 particles so collision code can run
        particle_container.as_ref().borrow_mut().add(Particle::default());
        particle_container.as_ref().borrow_mut().add(*Particle::default().set_position(vec2(0.01, 0.0)));

        // step the simulation 1 second forward in time
        sim.update(1.0);
    }
}