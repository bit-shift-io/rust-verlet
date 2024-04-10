mod sdl_system;
mod keyboard;
mod mouse;
mod application;

mod scenes {
    pub mod car {
        pub mod car_scene;
    }
    pub mod cloth {
        pub mod cloth_scene;
    }
    pub mod random_bodies {
        pub mod random_bodies_scene;
    }
}

mod v1 {
    pub mod cloth;
    pub mod particle;
    pub mod stick;
    pub mod solver;
}

mod v2 {
    pub mod body;
    pub mod body_shapes;
    pub mod particle;
    pub mod stick;
    pub mod solver;
}


use crate::application::Application;
use crate::sdl_system::SdlSystem;
use crate::scenes::random_bodies::random_bodies_scene::RandomBodiesScene;
use scenes::car::car_scene::CarScene;
use scenes::cloth::cloth_scene::ClothScene;

fn main() -> Result<(), String> {
    let mut sdl = SdlSystem::new("Rust Verlet", 1200, 800);
    let mut application = Application::new(&mut sdl);
    application.register_scene(Box::new(CarScene::new()));
    application.register_scene(Box::new(RandomBodiesScene::new()));
    application.register_scene(Box::new(ClothScene::new()));
    return application.run();
}
