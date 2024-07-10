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


use bevy::{
    core_pipeline::bloom::BloomSettings, prelude::*,
    sprite::MaterialMesh2dBundle,
};
//use bevy_rapier2d::geometry::CollidingEntities;
//use bevy_rapier2d::prelude::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(
            Color::hex("010d13").unwrap(),
        ))
        .add_plugins(DefaultPlugins)
        /* 
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: WindowDescriptor {
                title: "2d Bloom!".to_string(),
                ..default()
            },
            ..default()
        }))*/
        //.add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        // .add_plugin(RapierDebugRenderPlugin::default())

        .add_systems(Startup, setup_graphics)
        .add_systems(Startup, setup_physics)
            //.add_systems(Update, (update_people, greet_people).chain());
        //.add_startup_system(setup_graphics)
        //.add_startup_system(setup_physics)
        //.add_system(control_color)
        // .add_startup_system(setup_scene)
        // .add_system(update_bloom_settings)
        // .add_system(bounce_spheres)
        .run();
}

/* 
fn control_color(
    meshes: Query<(
        &CollidingEntities,
        &Handle<ColorMaterial>,
    )>,
    mut colors: ResMut<Assets<ColorMaterial>>,
) {
    for (entities, color_handle) in meshes.iter() {
        let color = colors.get_mut(color_handle).unwrap();
        let color_hsla = color.color.as_hsla();

        if let Color::Hsla {
            hue,
            saturation,
            lightness: _,
            alpha,
        } = color_hsla
        {
            color.color = Color::Hsla {
                hue,
                saturation,
                lightness: 0.3
                    + entities.len() as f32 / 5.0,
                alpha,
            };
        };
    }
}*/

fn setup_graphics(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                hdr: true,
                ..default()
            },
            transform: Transform::from_xyz(0.0, 20.0, 0.0),
            ..default()
        },/*
        BloomSettings {
            threshold: 0.5,
            ..default()
        },*/
    ));
}

pub fn setup_physics(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    /*
     * Ground
     */
    let ground_size = 500.0;
    let ground_height = 10.0;

    commands.spawn((
        //Collider::cuboid(ground_size, ground_height),
        MaterialMesh2dBundle {
            mesh: meshes
                .add(Mesh::from(shape::Quad::new(
                    Vec2::new(
                        2.0 * ground_size,
                        2.0 * ground_height,
                    ),
                )))
                .into(),
            material: materials.add(ColorMaterial::from(
                Color::Hsla {
                    hue: 100.0,
                    saturation: 0.7,
                    lightness: 0.4,
                    alpha: 1.0,
                },
            )),
            transform: Transform::from_xyz(
                0.0,
                0.0 * -ground_height,
                0.0,
            ),
            ..default()
        },
    ));

    /*
     * Create the cubes
     */
    let num = 8;
    let rad = 10.0;

    let shift = rad * 2.0 + rad;
    let centerx = shift * (num / 2) as f32;
    let centery = shift / 2.0;

    let mut offset =
        -(num as f32) * (rad * 2.0 + rad) * 0.5;

    for j in 0usize..20 {
        for i in 0..num {
            let x = i as f32 * shift - centerx + offset;
            let y = j as f32 * shift + centery + 30.0;

            commands.spawn((
                //CollidingEntities::default(),
                //ActiveEvents::COLLISION_EVENTS,
                //RigidBody::Dynamic,
                //Collider::cuboid(rad, rad),
                MaterialMesh2dBundle {
                    mesh: meshes
                        .add(Mesh::from(shape::Quad::new(
                            Vec2::new(2.0 * rad, 2.0 * rad),
                        )))
                        .into(),
                    material: materials.add(
                        ColorMaterial::from(Color::Hsla {
                            hue: 100.0,
                            saturation: 0.7,
                            lightness: 1.2,
                            alpha: 1.0,
                        }),
                    ),
                    transform: Transform::from_xyz(
                        x, y, 0.0,
                    ),
                    ..default()
                },
            ));
        }

        offset -= 0.05 * rad * (num as f32 - 1.0);
    }
}