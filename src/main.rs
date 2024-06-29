mod sdl_system;
mod keyboard;
mod mouse;
mod application;
mod point;

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


use crate::application::Application;
use crate::sdl_system::SdlSystem;
use crate::scenes::random_bodies::random_bodies_scene::RandomBodiesScene;
use scenes::car::car_scene::CarScene;

fn main() -> Result<(), String> {
    let mut sdl = SdlSystem::new("Rust Verlet", 1200, 800);
    let mut application = Application::new(&mut sdl);

    application.register_scene(Box::new(CarScene::new()));
    application.register_scene(Box::new(RandomBodiesScene::new()));

    return application.run();
}
