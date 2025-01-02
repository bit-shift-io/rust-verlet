pub mod particle;
pub mod particle_handle;
pub mod shape_builder {
    pub mod shape_builder;
    pub mod line_segment;
    pub mod rectangle;
    pub mod circle;
    pub mod rectangle_stick_grid;
    pub mod adjacent_sticks;
}
pub mod spatial_hash;
pub mod particle_container;
pub mod constraint_container;
pub mod particle_solvers {
    pub mod particle_solver;
    pub mod naive_particle_solver;
    pub mod spatial_hash_particle_solver;
}
pub mod constraint_solvers {
    pub mod constraint_solver;
}
pub mod constraints {
    pub mod constraint;
    pub mod stick_constraint;
}
pub mod particle_sim;
pub mod particle_manipulator;