use std::collections::HashMap;

use sdl2::pixels::Color;

use crate::{point::vec2_to_point, sdl_system::SdlSystem};

use super::types::Vec2;

#[derive(Clone)]
pub struct ParticleHandle {
    id: usize,
}

impl ParticleHandle {
    fn new(id: usize) -> Self {
        Self { id }
    }
}

pub type StickHandle = ParticleHandle;

struct VerletPosition {
    pos: Vec2,
    pos_prev: Vec2,

    force: Vec2,
    mass: f32,
}

struct Layer {
    mask: u32,

    particle_ids: Vec<usize>, // this lets us map back to the particle

    // todo: split into dynamic and static particle positions?
    //verlet_positions: Vec<VerletPosition>,
}

impl Layer {
    fn new(mask: u32) -> Self {
        Self {
            mask,
            particle_ids: vec![],
        }
    }
}

struct Particle {
    id: usize,
    radius: f32,
    mask: u32,
    is_static: bool,
    color: Color,
    //verlet_position_index: usize,
}

struct Stick {
    particle_indicies: [usize; 2],
    length: f32,
}

/* 
pub struct ParticleConstraintHandleVectors {
    particle_handles: Vec<ParticleHandle>,
    stick_handles: Vec<StickHandle>,
}*/

pub struct ParticleAccelerator {
    // here a particle is broken into two "channels", in order to perform SIMD operations on one part
    particles: Vec<Particle>,
    verlet_positions: Vec<VerletPosition>,

    sticks: Vec<Stick>,

    layer_map: HashMap<u32, Layer>,
}

impl ParticleAccelerator {
    pub fn new() -> Self {
        Self {
            particles: vec![],
            sticks: vec![],
            verlet_positions: vec![],
            layer_map: HashMap::new(),
        }
    }

    pub fn create_stick(&mut self, particle_handles: [&ParticleHandle; 2], length: f32) -> StickHandle {
        let id = self.sticks.len();
        let sitck = Stick {
            particle_indicies: [particle_handles[0].id, particle_handles[1].id],
            length
        };
        self.sticks.push(sitck);
        StickHandle::new(id)
    }

    pub fn create_particle(&mut self, pos: Vec2, radius: f32, mass: f32, mask: u32, color: Color) -> ParticleHandle {
        let id = self.particles.len();

        let layer = self.layer_map.entry(mask).or_insert(Layer::new(mask));
        layer.particle_ids.push(id);
        //let verlet_position_index = layer.verlet_positions.len();

        let particle = Particle {
            id,
            radius,
            mask,
            is_static: false,
            color
            //verlet_position_index
        };
        self.particles.push(particle);

        let verlet_position = VerletPosition { pos, pos_prev: pos, force: Vec2::zeros(), mass };
        self.verlet_positions.push(verlet_position);

        //layer.verlet_positions.push(VerletPosition { pos, pos_prev: pos });
        
        ParticleHandle::new(id) 
    }

    pub fn set_particle_static(&mut self, particle_handle: &ParticleHandle, is_static: bool) {
        self.particles[particle_handle.id].is_static = is_static;
    }
}

pub struct ParticleCollider {

}

impl ParticleCollider {
    pub fn new() -> Self {
        Self {}
    }

    fn compute_movement_weight(&self, a_is_static: bool, b_is_static: bool) -> (f32, f32) {
        // movement weight is used to stop static objects being moved
        let a_movement_weight = if a_is_static { 0.0f32 } else if b_is_static { 1.0f32 } else { 0.5f32 };
        let b_movement_weight = 1.0f32 - a_movement_weight;
        (a_movement_weight, b_movement_weight)
    }

    pub fn range_substeps(&self, dt: f32, substeps: usize) -> Vec<f32> {
        let sub_dt: f32 = dt / substeps as f32;
        vec![sub_dt; substeps]
    }


    /* 
    pub fn solve_particle_particle_collision(
        &self, 
        particle_accelerator: &mut ParticleAccelerator,
        particle_id_a: usize, 
        particle_id_b: usize,
        dt: f32
    ) {
        let particle_a = &particle_accelerator.particles[particle_id_a];
        let particle_b = &particle_accelerator.particles[particle_id_b];

        let (a_movement_weight, b_movement_weight) = self.compute_movement_weight(particle_a.is_static, particle_b.is_static);
        
        let collision_axis: Vec2;
        let dist: f32;
        let min_dist: f32;

        // in a code block so ap and bp borrows are released as we need to borrow mut later if
        // there is a collision
        {
            //let ap = a_particle.as_ref().borrow();
            //let bp = b_particle.as_ref().borrow();
            let verlet_position_a = &particle_accelerator.verlet_positions[particle_id_a];
            let verlet_position_b = &particle_accelerator.verlet_positions[particle_id_b];
        
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
            let verlet_position_a = &mut particle_accelerator.verlet_positions[particle_id_a];
            verlet_position_a.pos += a_movement_weight * delta * n;

            //let mut bp_mut = b_particle.as_ref().borrow_mut();
            let verlet_position_b = &mut particle_accelerator.verlet_positions[particle_id_b];
            verlet_position_b.pos -= b_movement_weight * delta * n;
        }
    }
*/

    pub fn acceleration_to_force(acc: Vec2, mass: f32) -> Vec2 {
        acc * mass
    }

