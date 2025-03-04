use std::collections::HashSet;
use std::fs::File;
use std::io::Write;
use std::simd::num::SimdFloat;
use std::simd::{f32x1, f32x2, f32x4, i32x1, i32x2, i32x4, StdFloat};
use std::usize;

use itertools::Itertools;

use smallvec::SmallVec;
use sorted_vec::SortedSet;

use crate::v5::simd_ext::f32x4Ext;
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
    //let grow_amount: f32x4 = f32x4::splat(2.0); // this if like the maximum a particle should be able to move per frame - 2metres

    // pointer to the start of the vector data
    let pos_ptr: *const f32x4 = particles.pos.as_ptr() as *const f32x4;
    let radius_ptr: *const f32x2 = particles.radius.as_ptr() as *const f32x2;
    
    let chunks = particle_count / 2;
    let remainder = particle_count - (chunks * 2);

    const TILE_SIZE: usize = 1;
    let tile_size_simd = f32x4::splat(TILE_SIZE as f32);

    for i in 0..chunks as isize {
        unsafe {
            // take 2 particles at a time
            // pos_simd has 2 positions packed in [p1.pos.x, p1.pos.y, p2.pos.x, p2.pos.y]
            // we setup radius_simd to have [p1.radius, p1.radius, p2.radius, p2.radius]
            let pos_simd = *pos_ptr.offset(i);
            let radius_simd = f32x4::from([(*radius_ptr.offset(i))[0], (*radius_ptr.offset(i))[0], (*radius_ptr.offset(i))[1], (*radius_ptr.offset(i))[1]]);
            //radius_simd += grow_amount;

            // compute a bounding box using position and radius
            let min_f32 = pos_simd - radius_simd;
            let max_f32 = pos_simd + radius_simd;
            
            // divide by spatial has tile size and apply rounding to conver to "cell space"
            let min_i: i32x4  = (min_f32 / tile_size_simd).floor().cast(); //.into();
            let max_i: i32x4 = (max_f32 / tile_size_simd).ceil().cast(); //.into();

            //println!("i: {}, pos: {}, radius: {}, min_f32: {}, max_f32: {}", i, pos_simd, radius_simd, min_f32, max_f32);

            // finally, for particle p1 and p2, use the iterators to add to spatial hash cells
            // this is the slow part of this algorithm
            let particle_idx: usize = (i * 2).try_into().unwrap();

            for y in min_i[1]..max_i[1] {
                for x in min_i[0]..max_i[0] {
                    let key = i32x2::from_array([x, y]);
                    func(key, particle_idx);
                }
            }

            for y in min_i[3]..max_i[3] {
                for x in min_i[2]..max_i[2] {
                    let key = i32x2::from_array([x, y]);
                    func(key, particle_idx + 1);
                }
            }
        }
    }

    // handle the remainders
    {
        let pos_ptr: *const f32x2 = particles.pos.as_ptr() as *const f32x2;
        let radius_ptr: *const f32x1 = particles.radius.as_ptr() as *const f32x1;

        let tile_size_simd = f32x2::splat(TILE_SIZE as f32);
        for ui in (particle_count-remainder)..particle_count {
            let i = ui as isize;

            unsafe {
                // take 1 particles at a time
                // pos_simd has 1 positions packed in [p1.pos.x, p1.pos.y]
                // we setup radius_simd to have [p1.radius, p1.radius]
                let pos_simd = *pos_ptr.offset(i);
                let radius_simd = f32x2::from([(*radius_ptr.offset(i))[0], (*radius_ptr.offset(i))[0]]);
                //radius_simd += grow_amount;
    
                // compute a bounding box using position and radius
                let min_f32 = pos_simd - radius_simd;
                let max_f32 = pos_simd + radius_simd;
                
                // divide by spatial has tile size and apply rounding to conver to "cell space"
                let min_i: i32x2  = (min_f32 / tile_size_simd).floor().cast(); //.into();
                let max_i: i32x2 = (max_f32 / tile_size_simd).ceil().cast(); //.into();
    
                // finally, for particle p1, use the iterators to add to spatial hash cells
                // this is the slow part of this algorithm
                if ui >= particle_count {
                    println!("spatial_hash_keys_for_particles error. particle idx: {} (# particles {})", ui, particle_count);
                }
                debug_assert!(ui < particle_count);
    
                for y in min_i[1]..max_i[1] {
                    for x in min_i[0]..max_i[0] {
                        let key = i32x2::from_array([x, y]);
                        func(key, ui);
                    }
                }
            }
        }
    }
}


