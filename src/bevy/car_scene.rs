use bevy::{
    core_pipeline::core_3d::Transparent3d,
    ecs::{
        query::QueryItem,
        system::{lifetimeless::*, SystemParamItem},
    },
    pbr::{
        MeshPipeline, MeshPipelineKey, RenderMeshInstances, SetMeshBindGroup, SetMeshViewBindGroup,
    },
    prelude::*,
    render::{
        camera::ScalingMode, extract_component::{ExtractComponent, ExtractComponentPlugin}, mesh::{GpuBufferInfo, GpuMesh, MeshVertexBufferLayoutRef}, render_asset::RenderAssets, render_phase::{
            AddRenderCommand, DrawFunctions, PhaseItem, PhaseItemExtraIndex, RenderCommand,
            RenderCommandResult, SetItemPipeline, TrackedRenderPass, ViewSortedRenderPhases,
        }, render_resource::*, renderer::RenderDevice, view::{ExtractedView, NoFrustumCulling}, Render, RenderApp, RenderSet
    },
};
use bytemuck::{Pod, Zeroable};

use crate::v3::{particle_accelerator::{self, ParticleAccelerator}, particle_collider::ParticleCollider, shape_builder::{radius_divisions_between_points, ShapeBuilder}, types::Vec2};

use super::{car::Car, instance_material_data::{InstanceData, InstanceMaterialData}};

pub fn m_to_cm(m: f32) -> f32 {
    m * 100.0
}

pub fn cm_to_m(cm: f32) -> f32 {
    cm * 0.01
}

pub fn g_to_kg(g: f32) -> f32 {
    g * 0.001
}

pub struct CarSceneContext<'a> {
    //pub keyboard: &'a mut Keyboard,
    //pub mouse: &'a mut Mouse,
    pub keys: Res<'a, ButtonInput<KeyCode>>,
    pub particle_accelerator: &'a mut ParticleAccelerator,
}

#[derive(Component)]
struct CarComponent {
    car: Car
}

impl CarComponent {
    pub fn new(particle_accelerator: &mut ParticleAccelerator) -> Self {
        let car = Car::new(particle_accelerator, Vec2::new(0.0, 0.3));

        Self {
            car
        }
    }
}

#[derive(Component)]
struct CarScene {
    particle_accelerator: ParticleAccelerator,
    paused: bool,
}

impl CarScene {
    pub fn new() -> Self {
        let mut particle_accelerator = ParticleAccelerator::new();

        let particle_radius = cm_to_m(5.0);
        let particle_mass = 1.0; //g_to_kg(0.1);

        // line along the ground
        let mask = 0x1;

        ShapeBuilder::new()
            .set_static(true)
            .add_line(Vec2::new(-5.0, 0.0), Vec2::new(5.0, 0.0), particle_radius)
            .add_line(Vec2::new(5.0, 0.0), Vec2::new(8.0, 1.0), particle_radius)
            .create_in_particle_accelerator(&mut particle_accelerator, mask);
        
        // add a jellow cube to the scene
        ShapeBuilder::new()
            .set_stiffness_factor(20.0) // this ignores mass
            .set_mass(particle_mass)
            .set_radius(particle_radius)
            .add_stick_grid(2, 5, particle_radius * 2.2, Vec2::new(-3.0, cm_to_m(50.0)))
            .create_in_particle_accelerator(&mut particle_accelerator, mask);

        
        // suspension bridge on the ground
        {
            let width = radius_divisions_between_points(Vec2::new(-5.0, 0.0), Vec2::new(-8.0, 0.0), particle_radius);
            let height = radius_divisions_between_points(Vec2::new(-8.0, 0.0), Vec2::new(-8.0, particle_radius * 6.0), particle_radius);
            let mut suspension_bridge = ShapeBuilder::new();
            suspension_bridge
                .set_radius(particle_radius)
                //.set_static(true)
                //.add_parallelogram(Vec2::new(-5.0, 0.0), Vec2::new(-8.0, 0.0), Vec2::new(-8.0, particle_radius * 4.0))
                //.connect_with_cross_grid_of_sticks(height);
                .add_stick_grid((width - 1) as i32, (height - 1) as i32, particle_radius * 2.0, Vec2::new(-8.0, 0.0));
                //.remove_first_particle() // to avoid colliding with other statis particles
                //.connect_with_adjacent_sticks();
                
            // make the very left particles and very right particle static
            //let particle_count = suspension_bridge.particles.len();
            for y in 0..height {
                let first_particle_idx = y * width;
                let last_particle_idx = first_particle_idx + width - 1;

                suspension_bridge.particles[first_particle_idx].is_static = true;
                suspension_bridge.particles[last_particle_idx].is_static = true;
            }


    /* 
            let particle_count = suspension_bridge.particles.len();
            suspension_bridge.particles[0].is_static = true;
            suspension_bridge.particles[particle_count - 1].is_static = true;
    */
            suspension_bridge.create_in_particle_accelerator(&mut particle_accelerator, mask);
        }

        Self {
            particle_accelerator,
            paused: true,
        }
    }
}


