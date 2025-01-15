use std::simd::num::SimdFloat;
use std::simd::{f32x1, f32x2, f32x4, i32x1, i32x2, i32x4, StdFloat};
use std::usize;


use crate::v5::spatial_hash_simd_2::KeyIter;

use super::aabb_simd::AabbSimd;
use super::particle_data::ParticleData;
use super::particle_vec::{ParticleVec, SharedParticleVec};
use super::spatial_hash_simd::{SpatialHashSimd};
use super::simd_ext::f32x2Ext;
use super::spatial_hash_simd_2::SpatialHashSimd2;



// given a set of particles, iterate over each and generate a set of spatial hash keys for each particle index
#[inline(always)]
pub fn spatial_hash_keys_for_particles<F>(particles: &ParticleVec, mut func: F) 
where 
    F: FnMut(i32x2, usize)
{      
    let particle_count: usize = particles.len();

    // todo: grow amount should not be needed as I will split movement phase to occur after we compute collisions
    // i.e. phase 1. compute collisions and store movement vectors
    // phase 2. move the particles
    let grow_amount: f32x4 = f32x4::splat(2.0); // this if like the maximum a particle should be able to move per frame - 2metres

    // pointer to the start of the vector data
    let pos_ptr: *const f32x4 = particles.pos.as_ptr() as *const f32x4;
    let radius_ptr: *const f32x2 = particles.radius.as_ptr() as *const f32x2;
    
    let chunks = particle_count / 2;

    // todo: handle reminder for when we have an odd amount of particles!

    const TILE_SIZE: usize = 1;
    let tile_size_simd = f32x4::splat(TILE_SIZE as f32);

    for i in 0..chunks as isize {

        unsafe {

            // take 2 particles at a time
            // pos_simd has 2 positions packed in [p1.pos.x, p1.pos.y, p2.pos.x, p2.pos.y]
            // we setup radius_simd to have [p1.radius, p1.radius, p2.radius, p2.radius]
            let pos_simd = *pos_ptr.offset(i);
            let mut radius_simd = f32x4::from([(*radius_ptr.offset(i))[0], (*radius_ptr.offset(i))[0], (*radius_ptr.offset(i))[1], (*radius_ptr.offset(i))[1]]);
            radius_simd += grow_amount;

            // compute a bounding box using position and radius
            let min_f32 = pos_simd - radius_simd;
            let max_f32 = pos_simd + radius_simd;
            
            // divide by spatial has tile size and apply rounding to conver to "cell space"
            let min_i: i32x4  = (min_f32 / tile_size_simd).floor().cast(); //.into();
            let max_i: i32x4 = (max_f32 / tile_size_simd).ceil().cast(); //.into();

            // now we setup 2 iterators, one for p1 and one for p2
            //let diff_i = max_i - min_i;
            //let [width_p1, height_p1, width_p2, height_p2] = diff_i.to_array();

            /*
            let count_p1 = width_p1 * height_p1;
            let count_p2 = width_p2 * height_p2;

            let key_it_p1 = KeyIter {
                start: i32x2::from_array([min_i[0], min_i[1]]),
                current: -1,
                width: width_p1,
                count: count_p1,
            };

            let key_it_p2 = KeyIter {
                start: i32x2::from_array([min_i[2], min_i[3]]),
                current: -1,
                width: width_p2,
                count: count_p2,
            };*/

            // finally, for particle p1 and p2, use the iterators to add to spatial hash cells
            // this is the slow part of this algorithm
            let particle_idx: usize = (i * 2).try_into().unwrap();
            /*
            for key in key_it_p1 {
                //println!("key: {:?}", key);
                dynamic_spatial_hash.map.get_mut(&key).unwrap().push(particle_idx);
            }*/

            for y in min_i[1]..max_i[1] {
                for x in min_i[0]..max_i[0] {
                    let key = i32x2::from_array([x, y]);
                    func(key, particle_idx);
                    //self.dynamic_spatial_hash.map.entry(key).or_default().push(particle_idx);
                    //dynamic_spatial_hash.map.get_mut(&key).unwrap().push(particle_idx);
                }
            }

            for y in min_i[3]..max_i[3] {
                for x in min_i[2]..max_i[2] {
                    let key = i32x2::from_array([x, y]);
                    func(key, particle_idx + 1);
                    //self.dynamic_spatial_hash.map.entry(key).or_default().push(particle_idx + 1);
                    //dynamic_spatial_hash.map.get_mut(&key).unwrap().push(particle_idx + 1);
                }
            }

            /*
            for key in key_it_p2 {
                //println!("key: {:?}", key);
                dynamic_spatial_hash.map.get_mut(&key).unwrap().push(particle_idx + 1);
            }*/
        }
    }
}


