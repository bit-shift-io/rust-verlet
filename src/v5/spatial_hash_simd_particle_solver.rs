

use std::simd::f32x2;
use std::usize;

use bevy::math::bounding::{Aabb2d, BoundingVolume};
use bevy::math::{vec2, Vec2};

use super::aabb2d_ext::Aabb2dExt;
use super::particle_vec::SharedParticleVec;
use super::spatial_hash::SpatialHash;
use super::simd_ext::f32x2Ext;

/// This seems to be around 2x better than naive implementation
/// based on real world testing.
/// We should try Octree's in future also.
pub struct SpatialHashSimdParticleSolver {
    particle_vec_arc: SharedParticleVec,
    static_spatial_hash: SpatialHash<usize>
}

impl Default for SpatialHashSimdParticleSolver {
    fn default() -> Self {
        Self { 
            particle_vec_arc: SharedParticleVec::default(),
            static_spatial_hash: SpatialHash::<usize>::new(),
        }
    }
}

impl SpatialHashSimdParticleSolver {

    pub fn bind(&mut self, particle_vec_arc: &SharedParticleVec) {
        self.particle_vec_arc = particle_vec_arc.clone();
        self.notify_particle_vec_changed();
    }

    fn notify_particle_vec_changed(&mut self/* , particle_vec: &Rc<RefCell<ParticleContainer>>, particle_index: usize*/) {
        // rebuild the static spatial hash if a static particle was changed
        self.static_spatial_hash = SpatialHash::new();

        let particle_vec = self.particle_vec_arc.as_ref().read().unwrap();        
        let particle_count: usize = particle_vec.len();

        for ai in 0..particle_count {
            if particle_vec.is_static[ai] && particle_vec.is_enabled[ai] {
                //let a = Aabb::default();
                //let r = a.fabian_test();

                let a_aabb = Aabb2d::from_position_and_radius(particle_vec.get_pos_vec2(ai), particle_vec.radius[ai]);
                self.static_spatial_hash.insert_aabb(a_aabb, ai);
            }
        }
    }

