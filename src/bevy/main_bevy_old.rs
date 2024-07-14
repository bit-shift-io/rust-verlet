use bevy::{
    prelude::*, render::camera::ScalingMode, sprite::MaterialMesh2dBundle
};

pub fn main_bevy() -> Result<(), String> {
    App::new()
        .insert_resource(ClearColor(
            Color::hex("010d13").unwrap(),
        ))

        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: format!(
                    "{} {}",
                    env!("CARGO_PKG_NAME"),
                    env!("CARGO_PKG_VERSION")
                ),
                resolution: (1280.0, 720.0).into(),
                ..Default::default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup_graphics)
        .add_systems(Startup, setup_physics)
        .add_systems(PostStartup, setup_text)
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

    let text = [
        TextSection::new("Primitive: ", style.clone()),
        TextSection::new(text, style.clone()),
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

fn setup_graphics(mut commands: Commands) {
    // https://bevy-cheatbook.github.io/2d/camera.html

    let mut my_2d_camera_bundle = Camera2dBundle::default();
    // For this example, let's make the screen/window height correspond to
    // 1600.0 world units. The width will depend on the aspect ratio.
    my_2d_camera_bundle.projection.scaling_mode = ScalingMode::FixedVertical(1600.0);
    my_2d_camera_bundle.transform = Transform::from_xyz(100.0, 200.0, 0.0);

    commands.spawn(my_2d_camera_bundle);
}

pub fn setup_physics(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // https://bevyengine.org/examples/math/render-primitives/
    let circle = Circle { radius: 20.0 };
    let material: Handle<ColorMaterial> = materials.add(Color::WHITE);

    //const LEFT_RIGHT_OFFSET_2D: f32 = 200.0;
    const POSITION: Vec3 = Vec3::new(0.0, 0.0, 0.0);

    let line_width = 1.0;
    let axis_length = 100.0;

    let rec = Rectangle {
        half_size: Vec2::new(line_width, axis_length),
    };

    // y-axis line
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(rec).into(),
            material: materials.add(Color::RED),
            transform: Transform::from_translation(Vec3::new(0.0, axis_length, 0.0)),
            ..Default::default()
        },
    ));

    // x-axis line
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(Rectangle {
                half_size: Vec2::new(axis_length, line_width),
            }).into(),
            material: materials.add(Color::GREEN),
            transform: Transform::from_translation(Vec3::new(axis_length, 0.0, 0.0)),
            ..Default::default()
        },
    ));

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(circle/* .mesh().build()*/).into(),
            material: material.clone(),
            transform: Transform::from_translation(POSITION),
            ..Default::default()
        },
    ));

    // fmnote:
    // now how do I effeciently draw a huge amount of circles:
    //      https://github.com/bevyengine/bevy/discussions/7366
    //
    // which points to this:
    //      https://github.com/bevyengine/bevy/blob/main/examples/shader/shader_instancing.rs
    //
    //  this might be better, I think it uses pointlist rendering:
    //      https://www.youtube.com/watch?v=MWIO-jP6pVo
    //      https://github.com/rust-adventure/bevy-examples/tree/829e01bf9eee5fb9af9780d759dadf4ea76e12ff/examples/pointcloud
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