/// This seems to be around 2x better than naive implementation
/// based on real world testing.
/// We should try Octree's in future also.
pub struct SpatialHashSimdParticleSolver {
    //pub particle_data: ParticleData,
    //pub particle_vec_arc: SharedParticleVec,
    pub static_spatial_hash: SpatialHashSimd2<usize>,
    pub dynamic_spatial_hash: SpatialHashSimd2<usize>,
}

impl Default for SpatialHashSimdParticleSolver {
    fn default() -> Self {
        Self { 
            //particle_data: ParticleData::default(),
            //particle_vec_arc: SharedParticleVec::default(),
            static_spatial_hash: SpatialHashSimd2::<usize>::new(),
            dynamic_spatial_hash: SpatialHashSimd2::<usize>::new(),
        }
    }
}

impl SpatialHashSimdParticleSolver {

    pub fn notify_particle_data_changed(&mut self, particle_data: &mut ParticleData) {
        // rebuild the static spatial hash if a static particle was changed
        self.static_spatial_hash = SpatialHashSimd2::new();

        let static_particles = &particle_data.static_particles;

        spatial_hash_keys_for_particles(static_particles, |key: i32x2, particle_idx: usize| {
            self.static_spatial_hash.map.entry(key).or_default().push(particle_idx);
        });
    }

    #[inline(always)]
    pub fn perform_dynamic_to_static_collision_detection(&mut self, particle_data: &mut ParticleData) {
        let dynamic_particles = &mut particle_data.dynamic_particles;  
        let static_particles = &particle_data.static_particles;      
        let particle_count: usize = dynamic_particles.len();

        // consider that there might be duplicate checks as a particle can be in multiple cells
        let mut collision_check = vec![usize::MAX; static_particles.len()];

        // perform dynamic-static collision detection
        for ai in 0..particle_count {
            let a_aabb = AabbSimd::from_position_and_radius(dynamic_particles.pos[ai], dynamic_particles.radius[ai][0]);
            
            for bi in self.static_spatial_hash.aabb_iter(&a_aabb) {
                // avoid double checking against the same particle
                if collision_check[bi] == ai {
                    //println!("static skipping collision check between {} and {}", bi, ai);
                    continue;
                }
                collision_check[bi] = ai;

                let mut a_pos = dynamic_particles.pos[ai]; //vec2(particle_vec.pos_x[ai], particle_vec.pos_y[ai]);
                let b_pos = static_particles.pos[bi]; //vec2(particle_vec.pos_x[bi], particle_vec.pos_y[bi]);
                
                // particle_a is dynamic while particle_b is static
                let collision_axis = a_pos - b_pos;
                let dist_squared = collision_axis.length_squared();
                let min_dist = dynamic_particles.radius[ai][0] + static_particles.radius[bi][0];
                let min_dist_squared = min_dist * min_dist;

                if dist_squared < min_dist_squared {
                    let dist = f32::sqrt(dist_squared);

                    // particles are too close to each other.
                    // just let them pass through each other
                    if dist <= f32::EPSILON {
                        println!("distance too small, skipping collision check {} and {}", bi, ai);
                        continue;
                    }

                    let n = collision_axis / f32x2::splat(dist); //::from_array([dist, dist]);
                    debug_assert!(!n[0].is_nan());
                    debug_assert!(!n[1].is_nan());

                    let delta = min_dist - dist;
                    let delta_f32x2 = f32x2::splat(delta); //from_array([delta, delta]);
                    let movement = delta_f32x2 * n;

                    //let mut_particle_a = &mut particle_vec.particles[ai];
                    //mut_particle_a.pos += movement;

                    debug_assert!(!movement[0].is_nan());
                    debug_assert!(!movement[1].is_nan());

                    a_pos += movement;
                    debug_assert!(!a_pos[0].is_nan());
                    debug_assert!(!a_pos[1].is_nan());
                    dynamic_particles.pos[ai] = a_pos;

                    // as the particle moves we need to move the aabb around
                    //dynamic_spatial_hash.insert_aabb(mut_particle_a.get_aabb(), ai);
                }
            }
        }
    }


