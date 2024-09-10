use std::any::Any;

use crate::v4::particle_handle::ParticleHandle;

use super::constraint::Constraint;


/// Constraint that ignores weight and forces and just moves particles to try to maintain a
/// given distance between them
#[derive(Debug, Copy, Clone)]
pub struct StickConstraint {
    pub particle_handles: [ParticleHandle; 2],
    pub length: f32,
    pub stiffness_factor: f32, // stiffness_factor. 1.0 = fully stiff, 0.9 = 90% per second
    pub is_enabled: bool
}

impl StickConstraint {
    pub fn set_stiffness_factor(&mut self, stiffness_factor: f32) -> &mut Self {
        self.stiffness_factor = stiffness_factor;
        self
    }

    pub fn set_particle_handles(&mut self, particle_handles: [ParticleHandle; 2]) -> &mut Self {
        self.particle_handles = particle_handles;
        self
    }

    pub fn set_length(&mut self, length: f32) -> &mut Self {
        self.length = length;
        self
    }
}

impl Constraint for StickConstraint {
    fn box_clone(&self) -> Box<dyn Constraint + Send + Sync> {
        Box::new((*self).clone())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn offset_particle_handles(&mut self, offset: u64) {
        for particle_handle in self.particle_handles.iter_mut() {
            particle_handle.offset(offset);
        }
    }
}

impl Default for StickConstraint {
    fn default() -> Self {
        Self {
            particle_handles: [ParticleHandle::default(); 2],
            length: 0.0,
            stiffness_factor: 1.0,
            is_enabled: true,
        }
    }
}