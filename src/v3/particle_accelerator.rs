use std::collections::HashMap;

use sdl2::pixels::Color;

use crate::{point::vec2_to_point, sdl_system::SdlSystem};

use super::types::Vec2;

#[derive(Clone)]
pub struct ParticleHandle {
    id: usize,
}

impl ParticleHandle {
    pub fn new(id: usize) -> Self {
        Self { id }
    }

   pub fn id(&self) -> usize {
        self.id
    }
}

pub type StickHandle = ParticleHandle;
pub type AttachmentConstraintHandle = ParticleHandle;

pub(crate) struct VerletPosition {
    pub(crate) pos: Vec2,
    pub(crate) pos_prev: Vec2,

    pub(crate) force: Vec2,
    pub(crate) mass: f32,
}

struct Layer {
    id: usize,
    particle_ids: Vec<usize>, // this lets us map back to the particle
}

impl Layer {
    fn new(id: usize) -> Self {
        Self {
            id,
            particle_ids: vec![],
        }
    }
}

pub(crate) struct Particle {
    pub(crate) id: usize,
    pub(crate) radius: f32,
    pub(crate) mask: u32,
    pub(crate) is_static: bool,
    pub(crate) color: Color,
    pub(crate) is_enabled: bool,
}

// todo: rename to StickConstraint
pub struct Stick {
    pub particle_indicies: [usize; 2], // rename to particle_ids ?
    pub length: f32,
    pub is_enabled: bool
}

#[derive(Clone)]
pub struct WeightedParticle {
    particle_id: usize,
    weight: f32,
}

impl WeightedParticle {
    pub fn new(particle_handle: ParticleHandle, weight: f32) -> Self {
        Self {
            particle_id: particle_handle.id,
            weight
        }
    }
}


// An Attachment has a set of weighted particles
// such that when updated, computes a virtual position
// from which you can attach things too. This set is called incomming_particles
//
// An Attachment also has a set of weighted particles to 'push' for when this attachment itself is moved. This set is called outgoing_particles
//
// todo: consider if this is just a 'constraint', and can be just lumped in with sticks?
struct AttachmentConstraint {
    incoming_weighted_particles: Vec<WeightedParticle>, // source particles
    outgoing_weighted_particles: Vec<WeightedParticle>, // target particles
    target_particle_id: usize, // target output particle
}


pub struct ParticleAccelerator {
    // here a particle is broken into two "channels", in order to perform SIMD operations on one part
    pub(crate) particles: Vec<Particle>,
    pub(crate) verlet_positions: Vec<VerletPosition>,

    pub(crate) sticks: Vec<Stick>,
    pub(crate) attachment_constraints: Vec<AttachmentConstraint>,

    pub(crate) layer_map: HashMap<usize, Layer>,
}

impl ParticleAccelerator {
    pub fn new() -> Self {
        Self {
            particles: vec![],
            verlet_positions: vec![],

            sticks: vec![],
            attachment_constraints: vec![],
            
            layer_map: HashMap::new(),
        }
    }

    pub fn create_attachment_constraint(&mut self, incoming_weighted_particles: Vec<WeightedParticle>, outgoing_weighted_particles: Vec<WeightedParticle>, target_particle_id: ParticleHandle) -> AttachmentConstraintHandle {
        let id = self.attachment_constraints.len();
        let constraint = AttachmentConstraint {
            incoming_weighted_particles,
            outgoing_weighted_particles,
            target_particle_id: target_particle_id.id,
        };
        self.attachment_constraints.push(constraint);
        AttachmentConstraintHandle::new(id)
    }

    pub fn create_stick(&mut self, particle_handles: [&ParticleHandle; 2], length: f32) -> StickHandle {
        let id = self.sticks.len();
        let sitck = Stick {
            particle_indicies: [particle_handles[0].id, particle_handles[1].id],
            length,
            is_enabled: true
        };
        self.sticks.push(sitck);
        StickHandle::new(id)
    }

    fn mask_to_layer_indicies(mask: u32) -> Vec<usize> {
        let mut indicies = vec![];
        let mut tmp_mask = mask;
        let mut idx = 0;
        while (tmp_mask != 0) {
            if ((tmp_mask & 0x1) == 0x1) {
                indicies.push(idx);
            }
            tmp_mask >>= 1;
            idx += 1;
        }
        return indicies;
    }

    pub fn create_particle(&mut self, pos: Vec2, radius: f32, mass: f32, mask: u32, color: Color) -> ParticleHandle {
        let id = self.particles.len();

        // push this particle index into each collision layer it needs to go into
        let layer_indicies = ParticleAccelerator::mask_to_layer_indicies(mask);
        for layer_index in layer_indicies {
            let layer = self.layer_map.entry(layer_index).or_insert(Layer::new(layer_index));
            layer.particle_ids.push(id);
        }

        let particle = Particle {
            id,
            radius,
            mask,
            is_static: false,
            color,
            is_enabled: true,
        };
        self.particles.push(particle);

        let verlet_position = VerletPosition { pos, pos_prev: pos, force: Vec2::zeros(), mass };
        self.verlet_positions.push(verlet_position);

        ParticleHandle::new(id) 
    }

    pub fn set_particle_static(&mut self, particle_handle: &ParticleHandle, is_static: bool) {
        self.particles[particle_handle.id].is_static = is_static;
    }

    pub fn get_particle_position(&self, particle_handle: &ParticleHandle) -> Vec2 {
        self.verlet_positions[particle_handle.id].pos
    }

    pub fn add_particle_force(&mut self, particle_handle: &ParticleHandle, force: Vec2) {
        self.verlet_positions[particle_handle.id].force += force;
    }