    // this is terribly slow, why?
    // it is the same as perform_dynamic_to_static_collision_detection, except I have replaced the loops with spatial_hash_keys_for_particles
    #[inline(always)]
    pub fn perform_dynamic_to_static_collision_detection_2(&mut self, particle_data: &mut ParticleData) {
        let dynamic_particles = &mut particle_data.dynamic_particles;  
        let static_particles = &particle_data.static_particles;

        // consider that there might be duplicate checks as a particle can be in multiple cells
        let mut collision_check = vec![usize::MAX; static_particles.len()];
  
        let dynamic_pos_ptr: *mut f32x2 = dynamic_particles.pos.as_mut_ptr() as *mut f32x2;
        let dynamic_radius_ptr: *const f32x1 = dynamic_particles.radius.as_ptr() as *const f32x1;

        let static_pos_ptr: *const f32x2 = static_particles.pos.as_ptr() as *const f32x2;
        let static_radius_ptr: *const f32x1 = static_particles.radius.as_ptr() as *const f32x1;

        spatial_hash_keys_for_particles(dynamic_particles, |key: i32x2, dynamic_particle_idx: usize| {
            let dynamic_idx = dynamic_particle_idx as isize;

            let cell = self.static_spatial_hash.map.get(&key);
            match cell {
                Some(small_vec) => {
                    for &static_particle_idx in small_vec {
                        let static_idx = static_particle_idx as isize;

                        if collision_check[static_particle_idx] == dynamic_particle_idx {
                            //println!("static skipping collision check between {} and {}", static_idx, dynamic_idx);
                            return;
                        }
                        collision_check[static_particle_idx] = dynamic_particle_idx;

                        unsafe {
                            let collision_axis = *dynamic_pos_ptr.offset(dynamic_idx) - *static_pos_ptr.offset(static_idx);

                            let dist_squared = collision_axis.length_squared_1();

                            let min_dist = *dynamic_radius_ptr.offset(dynamic_idx) + *static_radius_ptr.offset(static_idx);
                            let min_dist_squared = min_dist * min_dist;

                            if dist_squared < min_dist_squared {
                                let dist = f32x1::sqrt(dist_squared);

                                // particles are too close to each other.
                                // just let them pass through each other
                                if dist[0] <= f32::EPSILON {
                                    return;
                                }

                                let n = collision_axis / f32x2::splat(dist[0]);
                                debug_assert!(!n[0].is_nan());
                                debug_assert!(!n[1].is_nan());

                                let delta = min_dist - dist;
                                let delta_f32x2 = f32x2::splat(delta[0]);
                                let movement = delta_f32x2 * n;

                                debug_assert!(!movement[0].is_nan());
                                debug_assert!(!movement[1].is_nan());

                                *dynamic_pos_ptr.offset(dynamic_idx) += movement;
                                debug_assert!(!(*dynamic_pos_ptr.offset(dynamic_idx))[0].is_nan());
                                debug_assert!(!(*dynamic_pos_ptr.offset(dynamic_idx))[1].is_nan());
                            }
                        }
                    }
                },
                None => {}
            }
        });
    }


    #[inline(always)]
    pub fn perform_dynamic_to_static_collision_detection_3(&mut self, particle_data: &mut ParticleData) {
        // todo: copy perform_dynamic_to_static_collision_detection
        // but instead check against 2 static particles at once...
    }

