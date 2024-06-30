/*
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
*/

use bevy::prelude::*;

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Elaina Proctor".to_string())));
    commands.spawn((Person, Name("Renzo Hume".to_string())));
    commands.spawn((Person, Name("Zayna Nieves".to_string())));
}

fn greet_people(query: Query<&Name, With<Person>>) {
    for name in &query {
        println!("hello {}!", name.0);
    }
}

fn update_people(mut query: Query<&mut Name, With<Person>>) {
    for mut name in &mut query {
        if name.0 == "Elaina Proctor" {
            name.0 = "Elaina Hume".to_string();
            break; // We don’t need to change any other names
        }
    }
}


fn main() {
    App::new()
        .add_systems(Startup, add_people)
        .add_systems(Update, (hello_world_system, (update_people, greet_people).chain()))
        .run();
}

fn hello_world_system() {
    println!("hello world");
}