    pub fn reset_forces(&mut self, particle_accelerator: &mut ParticleAccelerator) {
        let gravity = Vec2::new(0f32, 1000f32);
        for verlet_position in particle_accelerator.verlet_positions.iter_mut() {
            let force = Self::acceleration_to_force(gravity, verlet_position.mass);
            verlet_position.force = force;
        }
        /*
        for (particle, verlet_position) in particle_accelerator.particles.iter().zip(particle_accelerator.verlet_positions.iter_mut()) {
            verlet_position.force = Vec2::zeros();
        }*/
    }

    pub fn solve_collisions(&mut self, particle_accelerator: &mut ParticleAccelerator) {
        for layer in particle_accelerator.layer_map.values() {
            // for each layer, we need to collide with other layers that share a bit of the bitmask
            // for now, assume a layer is self contained (i.e. wont collide with another layer)

            // for each layer, we need to collide with each particle

            let particle_count: usize = layer.particle_ids.len();
            for ai in 0..particle_count {
                for bi in (&ai+1)..particle_count {
                    let particle_id_a = layer.particle_ids[ai];
                    let particle_id_b = layer.particle_ids[bi];
                    //self.solve_particle_particle_collision(&mut *particle_accelerator, particle_id_a, particle_id_b, dt);

                    let particle_a = &particle_accelerator.particles[particle_id_a];
                    let particle_b = &particle_accelerator.particles[particle_id_b];

                    // ignore static - static collisions
                    if particle_a.is_static && particle_b.is_static {
                        continue;
                    }

                    let (a_movement_weight, b_movement_weight) = self.compute_movement_weight(particle_a.is_static, particle_b.is_static);
                    
                    let collision_axis: Vec2;
                    let dist: f32;
                    let min_dist: f32;

                    // in a code block so ap and bp borrows are released as we need to borrow mut later if
                    // there is a collision
                    {
                        //let ap = a_particle.as_ref().borrow();
                        //let bp = b_particle.as_ref().borrow();
                        let verlet_position_a = &particle_accelerator.verlet_positions[particle_id_a];
                        let verlet_position_b = &particle_accelerator.verlet_positions[particle_id_b];
                    
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
                        let verlet_position_a = &mut particle_accelerator.verlet_positions[particle_id_a];
                        verlet_position_a.pos += a_movement_weight * delta * n;

                        //let mut bp_mut = b_particle.as_ref().borrow_mut();
                        let verlet_position_b = &mut particle_accelerator.verlet_positions[particle_id_b];
                        verlet_position_b.pos -= b_movement_weight * delta * n;
                    }
                }
            }
        }
    }

    pub fn update_positions(&mut self, particle_accelerator: &mut ParticleAccelerator, dt: f32) {
        for (particle, verlet_position) in particle_accelerator.particles.iter().zip(particle_accelerator.verlet_positions.iter_mut()) {
            if particle.is_static {
                continue
            }

            let velocity: Vec2 = verlet_position.pos - verlet_position.pos_prev;
            let acceleration: Vec2 = verlet_position.force / verlet_position.mass;
            verlet_position.pos_prev = verlet_position.pos;
            verlet_position.pos = verlet_position.pos + velocity + acceleration * dt * dt;
        }
    }

    pub fn update_sticks(&mut self, particle_accelerator: &mut ParticleAccelerator) {
        for stick in particle_accelerator.sticks.iter_mut() {

            let p1 = &particle_accelerator.verlet_positions[stick.particle_indicies[0]];
            let p2 = &particle_accelerator.verlet_positions[stick.particle_indicies[1]];

            let difference = p1.pos - p2.pos;
            let diff_length = difference.magnitude();
            let diff_factor = (stick.length - diff_length) / diff_length * 0.5;
            let offset = difference * diff_factor;
    
            {
                let p1mut = &mut particle_accelerator.verlet_positions[stick.particle_indicies[0]];
                p1mut.pos += offset;
            }

            {
                let p2mut = &mut particle_accelerator.verlet_positions[stick.particle_indicies[1]];
                p2mut.pos -= offset;
            }
        }
    }
}

pub struct ParticleRenderer {

}

impl ParticleRenderer {
    pub fn new() -> Self {
        Self {}
    }

    pub fn draw(&self, sdl: &mut SdlSystem, particle_accelerator: &ParticleAccelerator) {
        // draw particles
        for (particle, verlet_position) in particle_accelerator.particles.iter().zip(particle_accelerator.verlet_positions.iter()) {
            sdl.draw_filled_circle(vec2_to_point(verlet_position.pos), particle.radius as i32, particle.color);
            /* 
            let layer_option = particle_accelerator.layer_map.get(&particle.mask);
            layer_option.as_ref().map(|layer| {
                let verlet_position = &layer.verlet_positions[particle.verlet_position_index];
                
            });*/
        }

        // draw sticks
        let col = Color::RGB(0, 128, 0);
        for stick in particle_accelerator.sticks.iter() {
            let p1pos = particle_accelerator.verlet_positions[stick.particle_indicies[0]].pos;
            let p2pos = particle_accelerator.verlet_positions[stick.particle_indicies[1]].pos;
            sdl.draw_line(vec2_to_point(p1pos), vec2_to_point(p2pos), col);
        }
    }
}