pub struct Stick {
    pub length: f32,
    pub p1: usize, // handle to a verlet object
    pub p2: usize // handle to a verlet object
}

impl Stick {
    pub fn new(length: f32, particle_handle_1: usize, particle_handle_2: usize) -> Self {
        Self { length, p1: particle_handle_1, p2: particle_handle_2 }
    }
}