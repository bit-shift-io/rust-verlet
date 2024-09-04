use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, RwLock};

use bevy::math::bounding::BoundingVolume;
use bevy::math::vec2;

use crate::v4::particle;
use crate::v4::spatial_hash::SpatialHash;

use super::super::particle_container::ParticleContainer;

use super::particle_solver::{compute_movement_weight, update_particle_positions, ParticleSolver, ParticleSolverMetrics};

pub struct SpatialHashParticleSolver {
    particle_container: Arc<RwLock<ParticleContainer>>,
    particle_solver_metrics: ParticleSolverMetrics,
    static_spatial_hash: SpatialHash<usize>
}

impl SpatialHashParticleSolver {
    pub fn new() -> Self {
        Self { 
            particle_container: Arc::new(RwLock::new(ParticleContainer::new())),
            particle_solver_metrics: ParticleSolverMetrics::default(),
            static_spatial_hash: SpatialHash::<usize>::new(),
        }
    }
}

impl ParticleSolver for SpatialHashParticleSolver {
    fn attach_to_particle_container(&mut self, particle_container: &Arc<RwLock<ParticleContainer>>) {
        self.particle_container = particle_container.clone();
        self.notify_particle_container_changed();
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
        // rebuild the static spatial hash if a static particle was changed
        self.static_spatial_hash = SpatialHash::new();
        for (idx, particle) in self.particle_container.as_ref().read().unwrap().particles.iter().enumerate() {
            if particle.is_static && particle.is_enabled {
                self.static_spatial_hash.insert_aabb(particle.get_aabb(), idx);
            }
        }
    }

    fn solve_collisions(&mut self) {
        let grow_amount = vec2(2.0, 2.0); // this if like the maximum a particle should be able to move per frame - 2metres

        let mut particle_container = self.particle_container.as_ref().write().unwrap();        let particle_count: usize = particle_container.particles.len();

        // consider that there might be duplicate checks as an entity can be in multiple cells
        let mut collision_check = vec![0; particle_count];

        // perform dynamic-static collision detection
        for ai in 0..particle_count {
            let particle_a = particle_container.particles[ai];
            if !particle_a.is_static && particle_a.is_enabled {
                for bi in self.static_spatial_hash.aabb_iter(particle_a.get_aabb()) {
                    // avoid double checking against the same particle
                    if collision_check[bi] == ai {
                        //println!("static skipping collision check between {} and {}", bi, ai);
                        continue;
                    }
                    collision_check[bi] = ai;


                    self.particle_solver_metrics.num_collision_checks += 1;

                    let particle_b = particle_container.particles[bi];

                    // particle_a is dynamic while particle_b is static
                    let collision_axis = particle_a.pos - particle_b.pos;
                    let dist_squared = collision_axis.length_squared();
                    let min_dist = particle_a.radius + particle_b.radius;
                    let min_dist_squared = min_dist * min_dist;

                    if dist_squared < min_dist_squared {
                        let dist = f32::sqrt(dist_squared);
                        let n = collision_axis / dist;
                        let delta = min_dist - dist;
                        let movement = delta * n;

                        let mut_particle_a = &mut particle_container.particles[ai];
                        mut_particle_a.pos += movement;

                        // as the particle moves we need to move the aabb around
                        //dynamic_spatial_hash.insert_aabb(mut_particle_a.get_aabb(), ai);
                    }
                }
            }
        }

        let mut dynamic_spatial_hash = SpatialHash::<usize>::new();
        for ai in 0..particle_count {
            let particle_a = particle_container.particles[ai];
            if !particle_a.is_static && particle_a.is_enabled {
                dynamic_spatial_hash.insert_aabb(particle_a.get_aabb().grow(grow_amount), ai);
            }
        }
 
        // perform dynamic-dynamic collision detection
        for ai in 0..particle_count {
            let particle_a = particle_container.particles[ai];
            if !particle_a.is_static && particle_a.is_enabled {
                for bi in dynamic_spatial_hash.aabb_iter(particle_a.get_aabb()) {
                    // skip self-collision, and anything that is before ai
                    if bi <= ai {
                        continue;
                    }

                    // avoid double checking against the same particle
                    if collision_check[bi] == ai {
                        //println!("dynamic skipping collision check between {} and {}", bi, ai);
                        continue;
                    }
                    collision_check[bi] = ai;
                    

                    self.particle_solver_metrics.num_collision_checks += 1;

                    let particle_b = particle_container.particles[bi];

                    // particle_a and particle_b are both dynamic particles
                    let collision_axis = particle_a.pos - particle_b.pos;
                    let dist_squared = collision_axis.length_squared();
                    let min_dist = particle_a.radius + particle_b.radius;
                    let min_dist_squared = min_dist * min_dist;

                    if dist_squared < min_dist_squared {
                        let dist = f32::sqrt(dist_squared);
                        let n = collision_axis / dist;
                        let delta = min_dist - dist;
                        let movement = delta * 0.5 * n;


                        //println!("collision occured between particle_a and particle_b {} {}. min_dist: {}, dist: {}. mmovement: {}", ai, bi, min_dist, dist, movement);



                        {
                            let mut_particle_a = &mut particle_container.particles[ai];
                            mut_particle_a.pos += movement;

                            // as the particle moves we need to move the aabb around
                            //dynamic_spatial_hash.insert_aabb(mut_particle_a.get_aabb(), ai);
                        }

                        {
                            let mut_particle_b = &mut particle_container.particles[bi];
                            mut_particle_b.pos -= movement;

                            // as the particle moves we need to move the aabb around
                            //dynamic_spatial_hash.insert_aabb(mut_particle_b.get_aabb(), bi);
                        }
                    }
                }
            }
        }
    }
}