
mod sdl_system;
mod keyboard;
mod mouse;
mod application;
mod point;

mod sdl {
    pub mod main_sdl;
}

mod bevy {
    pub mod shader_instancing;
    //mod main_bevy;
}

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

use crate::sdl::main_sdl::main_sdl;
//use crate::main_bevy::main_bevy;
use crate::bevy::shader_instancing::b_main;

fn main() -> Result<(), String> {
    // this lets us switch between the old SDL renderer and the new Bevy renderer
    //return main_sdl();
    //return main_bevy();
    b_main();
    Ok({})
}

