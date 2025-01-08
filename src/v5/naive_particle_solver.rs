use bevy::math::{vec2, Vec2};

use super::particle_solver::compute_movement_weight;
use super::particle_vec::SharedParticleVec;

pub struct NaiveParticleSolver {
    particle_vec_arc: SharedParticleVec,
}

impl Default for NaiveParticleSolver {
    fn default() -> Self {
        Self { 
            particle_vec_arc: SharedParticleVec::default(),
        }
    }
}

impl NaiveParticleSolver {
    pub fn bind(&mut self, particle_vec_arc: &SharedParticleVec) {
        self.particle_vec_arc = particle_vec_arc.clone();
    }

    pub fn solve_collisions(&mut self) {
        let mut particle_vec = self.particle_vec_arc.as_ref().write().unwrap();

        // for each layer, we need to collide with each particle
        let particle_count: usize = particle_vec.len();
        for ai in 0..particle_count {
            for bi in (&ai+1)..particle_count {

                // ignore static - static collisions
                if particle_vec.is_static[ai] && particle_vec.is_static[bi] {
                    continue;
                }

                // ignore disabled particles
                if !particle_vec.is_enabled[ai] || !particle_vec.is_enabled[bi] {
                    continue;
                }

                let (a_movement_weight, b_movement_weight) = compute_movement_weight(particle_vec.is_static[ai], particle_vec.is_static[bi]);
                
                let collision_axis: Vec2;
                let dist: f32;
                let min_dist: f32;

                // in a code block so ap and bp borrows are released as we need to borrow mut later if
                // there is a collision
                {
                    //let ap = a_particle.as_ref().borrow();
                    //let bp = b_particle.as_ref().borrow();
                    let verlet_position_a = particle_vec.get_pos_vec2(ai); //vec2(particle_vec.pos_x[ai], particle_vec.pos_y[ai]); //&particle_accelerator.verlet_positions[particle_id_a];
                    let verlet_position_b = particle_vec.get_pos_vec2(bi); //vec2(particle_vec.pos_x[bi], particle_vec.pos_y[bi]); //&particle_accelerator.verlet_positions[particle_id_b];
                
                    collision_axis = verlet_position_a - verlet_position_b;
                    dist = (collision_axis[0].powf(2f32) + collision_axis[1].powf(2f32)).sqrt();
                    min_dist = particle_vec.radius[ai][0] + particle_vec.radius[bi][0];
                }

                if dist < min_dist as f32 {
                    let n: Vec2 = collision_axis / dist;
                    let delta: f32 = min_dist as f32 - dist;

                    // is it better to have no if statement to make the loop tight at the cost
                    // of wasted vector computations?
                    //let mut ap_mut = a_particle.as_ref().borrow_mut();
                    let mut verlet_position_a = particle_vec.get_pos_vec2(ai); //vec2(particle_vec.pos_x[ai], particle_vec.pos_y[ai]); //&mut particle_accelerator.verlet_positions[particle_id_a];
                    verlet_position_a += a_movement_weight * delta * n;
                    debug_assert!(!verlet_position_a.x.is_nan());
                    debug_assert!(!verlet_position_a.y.is_nan());
                    particle_vec.set_pos_from_vec2(ai, &verlet_position_a);

                    //let mut bp_mut = b_particle.as_ref().borrow_mut();
                    let mut verlet_position_b = particle_vec.get_pos_vec2(bi); //vec2(particle_vec.pos_x[bi], particle_vec.pos_y[bi]); //particle_b; //&mut particle_accelerator.verlet_positions[particle_id_b];
                    verlet_position_b -= b_movement_weight * delta * n;
                    debug_assert!(!verlet_position_b.x.is_nan());
                    debug_assert!(!verlet_position_b.y.is_nan());
                    particle_vec.set_pos_from_vec2(bi, &verlet_position_b);
                }
            }
        }
    }
}