    pub fn solve_collisions(&mut self) {
        let grow_amount = vec2(2.0, 2.0); // this if like the maximum a particle should be able to move per frame - 2metres

        let mut particle_vec = self.particle_vec_arc.as_ref().write().unwrap();        
        let particle_count: usize = particle_vec.len();

        // consider that there might be duplicate checks as an entity can be in multiple cells
        let mut collision_check = vec![usize::MAX; particle_count];

        // perform dynamic-static collision detection
        for ai in 0..particle_count {
            if !particle_vec.is_static[ai] && particle_vec.is_enabled[ai] {

                let a_aabb = Aabb2d::from_position_and_radius(particle_vec.get_pos_vec2(ai), particle_vec.radius[ai]);
                
                for bi in self.static_spatial_hash.aabb_iter(a_aabb) {
                    // avoid double checking against the same particle
                    if collision_check[bi] == ai {
                        //println!("static skipping collision check between {} and {}", bi, ai);
                        continue;
                    }
                    collision_check[bi] = ai;

                    let mut a_pos = particle_vec.pos[ai]; //vec2(particle_vec.pos_x[ai], particle_vec.pos_y[ai]);
                    let b_pos = particle_vec.pos[bi]; //vec2(particle_vec.pos_x[bi], particle_vec.pos_y[bi]);
                    
                    // particle_a is dynamic while particle_b is static
                    let collision_axis = a_pos - b_pos;
                    let dist_squared = collision_axis.length_squared();
                    let min_dist = particle_vec.radius[ai] + particle_vec.radius[bi];
                    let min_dist_squared = min_dist * min_dist;

                    if dist_squared < min_dist_squared {
                        let dist = f32::sqrt(dist_squared);
                        let n = collision_axis / f32x2::from_array([dist, dist]);
                        let delta = min_dist - dist;
                        let delta_f32x2 = f32x2::from_array([delta, delta]);
                        let movement = delta_f32x2 * n;

                        //let mut_particle_a = &mut particle_vec.particles[ai];
                        //mut_particle_a.pos += movement;

                        a_pos += movement;
                        //debug_assert!(!a_pos.x.is_nan());
                        //debug_assert!(!a_pos.y.is_nan());
                        particle_vec.pos[ai] = a_pos;

                        // as the particle moves we need to move the aabb around
                        //dynamic_spatial_hash.insert_aabb(mut_particle_a.get_aabb(), ai);
                    }
                }
            }
        }

        let mut dynamic_spatial_hash = SpatialHash::<usize>::new();
        for ai in 0..particle_count {
            if !particle_vec.is_static[ai] && particle_vec.is_enabled[ai] {
                let a_aabb = Aabb2d::from_position_and_radius(particle_vec.get_pos_vec2(ai), particle_vec.radius[ai]);
                dynamic_spatial_hash.insert_aabb(a_aabb.grow(grow_amount), ai);
            }
        }
 
        // perform dynamic-dynamic collision detection
        for ai in 0..particle_count {
            if !particle_vec.is_static[ai] && particle_vec.is_enabled[ai] {

                let a_aabb = Aabb2d::from_position_and_radius(particle_vec.get_pos_vec2(ai), particle_vec.radius[ai]);
                
                for bi in dynamic_spatial_hash.aabb_iter(a_aabb) {
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
                    

                    let mut a_pos = particle_vec.pos[ai]; //vec2(particle_vec.pos_x[ai], particle_vec.pos_y[ai]);
                    let mut b_pos = particle_vec.pos[bi]; //vec2(particle_vec.pos_x[bi], particle_vec.pos_y[bi]);
                    
                    //let particle_b = particle_vec.particles[bi];

                    // particle_a and particle_b are both dynamic particles
                    let collision_axis = a_pos - b_pos;
                    let dist_squared = collision_axis.length_squared();
                    let min_dist = particle_vec.radius[ai] + particle_vec.radius[bi];
                    let min_dist_squared = min_dist * min_dist;

                    if dist_squared < min_dist_squared {
                        let mut dist = f32::sqrt(dist_squared);

                        if dist <= 0.0 {
                            // dist is zero, but min_dist_squared might have some tiny value. If so, use that.
                            if min_dist_squared > 0.0 {
                                dist = min_dist_squared;
                            } else {
                                // oh dear! 2 particles at the same spot! give up and ignore it
                                // todo: move the particles towards the prev pos instead to make some distance between them?
                                continue;
                            }
                        }

                        let n = collision_axis / f32x2::from_array([dist, dist]);
                        let delta = min_dist - dist;
                        let delta_f32x2 = f32x2::from_array([delta * 0.5, delta * 0.5]);
                        let movement = delta_f32x2 * n;

                        //println!("movement {}, min_dist_squared {}, dist {}, n {}, delta {}, collision_axis {}", movement, min_dist_squared, dist, n, delta, collision_axis);
                        //debug_assert!(!movement.x.is_nan());
                        //debug_assert!(!movement.y.is_nan());

                        //println!("collision occured between particle_a and particle_b {} {}. min_dist: {}, dist: {}. mmovement: {}", ai, bi, min_dist, dist, movement);

                        {
                            //let mut_particle_a = &mut particle_vec.particles[ai];
                            //mut_particle_a.pos += movement;

                            a_pos += movement;
                            //debug_assert!(!a_pos.x.is_nan());
                            //debug_assert!(!a_pos.y.is_nan());
                            particle_vec.pos[ai] = a_pos;

                            // as the particle moves we need to move the aabb around
                            //dynamic_spatial_hash.insert_aabb(mut_particle_a.get_aabb(), ai);
                        }

                        {
                            b_pos -= movement;
                            //debug_assert!(!b_pos.x.is_nan());
                            //debug_assert!(!b_pos.y.is_nan());
                            particle_vec.pos[bi] = b_pos;

                            // as the particle moves we need to move the aabb around
                            //dynamic_spatial_hash.insert_aabb(mut_particle_b.get_aabb(), bi);
                        }
                    }
                }
            }
        }
    }
}