    pub fn set_particle_position_previous(&mut self, particle_handle: &ParticleHandle, pos_prev: Vec2) {
        self.verlet_positions[particle_handle.id].pos_prev = pos_prev;
    }

    pub fn set_particle_color(&mut self, particle_handle: &ParticleHandle, color: Color) {
        self.particles[particle_handle.id].color = color;
    }

    pub fn get_stick(&self, stick_handle: &StickHandle) -> &Stick {
        &self.sticks[stick_handle.id]
    }

    pub fn set_stick_enabled(&mut self, stick_handle: &StickHandle, is_enabled: bool) {
        self.sticks[stick_handle.id].is_enabled = is_enabled
    }

    pub fn set_particle_enabled(&mut self, particle_handle: &StickHandle, is_enabled: bool) {
        self.particles[particle_handle.id].is_enabled = is_enabled
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

    pub fn acceleration_to_force(acc: Vec2, mass: f32) -> Vec2 {
        acc * mass
    }

    pub fn reset_forces(&mut self, particle_accelerator: &mut ParticleAccelerator, gravity: Vec2) {
        for verlet_position in particle_accelerator.verlet_positions.iter_mut() {
            let force = Self::acceleration_to_force(gravity, verlet_position.mass);
            verlet_position.force = force;
        }
    }

    pub fn solve_collisions(&mut self, particle_accelerator: &mut ParticleAccelerator) {
        for layer in particle_accelerator.layer_map.values() {
            // for each layer, we need to collide with each particle
            let particle_count: usize = layer.particle_ids.len();
            for ai in 0..particle_count {
                for bi in (&ai+1)..particle_count {
                    let particle_id_a = layer.particle_ids[ai];
                    let particle_id_b = layer.particle_ids[bi];
                   
                    let particle_a = &particle_accelerator.particles[particle_id_a];
                    let particle_b = &particle_accelerator.particles[particle_id_b];

                    // ignore static - static collisions
                    if particle_a.is_static && particle_b.is_static {
                        continue;
                    }

                    // ignore disabled particles
                    if !particle_a.is_enabled || !particle_b.is_enabled {
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
            if particle.is_static || !particle.is_enabled {
                continue
            }

            let velocity: Vec2 = verlet_position.pos - verlet_position.pos_prev;
            let acceleration: Vec2 = verlet_position.force / verlet_position.mass;
            verlet_position.pos_prev = verlet_position.pos;
            verlet_position.pos = verlet_position.pos + velocity + acceleration * dt * dt;
        }
    }

    pub fn update_constraints(&mut self, particle_accelerator: &mut ParticleAccelerator) {
        self.update_attachment_constraints(particle_accelerator);
        self.update_sticks(particle_accelerator);
    }

    pub fn update_attachment_constraints(&mut self, particle_accelerator: &mut ParticleAccelerator) {
        for attachment_constraint in particle_accelerator.attachment_constraints.iter_mut() {
            let mut pos = Vec2::new(0f32, 0f32);
            for weighted_particle in attachment_constraint.incoming_weighted_particles.iter() {
                let p = &particle_accelerator.verlet_positions[weighted_particle.particle_id];
                pos += p.pos * weighted_particle.weight;
            }
            pos /= attachment_constraint.incoming_weighted_particles.len() as f32;

            let delta_pos;
            {
                let target_particle = &mut particle_accelerator.verlet_positions[attachment_constraint.target_particle_id];
                target_particle.pos = pos;
                delta_pos = pos - target_particle.pos_prev;
            }
/* 
            // push any outgoing particles based on their weight
            for weighted_particle in attachment_constraint.outgoing_weighted_particles.iter() {
                let p = &mut particle_accelerator.verlet_positions[weighted_particle.particle_id];
                p.pos += delta_pos * weighted_particle.weight;
            }*/
        }
    }

    pub fn update_sticks(&mut self, particle_accelerator: &mut ParticleAccelerator) {
        for stick in particle_accelerator.sticks.iter_mut() {
            if !stick.is_enabled {
                continue;
            }

            let particle_a = &particle_accelerator.particles[stick.particle_indicies[0]];
            let particle_b = &particle_accelerator.particles[stick.particle_indicies[1]];

            let (a_movement_weight, b_movement_weight) = self.compute_movement_weight(particle_a.is_static, particle_b.is_static);
                    
            let p1 = &particle_accelerator.verlet_positions[stick.particle_indicies[0]];
            let p2 = &particle_accelerator.verlet_positions[stick.particle_indicies[1]];

            let difference = p1.pos - p2.pos;
            let diff_length = difference.magnitude();
            let diff_factor = (stick.length - diff_length) / diff_length * 0.5;
            let offset = difference * diff_factor;
    
            {
                let p1mut = &mut particle_accelerator.verlet_positions[stick.particle_indicies[0]];
                p1mut.pos += offset * a_movement_weight;
            }

            {
                let p2mut = &mut particle_accelerator.verlet_positions[stick.particle_indicies[1]];
                p2mut.pos -= offset * b_movement_weight;
            }
        }
    }
}

/**
 * Utility class to help with particle manipulation.
 */
pub struct ParticleManipulator {

}

impl ParticleManipulator {
    pub fn new() -> Self {
        Self{}
    }

    pub fn add_rotational_force_around_point(&self, particle_accelerator: &mut ParticleAccelerator, particle_handles: &Vec<ParticleHandle>, pos: Vec2, force_magnitude: f32) {
        for particle_handle in particle_handles.iter() {
            let particle_pos = particle_accelerator.get_particle_position(particle_handle);
            let delta = particle_pos - pos;
            let adjacent = Vec2::new(-delta[1], delta[0]); // compute a vector at 90 degress to delta
            particle_accelerator.add_particle_force(particle_handle, adjacent * force_magnitude);
        }
    }
}