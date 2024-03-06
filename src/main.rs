mod sdl_system;
mod application;
mod solver;
mod particle;
mod stick;
mod cloth;
mod scenes {
    pub mod car_scene;
    pub mod cloth_scene;
    pub mod random_bodies_scene;
}

use crate::application::Application;
use crate::sdl_system::SdlSystem;
use crate::scenes::random_bodies_scene::RandomBodiesScene;
use scenes::car_scene::CarScene;
use scenes::cloth_scene::ClothScene;

fn main() -> Result<(), String> {
    let mut sdl = SdlSystem::new("Rust Verlet", 1200, 800);
    let mut application = Application::new(&mut sdl);
    application.register_scene(Box::new(CarScene::new()));
    application.register_scene(Box::new(RandomBodiesScene::new()));
    application.register_scene(Box::new(ClothScene::new()));
    return application.run();
}
