use std::usize;

#[derive(Debug, Clone, Copy)]
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

impl Default for ParticleHandle {
    fn default() -> Self {
        Self {
            id: usize::MAX
        }
    }
}

pub type ConstraintHandle = ParticleHandle;
/* 
pub type StickHandle = ParticleHandle;
pub type SpringHandle = ParticleHandle;
pub type AttachmentConstraintHandle = ParticleHandle;
*/