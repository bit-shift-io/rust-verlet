
use crate::verlet_object::VerletObject;

pub struct StickConstraint {
    pub length: f32,
    pub p1: usize, // handle to a verlet object
    pub p2: usize // handle to a verlet object
}

impl StickConstraint {
    pub fn update(&mut self, dt: f32, p1: &mut VerletObject, p2: &mut VerletObject) {
        
    }
}