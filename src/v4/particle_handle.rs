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