    /*
    #[inline(always)]
    pub fn populate_dynamic_spatial_hash(&mut self, dynamic_spatial_hash: &mut SpatialHashSimd<usize>) {
        let particle_vec = self.particle_vec_arc.as_ref().read().unwrap();        
        let particle_count: usize = particle_vec.len();

        let grow_amount: f32x1 = f32x1::splat(2.0); ///let grow_amount = vec2(2.0, 2.0); // this if like the maximum a particle should be able to move per frame - 2metres

        for ai in 0..particle_count {
            // todo: for now I disabled the is_Static and is_enabled checks to give us equal comparison between this and populate_dynamic_spatial_hash_2
            //if !particle_vec.is_static[ai] && particle_vec.is_enabled[ai] {
                let a_aabb = AabbSimd::from_position_and_radius(particle_vec.pos[ai], particle_vec.radius[ai][0] + grow_amount[0]);
                dynamic_spatial_hash.insert_aabb(&a_aabb, ai);
            //}
        }
    }

    // attempt to optimise with simd
    #[inline(always)]
    pub fn populate_dynamic_spatial_hash_2(&mut self, dynamic_spatial_hash: &mut SpatialHashSimd2<usize>) {
        let particle_vec = self.particle_vec_arc.as_ref().read().unwrap();        
        let particle_count: usize = particle_vec.len();

        // todo: grow amount should not be needed as I will split movement phase to occur after we compute collisions
        // i.e. phase 1. compute collisions and store movement vectors
        // phase 2. move the particles
        let grow_amount: f32x4 = f32x4::splat(2.0); // this if like the maximum a particle should be able to move per frame - 2metres

        // pointer to the start of the vector data
        let pos_ptr: *const f32x4 = particle_vec.pos.as_ptr() as *const f32x4;
        let radius_ptr: *const f32x2 = particle_vec.radius.as_ptr() as *const f32x2;
        
        let chunks = particle_count / 2;

        const TILE_SIZE: usize = 1;
        let tile_size_simd = f32x4::splat(TILE_SIZE as f32);

        for i in 0..chunks as isize {

            unsafe {

                // take 2 particles at a time
                // pos_simd has 2 positions packed in [p1.pos.x, p1.pos.y, p2.pos.x, p2.pos.y]
                // we setup radius_simd to have [p1.radius, p1.radius, p2.radius, p2.radius]
                let pos_simd = *pos_ptr.offset(i);
                let mut radius_simd = f32x4::from([(*radius_ptr.offset(i))[0], (*radius_ptr.offset(i))[0], (*radius_ptr.offset(i))[1], (*radius_ptr.offset(i))[1]]);
                radius_simd += grow_amount;

                // compute a bounding box using position and radius
                let min_f32 = pos_simd - radius_simd;
                let max_f32 = pos_simd + radius_simd;
                
                // divide by spatial has tile size and apply rounding to conver to "cell space"
                let min_i: i32x4  = (min_f32 / tile_size_simd).floor().cast(); //.into();
                let max_i: i32x4 = (max_f32 / tile_size_simd).ceil().cast(); //.into();

                // now we setup 2 iterators, one for p1 and one for p2
                let diff_i = max_i - min_i;
                let [width_p1, height_p1, width_p2, height_p2] = diff_i.to_array();
                let count_p1 = width_p1 * height_p1;
                let count_p2 = width_p2 * height_p2;

                let key_it_p1 = KeyIter {
                    start: i32x2::from_array([min_i[0], min_i[1]]),
                    current: -1,
                    width: width_p1,
                    count: count_p1,
                };

                let key_it_p2 = KeyIter {
                    start: i32x2::from_array([min_i[2], min_i[3]]),
                    current: -1,
                    width: width_p2,
                    count: count_p2,
                };

                // finally, for particle p1 and p2, use the iterators to add to spatial hash cells
                // this is the slow part of this algorithm
                let particle_idx: usize = (i * 2).try_into().unwrap();
                for key in key_it_p1 {
                    dynamic_spatial_hash.map.entry(key).or_default().push(particle_idx);
                }

                for key in key_it_p2 {
                    dynamic_spatial_hash.map.entry(key).or_default().push(particle_idx + 1);
                }
            }
        }
    }
    */