pub struct CarScenePlugin;

impl Plugin for CarScenePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup_car_scene)
            .add_systems(Update, update_car_scene)
            .add_systems(Update, update_particle_instances);
    }
}

pub fn setup_car_scene(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    let mut car_scene = CarScene::new();

    let car = CarComponent::new(&mut car_scene.particle_accelerator);

    // create the required particles
    let instance_data = InstanceMaterialData(car_scene.particle_accelerator.particles.iter().zip(car_scene.particle_accelerator.verlet_positions.iter()).map(|(particle, verlet_position)| InstanceData {
        position: Vec3::new(verlet_position.pos.x, verlet_position.pos.y, 0.0),
        scale: particle.radius,
        //color: LinearRgba::from(Color::hsla(x * 360., y, 0.5, 1.0)).to_f32_array(),
        color: LinearRgba::from(Color::srgba_u8(particle.color.r, particle.color.g, particle.color.b, particle.color.a)).to_f32_array(),
    })
    .collect());

    // create a particle from each particle in the particle_accelerator
    let circle = Circle { radius: 1.0 };
    commands.spawn((
        //meshes.add(Cuboid::new(0.5, 0.5, 0.5)),
        meshes.add(circle),
        SpatialBundle::INHERITED_IDENTITY,
        instance_data,
        // NOTE: Frustum culling is done based on the Aabb of the Mesh and the GlobalTransform.
        // As the cube is at the origin, if its Aabb moves outside the view frustum, all the
        // instanced cubes will be culled.
        // The InstanceMaterialData contains the 'GlobalTransform' information for this custom
        // instancing, and that is not taken into account with the built-in frustum culling.
        // We must disable the built-in frustum culling by adding the `NoFrustumCulling` marker
        // component to avoid incorrect culling.
        NoFrustumCulling,
    ));

    // add car scene to bevy ecs
    commands.spawn((
        car_scene,
    ));

    // add car to bevy ecs
    commands.spawn((
        car,
    ));
}

fn update_particle_instances(
    time: Res<Time>, 
    mut query_car_scenes: Query<(&mut CarScene)>,
    mut instance_material_data_query: Query<(&mut InstanceMaterialData)>
) {
    // todo: will need to destroy the old InstanceMaterialData bundle and recreate if there is a different
    // number of particles (i.e. particles have spawned or been destroyed)

    let car_scene = query_car_scenes.single_mut();

    for mut instance_material_data in &mut instance_material_data_query {
        // https://www.reddit.com/r/bevy/comments/1e23o1z/animate_instance_data_in_update_loop/
        let mut i = 0;
        for instance in instance_material_data.iter_mut() {
            let verlet_position = &car_scene.particle_accelerator.verlet_positions[i];
            instance.position = Vec3::new(verlet_position.pos.x, verlet_position.pos.y, 0.0);
            //instance.scale += (time.elapsed_seconds()).sin() * 0.01;
            i += 1;
        } 
    }
}

fn update_car_scene(
    time: Res<Time>, 
    keys: Res<ButtonInput<KeyCode>>,
    mut commands: Commands, 
    mut meshes: ResMut<Assets<Mesh>>, 
    mut query_car_scenes: Query<(&mut CarScene)>,
    mut car_component_query: Query<(&mut CarComponent)>,
    mut instance_material_data_query: Query<(&mut InstanceMaterialData)>
) {
    let mut car_scene = query_car_scenes.single_mut();
    let mut car_component = car_component_query.single_mut();
    let delta_seconds = time.delta_seconds();

    // handle pause - could go in a different update system
    if keys.just_pressed(KeyCode::KeyP) {
        car_scene.paused = !car_scene.paused;
    }

    if car_scene.paused {
        return;
    }

    // reset forces to just the gravity value
    // 9.8 = units are in metres per second
    let gravity = Vec2::new(0.0, -9.8); // * 5.0); //9.8);
    let mut collider = ParticleCollider::new();
    collider.reset_forces(&mut car_scene.particle_accelerator, gravity);


    // do other physics here...
    // now update the car which will setup its forces on the particles
    let particle_accelerator = &mut car_scene.particle_accelerator;
    car_component.car.update(particle_accelerator, keys);


    // finally, solve everything for this frame
    let desired_hertz = 240.0; // times per second
    for sub_dt in collider.range_substeps_2(delta_seconds, desired_hertz).iter() {
        collider.solve_collisions(&mut car_scene.particle_accelerator);
        collider.update_constraints(&mut car_scene.particle_accelerator, *sub_dt);
        collider.update_positions(&mut car_scene.particle_accelerator, *sub_dt);
        collider.post_update_constraints(&mut car_scene.particle_accelerator, *sub_dt);
    }
}
