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