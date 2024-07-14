
use crate::application::Application;
use crate::sdl_system::SdlSystem;
use crate::scenes::random_bodies::random_bodies_scene::RandomBodiesScene;
use crate::scenes::car::car_scene::CarScene;

pub fn main_sdl() -> Result<(), String> {
    let mut sdl = SdlSystem::new("Rust Verlet", 1200, 800);
    let mut application = Application::new(&mut sdl);

    application.register_scene(Box::new(CarScene::new()));
    application.register_scene(Box::new(RandomBodiesScene::new()));

    return application.run();
}