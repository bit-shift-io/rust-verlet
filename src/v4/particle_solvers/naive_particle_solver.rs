use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, RwLock};

use bevy::math::Vec2;

use super::super::particle_container::ParticleContainer;

use super::particle_solver::{compute_movement_weight, update_particle_positions, ParticleSolver, ParticleSolverMetrics};

pub struct NaiveParticleSolver {
    particle_container: Arc<RwLock<ParticleContainer>>,
    particle_solver_metrics: ParticleSolverMetrics,
}

impl NaiveParticleSolver {
    pub fn new() -> Self {
        Self { 
            particle_container: Arc::new(RwLock::new(ParticleContainer::new())),
            particle_solver_metrics: ParticleSolverMetrics::default(),
        }
    }
}

impl ParticleSolver for NaiveParticleSolver {
    fn attach_to_particle_container(&mut self, particle_container: &Arc<RwLock<ParticleContainer>>) {
        self.particle_container = particle_container.clone();
    }

    fn reset_metrics(&mut self) {
        self.particle_solver_metrics = ParticleSolverMetrics::default()
    }

    fn get_metrics(&self) -> &ParticleSolverMetrics {
        &self.particle_solver_metrics
    }

    fn update_particle_positions(&mut self, delta_seconds: f32) {
        update_particle_positions(&mut self.particle_container.as_ref().write().unwrap(), delta_seconds);
    }
    
    fn notify_particle_container_changed(&mut self/* , particle_container: &Rc<RefCell<ParticleContainer>>, particle_index: usize*/) {
    }

    fn solve_collisions(&mut self) {
        let mut particle_container = self.particle_container.as_ref().write().unwrap();

        // for each layer, we need to collide with each particle
        let particle_count: usize = particle_container.particles.len();
        for ai in 0..particle_count {
            for bi in (&ai+1)..particle_count {
                self.particle_solver_metrics.num_collision_checks += 1;

                let particle_a = particle_container.particles[ai];
                let particle_b = particle_container.particles[bi];

                // ignore static - static collisions
                if particle_a.is_static && particle_b.is_static {
                    continue;
                }

                // ignore disabled particles
                if !particle_a.is_enabled || !particle_b.is_enabled {
                    continue;
                }

                let (a_movement_weight, b_movement_weight) = compute_movement_weight(particle_a.is_static, particle_b.is_static);
                
                let collision_axis: Vec2;
                let dist: f32;
                let min_dist: f32;

                // in a code block so ap and bp borrows are released as we need to borrow mut later if
                // there is a collision
                {
                    //let ap = a_particle.as_ref().borrow();
                    //let bp = b_particle.as_ref().borrow();
                    let verlet_position_a = particle_a; //&particle_accelerator.verlet_positions[particle_id_a];
                    let verlet_position_b = particle_b; //&particle_accelerator.verlet_positions[particle_id_b];
                
                    collision_axis = verlet_position_a.pos - verlet_position_b.pos;
                    dist = (collision_axis[0].powf(2f32) + collision_axis[1].powf(2f32)).sqrt();
                    min_dist = particle_a.radius + particle_b.radius;
                }

                if dist < min_dist as f32 {
                    let n: Vec2 = collision_axis / dist;
                    let delta: f32 = min_dist as f32 - dist;

                    // is it better to have no if statement to make the loop tight at the cost
                    // of wasted vector computations?
                    //let mut ap_mut = a_particle.as_ref().borrow_mut();
                    let verlet_position_a = &mut particle_container.particles[ai]; //&mut particle_accelerator.verlet_positions[particle_id_a];
                    verlet_position_a.pos += a_movement_weight * delta * n;

                    //let mut bp_mut = b_particle.as_ref().borrow_mut();
                    let verlet_position_b = &mut particle_container.particles[bi]; //particle_b; //&mut particle_accelerator.verlet_positions[particle_id_b];
                    verlet_position_b.pos -= b_movement_weight * delta * n;
                }
            }
        }
    }
}