    // attempt to optimise with simd + trying to optimise hash map insertion
    #[inline(always)]
    pub fn populate_dynamic_spatial_hash_3(&mut self, particle_data: &mut ParticleData) {
        let dynamic_particles = &mut particle_data.dynamic_particles;       
        let particle_count: usize = dynamic_particles.len();

        // todo: grow amount should not be needed as I will split movement phase to occur after we compute collisions
        // i.e. phase 1. compute collisions and store movement vectors
        // phase 2. move the particles
        let grow_amount: f32x4 = f32x4::splat(2.0); // this if like the maximum a particle should be able to move per frame - 2metres

        // pointer to the start of the vector data
        let pos_ptr: *const f32x4 = dynamic_particles.pos.as_ptr() as *const f32x4;
        let radius_ptr: *const f32x2 = dynamic_particles.radius.as_ptr() as *const f32x2;
        
        let chunks = particle_count / 2;

        // todo: handle reminder for when we have an odd amount of particles!

        const TILE_SIZE: usize = 1;
        let tile_size_simd = f32x4::splat(TILE_SIZE as f32);

        for i in 0..chunks as isize {

            unsafe {

                // take 2 particles at a time
                // pos_simd has 2 positions packed in [p1.pos.x, p1.pos.y, p2.pos.x, p2.pos.y]
                // we setup radius_simd to have [p1.radius, p1.radius, p2.radius, p2.radius]
                let pos_simd = *pos_ptr.offset(i);
                let mut radius_simd = f32x4::from([(*radius_ptr.offset(i))[0], (*radius_ptr.offset(i))[0], (*radius_ptr.offset(i))[1], (*radius_ptr.offset(i))[1]]);
                radius_simd += grow_amount;

                // compute a bounding box using position and radius
                let min_f32 = pos_simd - radius_simd;
                let max_f32 = pos_simd + radius_simd;
                
                // divide by spatial has tile size and apply rounding to conver to "cell space"
                let min_i: i32x4  = (min_f32 / tile_size_simd).floor().cast(); //.into();
                let max_i: i32x4 = (max_f32 / tile_size_simd).ceil().cast(); //.into();

                // now we setup 2 iterators, one for p1 and one for p2
                //let diff_i = max_i - min_i;
                //let [width_p1, height_p1, width_p2, height_p2] = diff_i.to_array();

                /*
                let count_p1 = width_p1 * height_p1;
                let count_p2 = width_p2 * height_p2;

                let key_it_p1 = KeyIter {
                    start: i32x2::from_array([min_i[0], min_i[1]]),
                    current: -1,
                    width: width_p1,
                    count: count_p1,
                };

                let key_it_p2 = KeyIter {
                    start: i32x2::from_array([min_i[2], min_i[3]]),
                    current: -1,
                    width: width_p2,
                    count: count_p2,
                };*/

                // finally, for particle p1 and p2, use the iterators to add to spatial hash cells
                // this is the slow part of this algorithm
                let particle_idx: usize = (i * 2).try_into().unwrap();
                /*
                for key in key_it_p1 {
                    //println!("key: {:?}", key);
                    dynamic_spatial_hash.map.get_mut(&key).unwrap().push(particle_idx);
                }*/

                for y in min_i[1]..max_i[1] {
                    for x in min_i[0]..max_i[0] {
                        let key = i32x2::from_array([x, y]);
                        self.dynamic_spatial_hash.map.entry(key).or_default().push(particle_idx);
                        //dynamic_spatial_hash.map.get_mut(&key).unwrap().push(particle_idx);
                    }
                }

                for y in min_i[3]..max_i[3] {
                    for x in min_i[2]..max_i[2] {
                        let key = i32x2::from_array([x, y]);
                        self.dynamic_spatial_hash.map.entry(key).or_default().push(particle_idx + 1);
                        //dynamic_spatial_hash.map.get_mut(&key).unwrap().push(particle_idx + 1);
                    }
                }

                /*
                for key in key_it_p2 {
                    //println!("key: {:?}", key);
                    dynamic_spatial_hash.map.get_mut(&key).unwrap().push(particle_idx + 1);
                }*/
            }
        }
    }


    // attempt to optimise with simd + trying to optimise hash map insertion
    #[inline(always)]
    pub fn populate_dynamic_spatial_hash_4(&mut self, particle_data: &mut ParticleData) {
        let dynamic_particles = &particle_data.dynamic_particles;   
        spatial_hash_keys_for_particles(dynamic_particles, |key: i32x2, particle_idx: usize| {
            self.dynamic_spatial_hash.map.entry(key).or_default().push(particle_idx);
        });
    }



