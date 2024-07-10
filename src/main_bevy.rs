
use bevy::{
    core_pipeline::bloom::BloomSettings, prelude::*, render::camera::ScalingMode, sprite::MaterialMesh2dBundle
};
//use bevy_rapier2d::geometry::CollidingEntities;
//use bevy_rapier2d::prelude::*;

pub fn main_bevy() -> Result<(), String> {
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
        .add_systems(PostStartup, setup_text)
            //.add_systems(Update, (update_people, greet_people).chain());
        //.add_startup_system(setup_graphics)
        //.add_startup_system(setup_physics)
        //.add_system(control_color)
        // .add_startup_system(setup_scene)
        // .add_system(update_bloom_settings)
        // .add_system(bounce_spheres)
        .run();

    Ok(())
}



fn setup_text(mut commands: Commands, cameras: Query<(Entity, &Camera)>) {
    let active_camera = cameras
        .iter()
        .find_map(|(entity, camera)| camera.is_active.then_some(entity))
        .expect("run condition ensures existence");
    let text = format!("{text}", text = "TEST");
    let style = TextStyle::default();
    let instructions = "Press 'C' to switch between 2D and 3D mode\n\
        Press 'Up' or 'Down' to switch to the next/previous primitive";
    let text = [
        TextSection::new("Primitive: ", style.clone()),
        TextSection::new(text, style.clone()),
        TextSection::new("\n\n", style.clone()),
        TextSection::new(instructions, style.clone()),
        TextSection::new("\n\n", style.clone()),
        TextSection::new(
            "(If nothing is displayed, there's no rendering support yet)",
            style.clone(),
        ),
    ];

    commands
        .spawn((
            //HeaderNode,
            NodeBundle {
                style: Style {
                    justify_self: JustifySelf::Center,
                    top: Val::Px(5.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            TargetCamera(active_camera),
        ))
        .with_children(|parent| {
            parent.spawn((
                //HeaderText,
                TextBundle::from_sections(text).with_text_justify(JustifyText::Center),
            ));
        });
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
    // https://bevy-cheatbook.github.io/2d/camera.html

    let mut my_2d_camera_bundle = Camera2dBundle::default();
    // For this example, let's make the screen/window height correspond to
    // 1600.0 world units. The width will depend on the aspect ratio.
    my_2d_camera_bundle.projection.scaling_mode = ScalingMode::FixedVertical(1600.0);
    my_2d_camera_bundle.transform = Transform::from_xyz(100.0, 200.0, 0.0);

    commands.spawn(my_2d_camera_bundle);
    /* 
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                //hdr: true,
                ..default()
            },
            transform: Transform::from_xyz(0.0, 20.0, 0.0),
            ..default()
        },/ *
        BloomSettings {
            threshold: 0.5,
            ..default()
        },
    ));*/
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

    // https://bevyengine.org/examples/math/render-primitives/
    let circle = Circle { radius: 20.0 };
    let material: Handle<ColorMaterial> = materials.add(Color::WHITE);

    const LEFT_RIGHT_OFFSET_2D: f32 = 200.0;
    const POSITION: Vec3 = Vec3::new(0.0, 0.0, 0.0);

    commands.spawn((
        /*
        MeshDim2,
        PrimitiveData {
            camera_mode,
            primitive_state: state,
        },*/
        MaterialMesh2dBundle {
            mesh: meshes.add(circle.mesh().build()).into(),
            material: material.clone(),
            transform: Transform::from_translation(POSITION),
            ..Default::default()
        },
    ));

    /*
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
    */

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