// given a set of particles, iterate over each and generate a set of spatial hash keys for each particle index
#[inline(always)]
pub fn spatial_hash_keys_for_particles_keys<F>(particles: &ParticleVec, mut func: F) 
where 
    F: FnMut(usize, &SmallVec::<[i32x2; 100]>)
{      
    let particle_count: usize = particles.len();

    // todo: grow amount should not be needed as I will split movement phase to occur after we compute collisions
    // i.e. phase 1. compute collisions and store movement vectors
    // phase 2. move the particles
    //let grow_amount: f32x4 = f32x4::splat(2.0); // this if like the maximum a particle should be able to move per frame - 2metres

    // pointer to the start of the vector data
    let pos_ptr: *const f32x4 = particles.pos.as_ptr() as *const f32x4;
    let radius_ptr: *const f32x2 = particles.radius.as_ptr() as *const f32x2;
    
    let chunks = particle_count / 2;
    let remainder = particle_count - (chunks * 2);

    const TILE_SIZE: usize = 1;
    let tile_size_simd = f32x4::splat(TILE_SIZE as f32);

     
    for i in 0..chunks as isize {
        unsafe {
            // take 2 particles at a time
            // pos_simd has 2 positions packed in [p1.pos.x, p1.pos.y, p2.pos.x, p2.pos.y]
            // we setup radius_simd to have [p1.radius, p1.radius, p2.radius, p2.radius]
            let pos_simd = *pos_ptr.offset(i);
            let radius_simd = f32x4::from([(*radius_ptr.offset(i))[0], (*radius_ptr.offset(i))[0], (*radius_ptr.offset(i))[1], (*radius_ptr.offset(i))[1]]);
            //radius_simd += grow_amount;

            // compute a bounding box using position and radius
            let min_f32 = pos_simd - radius_simd;
            let max_f32 = pos_simd + radius_simd;
            
            // divide by spatial has tile size and apply rounding to conver to "cell space"
            let min_i: i32x4  = (min_f32 / tile_size_simd).floor().cast(); //.into();
            let max_i: i32x4 = (max_f32 / tile_size_simd).ceil().cast(); //.into();

            // finally, for particle p1 and p2, use the iterators to add to spatial hash cells
            // this is the slow part of this algorithm
            let particle_idx: usize = (i * 2).try_into().unwrap();
            debug_assert!(particle_idx < particle_count);

            {
                let mut keys = SmallVec::<[i32x2; 100]>::new();
                for y in min_i[1]..max_i[1] {
                    for x in min_i[0]..max_i[0] {
                        let key = i32x2::from_array([x, y]);
                        keys.push(key);
                    }
                }
                func(particle_idx, &keys)
            }

            {
                let mut keys = SmallVec::<[i32x2; 100]>::new();
                for y in min_i[3]..max_i[3] {
                    for x in min_i[2]..max_i[2] {
                        let key = i32x2::from_array([x, y]);
                        keys.push(key);
                    }
                }
                func(particle_idx + 1, &keys);
            }
        }
    }

    // handle the remainders
    {
        let pos_ptr: *const f32x2 = particles.pos.as_ptr() as *const f32x2;
        let radius_ptr: *const f32x1 = particles.radius.as_ptr() as *const f32x1;

        let tile_size_simd = f32x2::splat(TILE_SIZE as f32);
        for ui in (particle_count-remainder)..particle_count {
            let i = ui as isize;

            unsafe {
                // take 2 particles at a time
                // pos_simd has 2 positions packed in [p1.pos.x, p1.pos.y, p2.pos.x, p2.pos.y]
                // we setup radius_simd to have [p1.radius, p1.radius, p2.radius, p2.radius]
                let pos_simd = *pos_ptr.offset(i);
                let radius_simd = f32x2::from([(*radius_ptr.offset(i))[0], (*radius_ptr.offset(i))[0]]);
                //radius_simd += grow_amount;
    
                // compute a bounding box using position and radius
                let min_f32 = pos_simd - radius_simd;
                let max_f32 = pos_simd + radius_simd;
                
                // divide by spatial has tile size and apply rounding to conver to "cell space"
                let min_i: i32x2  = (min_f32 / tile_size_simd).floor().cast(); //.into();
                let max_i: i32x2 = (max_f32 / tile_size_simd).ceil().cast(); //.into();
    
                // finally, for particle p1 and p2, use the iterators to add to spatial hash cells
                // this is the slow part of this algorithm
                debug_assert!(ui < particle_count);

                {
                    let mut keys = SmallVec::<[i32x2; 100]>::new();
                    for y in min_i[1]..max_i[1] {
                        for x in min_i[0]..max_i[0] {
                            let key = i32x2::from_array([x, y]);
                            keys.push(key);
                        }
                    }
                    func(ui, &keys)
                }
            }
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
    pub frame: usize,
    pub file: File,
}

impl Default for SpatialHashSimdParticleSolver {
    fn default() -> Self {

        let mut file = File::create("output.csv").expect("Couldn't create file");
        file.write_all(format!("frame; d-?; idx_0; idx_1; pos_0; pos_1; dist; min_dist_squared; dist_squared; delta(overlap); movement; n(dir)\n").as_bytes()).unwrap();

        Self { 
            //particle_data: ParticleData::default(),
            //particle_vec_arc: SharedParticleVec::default(),
            static_spatial_hash: SpatialHashSimd2::<usize>::new(),
            dynamic_spatial_hash: SpatialHashSimd2::<usize>::new(),
            frame: 0,
            file: file
        }
    }
}

impl SpatialHashSimdParticleSolver {

    pub fn notify_particle_data_changed(&mut self, particle_data: &mut ParticleData) {
        // rebuild the static spatial hash if a static particle was changed
        self.static_spatial_hash = SpatialHashSimd2::new();

        let static_particles = &particle_data.static_particles;

        println!("static particles: {:?}", static_particles.len());

        spatial_hash_keys_for_particles(static_particles, |key: i32x2, particle_idx: usize| {
            debug_assert!(particle_idx < static_particles.len());
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
                        debug_assert!(static_particle_idx < static_particles.len());

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


    // there are a few ways to try to get fastest performance for finding which particles collide
    // option 1. static hash and dynamic hash seperately. static is only regenerated when a static particle is changed
    //      then we need to search through 2 spatial hashes for colliding particles
    //
    // option 2. a single spatial hash just add all static and dynamic hashes every frame, then search through 1 spatial has
    // option 3. a static hash, computed on change, and a clone is made then dynamic is added on top, so only 1 spatial hash is searched through
    //
    // on top of this we need to consider how to most efficiently iterate to find colliders
    //

    #[inline(always)]
    pub fn find_colliders_option_1_old_iteration(&mut self, particle_data: &mut ParticleData) {
        // option 1. static hash and dynamic hash seperately. static is only regenerated when a static particle is changed
        //      then we need to search through 2 spatial hashes for colliding particles

        self.dynamic_spatial_hash.soft_clear();
        self.populate_dynamic_spatial_hash_4(particle_data);

        {
            let dynamic_particles = &mut particle_data.dynamic_particles;  
            let static_particles = &particle_data.static_particles;      
            let particle_count: usize = dynamic_particles.len();

            // consider that there might be duplicate checks as a particle can be in multiple cells
            //let mut collision_check = vec![usize::MAX; static_particles.len()];

            // perform dynamic-static collision detection
            for ai in 0..particle_count {
                let a_aabb = AabbSimd::from_position_and_radius(dynamic_particles.pos[ai], dynamic_particles.radius[ai][0]);
                
                let mut static_indicies = SmallVec::<[usize; 100]>::new();

                for bi in self.static_spatial_hash.aabb_iter(&a_aabb) {
                    static_indicies.push(bi);
                }

                //println!("static_indicies: {}", static_indicies.len());
            }
        }

        {
            let dynamic_particles = &mut particle_data.dynamic_particles;      
            let particle_count: usize = dynamic_particles.len();

            // consider that there might be duplicate checks as a particle can be in multiple cells
            //let mut collision_check = vec![usize::MAX; dynamic_particles.len()];

            // perform dynamic-dynamic collision detection
            for ai in 0..particle_count {
                let a_aabb = AabbSimd::from_position_and_radius(dynamic_particles.pos[ai], dynamic_particles.radius[ai][0]);
                
                let mut dynamic_indicies = SmallVec::<[usize; 100]>::new();

                for bi in self.dynamic_spatial_hash.aabb_iter(&a_aabb) {
                    dynamic_indicies.push(bi);
                }

                //println!("dynamic_indicies: {}", dynamic_indicies.len());
            }
        }
    }

    #[inline(always)]
    pub fn find_colliders_option_1_new_iteration(&mut self, particle_data: &mut ParticleData) {
        // option 1. static hash and dynamic hash seperately. static is only regenerated when a static particle is changed
        //      then we need to search through 2 spatial hashes for colliding particles

        self.dynamic_spatial_hash.soft_clear();
        self.populate_dynamic_spatial_hash_4(particle_data);

        let dynamic_particles = &mut particle_data.dynamic_particles;

        spatial_hash_keys_for_particles_keys(dynamic_particles, |dynamic_particle_idx: usize, keys: &SmallVec::<[i32x2; 100]>| {
            let mut static_indicies = SmallVec::<[usize; 100]>::new();
            let mut dynamic_indicies = SmallVec::<[usize; 100]>::new();

            let static_it = keys.iter()
                .filter_map(|key| self.static_spatial_hash.map.get(key))
                .flatten()
                .copied();

            for static_particle_idx in static_it {
                static_indicies.push(static_particle_idx);
            }

            let dynamic_it = keys.iter()
                .filter_map(|key| self.dynamic_spatial_hash.map.get(key))
                .flatten()
                .copied();

            for dynamic_particle_idx in dynamic_it {
                dynamic_indicies.push(dynamic_particle_idx);
            }

            //println!("static_indicies: {}", static_indicies.len());
            //println!("dynamic_indicies: {}", dynamic_indicies.len());
        });
    }


    // This is the fastest method!
    // There doesn't seem to be any benifit to seperating dynamic and static particles
    // unless I spent time coming up with some sort of linked list spatial hash structure that might make 
    // "cloning" a hash map quicker.
    #[inline(always)]
    pub fn find_colliders_option_2(&mut self, particle_data: &mut ParticleData) {
        // option 2. a single spatial hash just add all static and dynamic hashes every frame, then search through 1 spatial has

        self.dynamic_spatial_hash.soft_clear();
        self.populate_dynamic_spatial_hash_4(particle_data);

        // add static particles to hash
        let static_particles = &particle_data.static_particles;   
        spatial_hash_keys_for_particles(static_particles, |key: i32x2, particle_idx: usize| {
            self.dynamic_spatial_hash.map.entry(key).or_default().push(particle_idx);
        });

        let dynamic_particles = &mut particle_data.dynamic_particles;

        spatial_hash_keys_for_particles_keys(dynamic_particles, |dynamic_particle_idx: usize, keys: &SmallVec::<[i32x2; 100]>| {
            let mut dynamic_indicies = SmallVec::<[usize; 100]>::new();

            let dynamic_it = keys.iter()
                .filter_map(|key| self.dynamic_spatial_hash.map.get(key))
                .flatten()
                .copied();

            for dynamic_particle_idx in dynamic_it {
                dynamic_indicies.push(dynamic_particle_idx);
            }

            //println!("static_indicies: {}", static_indicies.len());
            //println!("dynamic_indicies: {}", dynamic_indicies.len());
        });
    }


    #[inline(always)]
    pub fn find_colliders_option_3(&mut self, particle_data: &mut ParticleData) {
        // option 3. a static hash, computed on change, and a clone is made then dynamic is added on top, so only 1 spatial hash is searched through

        self.dynamic_spatial_hash = self.static_spatial_hash.clone();
        self.populate_dynamic_spatial_hash_4(particle_data);

        let dynamic_particles = &mut particle_data.dynamic_particles;

        spatial_hash_keys_for_particles_keys(dynamic_particles, |dynamic_particle_idx: usize, keys: &SmallVec::<[i32x2; 100]>| {
            let mut dynamic_indicies = SmallVec::<[usize; 100]>::new();

            let dynamic_it = keys.iter()
                .filter_map(|key| self.dynamic_spatial_hash.map.get(key))
                .flatten()
                .copied();

            for dynamic_particle_idx in dynamic_it {
                dynamic_indicies.push(dynamic_particle_idx);
            }

            //println!("static_indicies: {}", static_indicies.len());
            //println!("dynamic_indicies: {}", dynamic_indicies.len());
        });
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
        //let grow_amount: f32x4 = f32x4::splat(2.0); // this if like the maximum a particle should be able to move per frame - 2metres

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
                //radius_simd += grow_amount;

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
        // 2.0ms
        self.perform_dynamic_to_static_collision_detection(particle_data);

        // 3.7ms
        self.dynamic_spatial_hash.soft_clear();
        self.populate_dynamic_spatial_hash_4(particle_data);
        
        // 7.2ms
        self.perform_dynamic_to_dynamic_collision_detection(particle_data);
    }

    // Trying to keep seperate static and dynamic particles, while processing 2 particles at once where possible.
    // for some reason the basic solve_collisions is still faster than this... need to investigate why
    // also processing 1 particle at once seems faster as well, do might need to try a new variation there also....
    //
    // faster than solve_collisions, but still need to apply particle movement
    //
    // TODO: it feels like a lot of time is wasted just iterating here, trying to build pairs in order to use simd
    // so why not just keep it simple, and don't try to process multiple particles at once?
    //
    pub fn solve_collisions_5(&mut self, particle_data: &mut ParticleData) {
        let dynamic_particles = &mut particle_data.dynamic_particles;

        // setup the spatial hash
        // 3.7ms
        self.dynamic_spatial_hash.soft_clear();
           
        spatial_hash_keys_for_particles(dynamic_particles, |key: i32x2, particle_idx: usize| {
            self.dynamic_spatial_hash.map.entry(key).or_default().push(particle_idx);
        });
        //

        let pos_ptr: *const f32x2 = dynamic_particles.pos.as_ptr() as *const f32x2;
        let radius_ptr: *const f32x1 = dynamic_particles.radius.as_ptr() as *const f32x1;
        let movement_ptr: *mut f32x2 = dynamic_particles.movement.as_mut_ptr() as *mut f32x2;

        let static_particles = &mut particle_data.static_particles; 
        let static_pos_ptr: *const f32x2 = static_particles.pos.as_ptr() as *const f32x2;
        let static_radius_ptr: *const f32x1 = static_particles.radius.as_ptr() as *const f32x1;

        // iterate over each dynamic particle
        spatial_hash_keys_for_particles_keys(dynamic_particles, |uidx_0: usize, keys: &SmallVec::<[i32x2; 100]>| {
            let idx_0 = uidx_0 as isize;

            let pos_0 = unsafe {
                f32x4::from_array([(*pos_ptr.offset(idx_0))[0], (*pos_ptr.offset(idx_0))[1], (*pos_ptr.offset(idx_0))[0], (*pos_ptr.offset(idx_0))[1]])
            };

            let radius_0 = unsafe {
                f32x4::splat((*radius_ptr.offset(idx_0))[0])
            };

            // dynamic particle collisions
            // 7.1 ms
            {
                // if we move this outside the loop "spatial_hash_keys_for_particles_keys" we gain 1ms, but
                // that won't work when we go multithreaded!
                let mut particle_idxs_set = SortedSet::<usize>::with_capacity(20); //new();
                for i in 0..keys.len() {
                    let entry = self.dynamic_spatial_hash.map.get(&keys[i]);
                    match entry {
                        Some(particle_idxs) => {
                            for p_idx in particle_idxs {
                                if *p_idx > uidx_0 {
                                    particle_idxs_set.push(*p_idx);
                                }
                            }
                        },
                        None => {}
                    }
                }
/* 
                let particle_idx_it = keys.iter()
                    .filter_map(|key| self.dynamic_spatial_hash.map.get(key))
                    .flatten();

                // Remove any particle indexes that are less then our index.
                // Trying to avoid checking collision twice, if we have 3 particles [a, b, c]
                // we will end up in here with [a => b] but also [b => a] in the case a and b are in the same cells
                // this also stops self collisions [a => a]
                //
                // 6.8ms! - TODO: I think this is slow due to the above iterator being used.
                // I could try iterating over the data manually
                //
                for p_idx in particle_idx_it {
                    if *p_idx > uidx_0 {
                        particle_idxs_set.push(*p_idx);
                    }
                }*/

                //println!("d: {}", particle_idxs_set.len());

                for particle_idxs in particle_idxs_set.chunks(2) {
                    match particle_idxs {
                        [uidx_1, uidx_2] => {
                            let idx_1 = *uidx_1 as isize;
                            let idx_2 = *uidx_2 as isize;

                            unsafe { 
                                let pos_1 = f32x4::from_array([(*pos_ptr.offset(idx_1))[0], (*pos_ptr.offset(idx_1))[1], (*pos_ptr.offset(idx_2))[0], (*pos_ptr.offset(idx_2))[1]]);

                                let collision_axis = pos_0 - pos_1;
                                let dist_squared = collision_axis.length_squared_2_into_4();

                                let min_dist = radius_0 + f32x4::from_array([(*radius_ptr.offset(idx_1))[0], (*radius_ptr.offset(idx_1))[0], (*radius_ptr.offset(idx_2))[0], (*radius_ptr.offset(idx_2))[0]]);
                                let min_dist_squared = min_dist * min_dist;

                                
                                if dist_squared < min_dist_squared {
                                    let dist = f32x4::sqrt(dist_squared);

                                    /*
                                    // particles are too close to each other.
                                    // just let them pass through each other
                                    if dist[0] <= f32::EPSILON {
                                        return;
                                    }*/

                                    // n = normalised vector between particles
                                    let n = collision_axis / dist;
                                    debug_assert!(!n[0].is_nan());
                                    debug_assert!(!n[1].is_nan());
                                    debug_assert!(!n[3].is_nan());
                                    debug_assert!(!n[4].is_nan());

                                    let delta = min_dist - dist;
                                    //let delta_f32x2 = f32x4::from_array([dist[0], dist[0], dist[1], dist[1]]); //f32x2::splat(delta[0]);
                                    let movement = delta * n;

                                    debug_assert!(!movement[0].is_nan());
                                    debug_assert!(!movement[1].is_nan());
                                    debug_assert!(!movement[2].is_nan());
                                    debug_assert!(!movement[3].is_nan());

                                    // todo: some sort of select here based on distance?

                                    let m_ptr: *const f32x2 = movement.as_array().as_ptr() as *const f32x2;
                                    let movement_2 = *m_ptr.offset(0) + *m_ptr.offset(1);
                                    *movement_ptr.offset(idx_0) += movement_2;
                                }
                            }
                        },
                        [uidx_1] => {
                            let idx_1 = *uidx_1 as isize;

                            unsafe { 
                                let collision_axis = *pos_ptr.offset(idx_0) - *pos_ptr.offset(idx_1);
                                let dist_squared = collision_axis.length_squared_2_into_2();

                                let min_dist = f32x2::splat((*radius_ptr.offset(idx_0))[0]) + f32x2::splat((*radius_ptr.offset(idx_1))[0]);
                                let min_dist_squared = min_dist * min_dist;

                                 
                                if dist_squared < min_dist_squared {
                                    let dist = f32x2::sqrt(dist_squared);

                                    // particles are too close to each other.
                                    // just let them pass through each other
                                    if dist[0] <= f32::EPSILON {
                                        return;
                                    }

                                    // n = normalised vector between particles
                                    let n = collision_axis / dist;
                                    debug_assert!(!n[0].is_nan());
                                    debug_assert!(!n[1].is_nan());

                                    let delta = min_dist - dist;
                                    //let delta_f32x2 = f32x4::from_array([dist[0], dist[0], dist[1], dist[1]]); //f32x2::splat(delta[0]);
                                    let movement = delta * n;

                                    debug_assert!(!movement[0].is_nan());
                                    debug_assert!(!movement[1].is_nan());

                                    *movement_ptr.offset(idx_0) += movement;
                                }
                            }
                        },
                        _ => {}
                    }
                }
            }

             
            // static particle collisions
            // 1.4ms
            {
                let mut particle_idxs_set = SortedSet::<usize>::with_capacity(20);
                for i in 0..keys.len() {
                    let entry = self.static_spatial_hash.map.get(&keys[i]);
                    match entry {
                        Some(particle_idxs) => {
                            for p_idx in particle_idxs {
                                particle_idxs_set.push(*p_idx);
                            }
                        },
                        None => {}
                    }
                }

                //println!("s: {}", particle_idxs_set.len());
                /*

                let particle_idx_it = keys.iter()
                    .filter_map(|key| self.static_spatial_hash.map.get(key))
                    .flatten();

                for p_idx in particle_idx_it {
                    particle_idxs_set.push(*p_idx);
                }*/

                for particle_idxs in particle_idxs_set.chunks(2) {
                    match particle_idxs {
                        [uidx_1, uidx_2] => {
                            let idx_1 = *uidx_1 as isize;
                            let idx_2 = *uidx_2 as isize;

                            unsafe {
                                let pos_1 = f32x4::from_array([(*static_pos_ptr.offset(idx_1))[0], (*static_pos_ptr.offset(idx_1))[1], (*static_pos_ptr.offset(idx_2))[0], (*static_pos_ptr.offset(idx_2))[1]]);

                                let collision_axis = pos_0 - pos_1;
                                let dist_squared = collision_axis.length_squared_2_into_4();

                                let min_dist = radius_0 + f32x4::from_array([(*static_radius_ptr.offset(idx_1))[0], (*static_radius_ptr.offset(idx_1))[0], (*static_radius_ptr.offset(idx_2))[0], (*static_radius_ptr.offset(idx_2))[0]]);
                                let min_dist_squared = min_dist * min_dist;

                                if dist_squared < min_dist_squared {
                                    let dist = f32x4::sqrt(dist_squared);

                                    /*
                                    // particles are too close to each other.
                                    // just let them pass through each other
                                    if dist[0] <= f32::EPSILON {
                                        return;
                                    }*/

                                    // n = normalised vector between particles
                                    let n = collision_axis / dist;
                                    debug_assert!(!n[0].is_nan());
                                    debug_assert!(!n[1].is_nan());
                                    debug_assert!(!n[3].is_nan());
                                    debug_assert!(!n[4].is_nan());

                                    let delta = min_dist - dist;
                                    //let delta_f32x2 = f32x4::from_array([dist[0], dist[0], dist[1], dist[1]]); //f32x2::splat(delta[0]);
                                    let movement = delta * n;

                                    debug_assert!(!movement[0].is_nan());
                                    debug_assert!(!movement[1].is_nan());
                                    debug_assert!(!movement[2].is_nan());
                                    debug_assert!(!movement[3].is_nan());

                                    // todo: some sort of select here based on distance?

                                    let m_ptr: *const f32x2 = movement.as_array().as_ptr() as *const f32x2;
                                    let movement_2 = *m_ptr.offset(0) + *m_ptr.offset(1);
                                    *movement_ptr.offset(idx_0) += movement_2;
                                }
                            }
                        },
                        [uidx_1] => {
                            let idx_1 = *uidx_1 as isize;

                            unsafe {
                                let collision_axis = *pos_ptr.offset(idx_0) - *static_pos_ptr.offset(idx_1);
                                let dist_squared = collision_axis.length_squared_2_into_2();

                                let min_dist = f32x2::splat((*radius_ptr.offset(idx_0))[0]) + f32x2::splat((*static_radius_ptr.offset(idx_1))[0]);
                                let min_dist_squared = min_dist * min_dist;

                                if dist_squared < min_dist_squared {
                                    let dist = f32x2::sqrt(dist_squared);

                                    // particles are too close to each other.
                                    // just let them pass through each other
                                    if dist[0] <= f32::EPSILON {
                                        return;
                                    }

                                    // n = normalised vector between particles
                                    let n = collision_axis / dist;
                                    debug_assert!(!n[0].is_nan());
                                    debug_assert!(!n[1].is_nan());

                                    let delta = min_dist - dist;
                                    //let delta_f32x2 = f32x4::from_array([dist[0], dist[0], dist[1], dist[1]]); //f32x2::splat(delta[0]);
                                    let movement = delta * n;

                                    debug_assert!(!movement[0].is_nan());
                                    debug_assert!(!movement[1].is_nan());

                                    *movement_ptr.offset(idx_0) += movement;
                                }
                            }
                        },
                        _ => {}
                    }
                }
            }
        });

        //println!("{}", count);
        // todo: go through each particle an apply movement to the particle
    }

    // Like #5, but it seems like all the sorting and filtering to try to use simd is wasting a lot of time
    // so I cut all the crap and simplify
    //
    // 6.5ms where solve_collisions was taking 13ms, and solve_collisions_5 was taking 12ms
    pub fn solve_collisions_6(&mut self, particle_data: &mut ParticleData) {
        let dynamic_particles = &mut particle_data.dynamic_particles;

        // setup the spatial hash
        // 3.7ms
        self.dynamic_spatial_hash.soft_clear();   
        spatial_hash_keys_for_particles(dynamic_particles, |key: i32x2, particle_idx: usize| {
            /*
            if particle_idx >= dynamic_particles.len() {
                println!("error adding particle to spatial hash. particle idx: {} (# particles {})", particle_idx, dynamic_particles.len());
            }*/
            debug_assert!(particle_idx < dynamic_particles.len());
            self.dynamic_spatial_hash.map.entry(key).or_default().push(particle_idx);
        });
        //

        let pos_ptr: *const f32x2 = dynamic_particles.pos.as_ptr() as *const f32x2;
        let radius_ptr: *const f32x1 = dynamic_particles.radius.as_ptr() as *const f32x1;
        let movement_ptr: *mut f32x2 = dynamic_particles.movement.as_mut_ptr() as *mut f32x2;

        let static_particles = &mut particle_data.static_particles; 
        let static_pos_ptr: *const f32x2 = static_particles.pos.as_ptr() as *const f32x2;
        let static_radius_ptr: *const f32x1 = static_particles.radius.as_ptr() as *const f32x1;

        //let mut col_count = 0; // this is just for debugging. it can go in future

        // todo: should these be small vecs?
        // consider that there might be duplicate checks as a particle can be in multiple cells
        //let mut dynamic_collision_check = vec![isize::MAX; dynamic_particles.len()];
        //let mut static_collision_check = vec![isize::MAX; static_particles.len()];

        // +1 as we need to use a 1 based index as 0 * X = 0, so 0 based index doesn't work
        let mut dynamic_dynamic_collision_matrix = vec![false; dynamic_particles.len() * dynamic_particles.len()];
        let mut dynamic_static_collision_matrix = vec![false; dynamic_particles.len() * static_particles.len()];

        //println!("------- start of frame: {} -------", self.frame);
        
        // iterate over each dynamic particle
        // 2.8ms! wow nice!
        spatial_hash_keys_for_particles_keys(dynamic_particles, |uidx_0: usize, keys: &SmallVec::<[i32x2; 100]>| {
            let idx_0 = uidx_0 as isize;

            /*
            // testing problem with dynamic cols
            {
                let ai = uidx_0;
                let a_aabb = AabbSimd::from_position_and_radius(dynamic_particles.pos[ai], dynamic_particles.radius[ai][0]);
            
                for bi in self.dynamic_spatial_hash.aabb_iter(&a_aabb) {
                    if (ai == bi) {
                        continue;
                    }

                    let mut a_pos = dynamic_particles.pos[ai]; //vec2(particle_vec.pos_x[ai], particle_vec.pos_y[ai]);
                    let b_pos = dynamic_particles.pos[bi]; //vec2(particle_vec.pos_x[bi], particle_vec.pos_y[bi]);
                    
                    // particle_a is dynamic while particle_b is static
                    let collision_axis = a_pos - b_pos;
                    let dist_squared = collision_axis.length_squared();
                    let min_dist = dynamic_particles.radius[ai][0] + dynamic_particles.radius[bi][0];
                    let min_dist_squared = min_dist * min_dist;
    
                    if dist_squared < min_dist_squared {
                        let dist = f32::sqrt(dist_squared);
    
                        println!("[test] dyn-dyn collision between: {} and {}", ai, bi);
                    }
                }
            }*/

            // dynamic particle collisions
            {
                
                for i in 0..keys.len() {
                    let entry = self.dynamic_spatial_hash.map.get(&keys[i]);
                    match entry {
                        Some(particle_idxs) => {
                            for p_idx in particle_idxs {
                                // avoid self collision
                                if uidx_0 == *p_idx {
                                    continue;
                                }

                                // stop checking the same particle-particle collision
                                let collision_matrix_idx = uidx_0 + (dynamic_particles.len() * (*p_idx)); //(uidx_0+1) * ((*p_idx)+1);

                                /*
                                if *p_idx >= dynamic_particles.len() {
                                    println!("error in particle idx: {} while checking: {} (# particles {})", p_idx, uidx_0, dynamic_particles.len());
                                }*/

                                //println!("dyn-dyn collision between: {} and {}", uidx_0, p_idx);

                                debug_assert!(uidx_0 < dynamic_particles.len());
                                debug_assert!(*p_idx < dynamic_particles.len());
                                if dynamic_dynamic_collision_matrix[collision_matrix_idx] {
                                    //println!("stopped double col check dyn-static {} {}", uidx_0, p_idx);
                                    continue;
                                }
                                dynamic_dynamic_collision_matrix[collision_matrix_idx] = true;

                                /*
                                if *p_idx <= uidx_0 {
                                    continue;
                                }

                                // avoid double checking against the same particle
                                // todo: double check this is exhaustive!
                                // i.e. if a collides with b. when b checks against a, is it correctly skipped?
                                if dynamic_collision_check[*p_idx] == idx_0 {
                                    //println!("static skipping collision check between {} and {}", bi, ai);
                                    continue;
                                }
                                dynamic_collision_check[*p_idx] = idx_0;
                                */

                                let idx_1 = *p_idx as isize;

                                unsafe { 
                                    let collision_axis = *pos_ptr.offset(idx_0) - *pos_ptr.offset(idx_1);
                                    let dist_squared = collision_axis.length_squared_2_into_2();

                                    let min_dist = f32x2::splat((*radius_ptr.offset(idx_0))[0]) + f32x2::splat((*radius_ptr.offset(idx_1))[0]);
                                    let min_dist_squared = min_dist * min_dist;

                                    
                                    if dist_squared < min_dist_squared {
                                        let dist = f32x2::sqrt(dist_squared);

                                        // particles are too close to each other.
                                        // just let them pass through each other
                                        if dist[0] <= f32::EPSILON {
                                            continue;
                                        }

                                        // n = normalised vector between particles
                                        let n = collision_axis / dist;
                                        debug_assert!(!n[0].is_nan());
                                        debug_assert!(!n[1].is_nan());

                                        //let n_length = f32::sqrt((n[0] * n[0]) + (n[1] * n[1]));
                                        //debug_assert!(n_length == 1.0);

                                        let delta = min_dist - dist;
                                        //let delta_f32x2 = f32x4::from_array([dist[0], dist[0], dist[1], dist[1]]); //f32x2::splat(delta[0]);
                                        let movement = (delta * n) * f32x2::splat(0.5);

                                        debug_assert!(!movement[0].is_nan());
                                        debug_assert!(!movement[1].is_nan());


                                        /* 
                                        println!("collision occured. idx_0: {:?}, idx_1: {:?}, pos_0: {:?}, pos_1: {:?}, radius_0: {:?}, radius_1: {:?}, dist: {:?}, min_dist_squared: {:?}, dist_squared: {:?}, delta(overlap): {:?}, movement: {:?}, n(dir): {:?}, n_length: {:?}", 
                                            idx_0, idx_1, *pos_ptr.offset(idx_0), *pos_ptr.offset(idx_1),
                                            *radius_ptr.offset(idx_0), *radius_ptr.offset(idx_1),
                                            dist[0], min_dist_squared, dist_squared,
                                            delta, movement, n, n_length);
                                        */


                                        /* 
                                        self.file.write_all(format!("{:?}; d-d; {:?}; {:?}; {:?}; {:?}; {:?}; {:?}; {:?}; {:?}; {:?}; {:?}\n", 
                                            self.frame, idx_0, idx_1, *pos_ptr.offset(idx_0), *pos_ptr.offset(idx_1),
                                            dist[0], min_dist_squared, dist_squared,
                                            delta, movement, n).as_bytes()).unwrap();
                                        self.file.flush().unwrap();


                                        if movement[1].abs() > 0.3 {
                                            println!("too much movement in y axis! for {}", uidx_0);
                                        }*/

                                        /* 
                                        println!("particle {}, collided with dyn: {}", idx_0, idx_1);
                                        println!("collision_axis: {}, {}", collision_axis[0], collision_axis[1]);
                                        println!("movement: {}, {}", movement[0], movement[1]);
                                        println!("dist: {}", dist[0]);
                                        */

                                        *movement_ptr.offset(idx_0) += movement;
                                        *movement_ptr.offset(idx_1) -= movement;
                                        //*pos_ptr.offset(idx_0) += movement;


                                        /*
                                        if (*movement_ptr.offset(idx_0))[1].abs() > 0.01 {
                                            println!("too much movement in y axis! for {}", uidx_0);
                                        }*/
                                    }
                                }
                            }
                        },
                        None => {}
                    }
                }
            }
             
            /*
            // testing problem with static cols
            {
                let ai = uidx_0;
                let a_aabb = AabbSimd::from_position_and_radius(dynamic_particles.pos[ai], dynamic_particles.radius[ai][0]);
            
                for bi in self.static_spatial_hash.aabb_iter(&a_aabb) {
                    let mut a_pos = dynamic_particles.pos[ai]; //vec2(particle_vec.pos_x[ai], particle_vec.pos_y[ai]);
                    let b_pos = static_particles.pos[bi]; //vec2(particle_vec.pos_x[bi], particle_vec.pos_y[bi]);
                    
                    // particle_a is dynamic while particle_b is static
                    let collision_axis = a_pos - b_pos;
                    let dist_squared = collision_axis.length_squared();
                    let min_dist = dynamic_particles.radius[ai][0] + static_particles.radius[bi][0];
                    let min_dist_squared = min_dist * min_dist;
    
                    if dist_squared < min_dist_squared {
                        let dist = f32::sqrt(dist_squared);
    
                        println!("dyn-stat collision between: {} and {}", ai, bi);
                    }
                }
            }*/

            // static particle collisions
            {
                for i in 0..keys.len() {
                    let entry = self.static_spatial_hash.map.get(&keys[i]);
                    match entry {
                        Some(particle_idxs) => {
                            for p_idx in particle_idxs {
                                //println!("dyn-static check for col between: {} and {}", uidx_0, p_idx);

                                // stop checking the same particle-particle collision
                                let collision_matrix_idx = uidx_0 + (dynamic_particles.len() * (*p_idx)); //(uidx_0+1) * ((*p_idx)+1);
                                debug_assert!(uidx_0 < dynamic_particles.len());
                                debug_assert!(*p_idx < static_particles.len());
                                if dynamic_static_collision_matrix[collision_matrix_idx] {
                                    //println!("stopped double col check dyn-static {} {}", uidx_0, p_idx);
                                    continue;
                                }
                                dynamic_static_collision_matrix[collision_matrix_idx] = true;

                                let idx_1 = *p_idx as isize;
/* 
                                // avoid double checking against the same particle
                                // todo: double check this is exhaustive!
                                if static_collision_check[idx_1 as usize] == idx_0 {
                                    //println!("static skipping collision check between {} and {}", bi, ai);
                                    continue;
                                }
                                static_collision_check[idx_1 as usize] = idx_0;
*/
                                unsafe {
                                    let collision_axis = *pos_ptr.offset(idx_0) - *static_pos_ptr.offset(idx_1);

                                    debug_assert!(!(*static_pos_ptr.offset(idx_1))[0].is_nan());
                                    debug_assert!(!(*static_pos_ptr.offset(idx_1))[1].is_nan());

                                    let dist_squared = collision_axis.length_squared_2_into_2();

                                    let min_dist = f32x2::splat((*radius_ptr.offset(idx_0))[0]) + f32x2::splat((*static_radius_ptr.offset(idx_1))[0]);
                                    let min_dist_squared = min_dist * min_dist;

                                    if dist_squared < min_dist_squared {
                                        let dist = f32x2::sqrt(dist_squared);

                                        // particles are too close to each other.
                                        // just let them pass through each other
                                        if dist[0] <= f32::EPSILON {
                                            continue;
                                        }

                                        // n = normalised vector between particles
                                        let n = collision_axis / dist;
                                        debug_assert!(!n[0].is_nan());
                                        debug_assert!(!n[1].is_nan());

                                        //let n_length = f32::sqrt((n[0] * n[0]) + (n[1] * n[1]));

                                        let delta = min_dist - dist;
                                        //let delta_f32x2 = f32x4::from_array([dist[0], dist[0], dist[1], dist[1]]); //f32x2::splat(delta[0]);
                                        let movement = delta * n;

                                        debug_assert!(!movement[0].is_nan());
                                        debug_assert!(!movement[1].is_nan());
 
                                        /* 
                                        println!("particle {}, collided with static: {}", idx_0, idx_1);
                                        println!("collision_axis: {}, {}", collision_axis[0], collision_axis[1]);
                                        println!("movement: {}, {}", movement[0], movement[1]);
                                        println!("dist: {}", dist[0]);
                                        
                                        col_count += 1;
                                        */

                                        /*
                                        self.file.write_all(format!("{:?}; d-s; {:?}; {:?}; {:?}; {:?}; {:?}; {:?}; {:?}; {:?}; {:?}; {:?}\n", 
                                            self.frame, idx_0, idx_1, *pos_ptr.offset(idx_0), *static_pos_ptr.offset(idx_1),
                                            dist[0], min_dist_squared, dist_squared,
                                            delta, movement, n).as_bytes()).unwrap();
                                        self.file.flush().unwrap();
                                        */

                                        *movement_ptr.offset(idx_0) += movement;

                                        //*pos_ptr.offset(idx_0) += movement;
                                    }
                                }
                            }
                        },
                        None => {}
                    }
                }
            }
        });

        
        // go through each particle an apply movement to the particle
        // todo: process multiple particles at once with simd!
        {
            // drain energy out of the system
            // todo: does this need to be multiplied by frame rate?
            let damping = f32x2::splat(0.5);

            let pos_ptr: *mut f32x2 = dynamic_particles.pos.as_mut_ptr() as *mut f32x2;
            for i in 0..dynamic_particles.len() {
                unsafe {
                    let movement = *movement_ptr.offset(i as isize) * damping;

                    /*
                    if col_count > 0 {
                        println!("particle {} collided with {} times", i, col_count);
                        println!("movement: {}, {}", movement[0], movement[1]);
                    }*/

                    *pos_ptr.offset(i as isize) += movement;
                    *movement_ptr.offset(i as isize) = f32x2::splat(0.0);
                }
            }

            /*
            // checking for buffer overrun errors!
            for i in 0..dynamic_particles.len() {
                dynamic_particles.pos[i] += dynamic_particles.movement[i];
                dynamic_particles.movement[i] = f32x2::splat(0.0);
            }*/
        }

        /*
        {
            self.perform_dynamic_to_static_collision_detection(particle_data);
        }*/


        self.frame += 1;
    }

}
