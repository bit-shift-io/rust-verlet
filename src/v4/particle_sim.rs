use std::{cell::RefCell, rc::Rc};

use bevy::math::Vec2;

use super::{particle_container::ParticleContainer, particle_solvers::particle_solver::ParticleSolver};



pub struct ParticleSim {
    pub particle_container: Rc<RefCell<ParticleContainer>>,
    pub particle_solver: Box<dyn ParticleSolver>,
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


    fn notify_particle_container_changed(&mut self) {
        self.particle_solver.notify_particle_container_changed();
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

    use crate::v4::{particle::Particle, particle_solvers::{naive_particle_solver::NaiveParticleSolver, spatial_hash_particle_solver::SpatialHashParticleSolver}, shape_builder::shape_builder::{Circle, LineSegment, ShapeBuilder}};

    use super::*;

    extern crate test;
    use test::Bencher;

    // This lets us do some standard test on a solver to get some comparison
    fn run_sim_solver_test(sim: &mut ParticleSim) {
        // create some static shapes
        {
            let mut particle_container_mutref = sim.particle_container.as_ref().borrow_mut();
            let particle_container = &mut *particle_container_mutref;

            // static perimiter
            let mut b = ShapeBuilder::new();
            b.set_particle_template(Particle::default().set_static(true).clone());
            b.add_particles(&Circle::new(vec2(0.0, 0.0), 10.0));
            b.create_in_particle_container(particle_container);

            // some dynamic particles on the inside
            let mut b2 = ShapeBuilder::new();
            b2.add_particles(&LineSegment::new(vec2(-3.0, 0.0), vec2(3.0, 0.0)));
            b2.create_in_particle_container(particle_container);
        }

        // step the simulation 1 second forward in time
        sim.update(1.0);

        // print out the metrics to help us determine performance
        println!("num_collision_checks: {}", sim.particle_solver.as_ref().get_metrics().num_collision_checks);
    }

    #[test]
    fn naive_particle_solver_particle_sim() {
        let particle_container = Rc::new(RefCell::new(ParticleContainer::new()));
        let particle_solver = Box::new(NaiveParticleSolver::new());
        let mut sim = ParticleSim::new(&particle_container, particle_solver);
        run_sim_solver_test(&mut sim);
    }


    #[test]
    fn spatial_hash_particle_solver_particle_sim() {
        let particle_container = Rc::new(RefCell::new(ParticleContainer::new()));
        let particle_solver = Box::new(SpatialHashParticleSolver::new());
        let mut sim = ParticleSim::new(&particle_container, particle_solver);
        run_sim_solver_test(&mut sim);
    }

    #[bench]
    fn naive_particle_solver_particle_sim_bench(b: &mut Bencher) {
        b.iter(|| naive_particle_solver_particle_sim());
    }

    #[bench]
    fn spatial_hash_particle_solver_particle_sim_bench(b: &mut Bencher) {
        b.iter(|| spatial_hash_particle_solver_particle_sim());
    }
}