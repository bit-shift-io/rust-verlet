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
pub type SpringHandle = ParticleHandle;
pub type AttachmentConstraintHandle = ParticleHandle;

pub(crate) struct VerletPosition {
    pub(crate) pos: Vec2,
    pub(crate) pos_prev: Vec2,

    pub(crate) force: Vec2,
    pub(crate) mass: f32,
}

pub(crate) struct Layer {
    pub(crate) id: usize,
    pub(crate) particle_ids: Vec<usize>, // this lets us map back to the particle
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
/// Constraint that ignores weight and forces and just moves particles to try to maintain a
/// given distance between them
pub struct Stick {
    pub particle_indicies: [usize; 2], // rename to particle_ids ?
    pub length: f32,
    pub is_enabled: bool
}

/// Constraint that applies forces to particles in the fashion of a spring
/// higher spring_constant is a stuffer spring
/// elastic_limit serves as a a limit where the length can start to be modified
pub struct Spring {
    pub particle_indicies: [usize; 2], // rename to particle_ids ?
    pub length: f32,
    pub is_enabled: bool,

    pub elastic_limit: f32, // see 2. Plastic deformation here: https://www.khanacademy.org/science/physics/work-and-energy/hookes-law/a/what-is-hookes-law
        // -ve number = non elastic limit

    pub spring_constant: f32, // aka tightness: https://www.linkedin.com/pulse/springs-video-games-kieran-bradbury. Measured in N/m.
    pub damping: f32, // damping coefficient
}

#[derive(Clone)]
pub struct WeightedParticle {
    pub(crate) particle_id: usize,
    pub(crate) weight: f32,
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
pub(crate) struct AttachmentConstraint {
    pub(crate) incoming_weighted_particles: Vec<WeightedParticle>, // source particles
    pub(crate) outgoing_weighted_particles: Vec<WeightedParticle>, // target particles
    pub(crate) target_particle_id: usize, // target output particle
}


/// A container for particles and constraints
pub struct ParticleAccelerator {
    // here a particle is broken into two "channels", in order to perform SIMD operations on one part
    pub(crate) particles: Vec<Particle>,
    pub(crate) verlet_positions: Vec<VerletPosition>,

    pub(crate) sticks: Vec<Stick>,
    pub(crate) springs: Vec<Spring>,
    pub(crate) attachment_constraints: Vec<AttachmentConstraint>,

    pub(crate) layer_map: HashMap<usize, Layer>,
}

impl ParticleAccelerator {
    pub fn new() -> Self {
        Self {
            particles: vec![],
            verlet_positions: vec![],

            sticks: vec![],
            springs: vec![],
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
        let stick = Stick {
            particle_indicies: [particle_handles[0].id, particle_handles[1].id],
            length,
            is_enabled: true
        };
        self.sticks.push(stick);
        StickHandle::new(id)
    }

    pub fn create_spring(&mut self, particle_handles: [&ParticleHandle; 2], length: f32, spring_constant: f32, damping: f32, elastic_limit: f32) -> SpringHandle {
        let id = self.sticks.len();
        let spring = Spring {
            particle_indicies: [particle_handles[0].id, particle_handles[1].id],
            length,
            is_enabled: true,
            spring_constant,
            elastic_limit,
            damping
        };
        self.springs.push(spring);
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