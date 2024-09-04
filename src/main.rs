#![feature(test)]

/*
mod sdl_system;
mod keyboard;
mod mouse;
mod application;
mod point;

mod sdl {
    pub mod main_sdl;
}
*/

mod bevy {
    pub mod instance_material_data;
    pub mod main_bevy;
    pub mod car_scene;
    pub mod performance_ui;
    //pub mod car;
}

/*
mod scenes {
    pub mod car {
        pub mod car_scene;
        pub mod car;
        pub mod cloth;
    }
    
    pub mod random_bodies {
        pub mod random_bodies_scene;
    }
}

mod v3 {
    pub mod types;
    pub mod particle_accelerator;
    pub mod particle_renderer;
    pub mod particle_collider;
    pub mod shape_builder;
}
*/

mod v4 {
    pub mod particle;
    pub mod particle_handle;
    pub mod shape_builder {
        pub mod shape_builder;
        pub mod line_segment;
        pub mod rectangle;
        pub mod circle;
    }
    pub mod spatial_hash;
    pub mod particle_container;
    pub mod constraint_container;
    pub mod particle_solvers {
        pub mod particle_solver;
        pub mod naive_particle_solver;
        pub mod spatial_hash_particle_solver;
    }
    pub mod constraints {
        pub mod constraint;
        pub mod stick_constraint;
    }
    pub mod particle_sim;
}

//use crate::sdl::main_sdl::main_sdl;
//use crate::main_bevy::main_bevy;
use crate::bevy::main_bevy::main_bevy;

fn main() -> Result<(), String> {
    // this lets us switch between the old SDL renderer and the new Bevy renderer
    //return main_sdl();
    //return main_bevy();
    main_bevy();
    Ok({})
}

