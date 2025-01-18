#![feature(test)]
#![feature(extract_if)]
#![feature(portable_simd)]
#![feature(iter_array_chunks)]

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

mod random;
mod bevy;
mod v4;
mod v5;

mod level {
    pub mod level;
    pub mod level_builder;
    pub mod level_builder_operation;
    pub mod level_builder_operation_registry;
    pub mod level_blocks {
        pub mod level_block;
        pub mod straight_level_block;
        pub mod spawn_operation;
        pub mod saggy_bridge_operation;
        pub mod finish_operation;
        pub mod cliff_operation;
        pub mod fluid_funnel;
        pub mod jelly_cube;
        pub mod drop_direction_reverse;
    }
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