    #[inline(always)]
    pub fn perform_dynamic_to_dynamic_collision_detection(&mut self, particle_data: &mut ParticleData) {
        let dynamic_particles = &mut particle_data.dynamic_particles;      
        let particle_count: usize = dynamic_particles.len();

        // consider that there might be duplicate checks as a particle can be in multiple cells
        let mut collision_check = vec![usize::MAX; dynamic_particles.len()];

        // perform dynamic-dynamic collision detection
        for ai in 0..particle_count {
            let a_aabb = AabbSimd::from_position_and_radius(dynamic_particles.pos[ai], dynamic_particles.radius[ai][0]);
            
            for bi in self.dynamic_spatial_hash.aabb_iter(&a_aabb) {
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
                

                let mut a_pos = dynamic_particles.pos[ai]; //vec2(particle_vec.pos_x[ai], particle_vec.pos_y[ai]);
                let mut b_pos = dynamic_particles.pos[bi]; //vec2(particle_vec.pos_x[bi], particle_vec.pos_y[bi]);
                
                //let particle_b = particle_vec.particles[bi];

                // particle_a and particle_b are both dynamic particles
                let collision_axis = a_pos - b_pos;
                let dist_squared = collision_axis.length_squared();
                let min_dist = dynamic_particles.radius[ai][0] + dynamic_particles.radius[bi][0];
                let min_dist_squared = min_dist * min_dist;

                if dist_squared < min_dist_squared {
                    let mut dist = f32::sqrt(dist_squared);


                    // particles are too close to each other.
                    // just let them pass through each other
                    if dist <= f32::EPSILON {
                        continue;
                    }
                    /*
                    if dist <= 0.0 {
                        // dist is zero, but min_dist_squared might have some tiny value. If so, use that.
                        if min_dist_squared > 0.0 {
                            dist = min_dist_squared;
                        } else {
                            // oh dear! 2 particles at the same spot! give up and ignore it
                            // todo: move the particles towards the prev pos instead to make some distance between them?
                            continue;
                        }
                    }*/

                    let n = collision_axis / f32x2::splat(dist); //from_array([dist, dist]);
                    let delta = min_dist - dist;
                    let delta_f32x2 = f32x2::splat(delta * 0.5); //from_array([delta * 0.5, delta * 0.5]);
                    let movement = delta_f32x2 * n;

                    //println!("movement {}, min_dist_squared {}, dist {}, n {}, delta {}, collision_axis {}", movement, min_dist_squared, dist, n, delta, collision_axis);
                    //debug_assert!(!movement.x.is_nan());
                    //debug_assert!(!movement.y.is_nan());

                    //println!("collision occured between particle_a and particle_b {} {}. min_dist: {}, dist: {}. mmovement: {}", ai, bi, min_dist, dist, movement);

                    {
                        //let mut_particle_a = &mut particle_vec.particles[ai];
                        //mut_particle_a.pos += movement;

                        a_pos += movement;
                        debug_assert!(!a_pos[0].is_nan());
                        debug_assert!(!a_pos[1].is_nan());
                        dynamic_particles.pos[ai] = a_pos;

                        // as the particle moves we need to move the aabb around
                        //dynamic_spatial_hash.insert_aabb(mut_particle_a.get_aabb(), ai);
                    }

                    {
                        b_pos -= movement;
                        debug_assert!(!b_pos[0].is_nan());
                        debug_assert!(!b_pos[1].is_nan());
                        dynamic_particles.pos[bi] = b_pos;

                        // as the particle moves we need to move the aabb around
                        //dynamic_spatial_hash.insert_aabb(mut_particle_b.get_aabb(), bi);
                    }
                }
            }
        }
    }

    pub fn solve_collisions(&mut self, particle_data: &mut ParticleData) {
        self.perform_dynamic_to_static_collision_detection(particle_data);

        self.dynamic_spatial_hash.soft_clear();
        self.populate_dynamic_spatial_hash_4(particle_data);
       
        self.perform_dynamic_to_dynamic_collision_detection(particle_data);
    }
}