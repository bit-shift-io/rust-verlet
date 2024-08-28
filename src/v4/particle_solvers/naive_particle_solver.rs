use std::cell::RefCell;
use std::rc::Rc;

use bevy::math::Vec2;

use super::super::particle_container::ParticleContainer;

use super::particle_solver::ParticleSolver;

pub struct NaiveParticleSolver {
    particle_container: Rc<RefCell<ParticleContainer>>
}

impl NaiveParticleSolver {
    pub fn new() -> Self {
        Self { 
            particle_container: Rc::new(RefCell::new(ParticleContainer::new()))
        }
    }

    fn compute_movement_weight(a_is_static: bool, b_is_static: bool) -> (f32, f32) {
        // movement weight is used to stop static objects being moved
        let a_movement_weight = if a_is_static { 0.0f32 } else if b_is_static { 1.0f32 } else { 0.5f32 };
        let b_movement_weight = 1.0f32 - a_movement_weight;
        (a_movement_weight, b_movement_weight)
    }
}

impl ParticleSolver for NaiveParticleSolver {
    fn attach_to_particle_container(&mut self, particle_container: &Rc<RefCell<ParticleContainer>>) {
        self.particle_container = particle_container.clone();
    }

    fn solve_collisions(&mut self) {
        let mut particle_container = self.particle_container.as_ref().borrow_mut();

        // for each layer, we need to collide with each particle
        let particle_count: usize = particle_container.particles.len();
        for ai in 0..particle_count {
            for bi in (&ai+1)..particle_count {
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

                let (a_movement_weight, b_movement_weight) = Self::compute_movement_weight(particle_a.is_static, particle_b.is_static);
                
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