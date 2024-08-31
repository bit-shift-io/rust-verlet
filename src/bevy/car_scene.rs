use std::{cell::RefCell, rc::Rc, sync::{Arc, RwLock}};

use bevy::{
    core_pipeline::core_3d::Transparent3d, ecs::{
        query::QueryItem,
        system::{lifetimeless::*, SystemParamItem},
    }, math::vec2, pbr::{
        MeshPipeline, MeshPipelineKey, RenderMeshInstances, SetMeshBindGroup, SetMeshViewBindGroup,
    }, prelude::*, render::{
        camera::ScalingMode, extract_component::{ExtractComponent, ExtractComponentPlugin}, mesh::{GpuBufferInfo, GpuMesh, MeshVertexBufferLayoutRef}, render_asset::RenderAssets, render_phase::{
            AddRenderCommand, DrawFunctions, PhaseItem, PhaseItemExtraIndex, RenderCommand,
            RenderCommandResult, SetItemPipeline, TrackedRenderPass, ViewSortedRenderPhases,
        }, render_resource::*, renderer::RenderDevice, view::{ExtractedView, NoFrustumCulling}, Render, RenderApp, RenderSet
    }
};
use bytemuck::{Pod, Zeroable};

use crate::v4::{constraints::stick_constraint::StickConstraint, particle::Particle, particle_container::ParticleContainer, particle_sim::ParticleSim, particle_solvers::{naive_particle_solver::NaiveParticleSolver, spatial_hash_particle_solver::SpatialHashParticleSolver}, shape_builder::{line_segment::LineSegment, rectangle::Rectangle, shape_builder::{radius_divisions_between_points, ShapeBuilder}}};

use super::{instance_material_data::{InstanceData, InstanceMaterialData}};

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
    pub particle_sim: &'a mut ParticleSim,
}

/* 
#[derive(Component)]
struct CarComponent {
    car: Car
}

impl CarComponent {
    pub fn new(particle_sim: &mut ParticleSim) -> Self {
        let car = Car::new(particle_sim, Vec2::new(0.0, 0.5));

        Self {
            car
        }
    }
}
    */

#[derive(Component)]
struct CarScene {
    particle_sim: ParticleSim,
    paused: bool,
}

impl CarScene {
    pub fn new() -> Self {
        let particle_container = Arc::new(RwLock::new(ParticleContainer::new()));
        let particle_solver = Box::new(NaiveParticleSolver::new()); // SpatialHashParticleSolver::new()); // NaiveParticleSolver::new()); 
        let mut particle_sim = ParticleSim::new(&particle_container, particle_solver);

        {
            let mut particle_container = particle_sim.particle_container.as_ref().write().unwrap();


            let particle_radius = cm_to_m(4.0);
            let particle_mass = 1.0; //g_to_kg(0.1);

            // line along the ground
            //let mask = 0x1;

            ShapeBuilder::new()
                .set_particle_template(Particle::default().set_static(true).set_radius(particle_radius).clone()) 
                .add_particles(&LineSegment::new(vec2(-5.0, 0.0), vec2(5.0, 0.0)))
                .add_particles(&LineSegment::new(vec2(5.0, 0.0), vec2(8.0, 0.5)))
                .create_in_particle_container(&mut particle_container);
        

            // add a jellow cube to the scene
            ShapeBuilder::new()
                .set_particle_template(Particle::default().set_mass(particle_mass).set_radius(particle_radius).clone())
                .set_constraint_template(StickConstraint::default().set_stiffness_factor(20.0).clone())// this ignores mass
                .add_particles(&Rectangle::from_center_size(vec2(-3.0, 1.0), vec2(1.0, 1.0)))//                 //.add_stick_grid(2, 5, particle_radius * 2.2, Vec2::new(-3.0, cm_to_m(50.0)))
                .create_in_particle_container(&mut particle_container);
 
/*          // single particle for easier testing
            ShapeBuilder::new()
                .set_particle_template(Particle::default().set_mass(particle_mass).set_radius(particle_radius).clone())
                .set_constraint_template(StickConstraint::default().set_stiffness_factor(20.0).clone())// this ignores mass
                .add_particles(&LineSegment::new(vec2(-particle_radius * 2.0, 1.0), vec2(0.0, 1.0)))//                 //.add_stick_grid(2, 5, particle_radius * 2.2, Vec2::new(-3.0, cm_to_m(50.0)))
                .create_in_particle_container(&mut particle_container);

            */

            /* 
            // suspension bridge on the ground
            {
                let width = radius_divisions_between_points(Vec2::new(-5.0, 0.0), Vec2::new(-8.0, 0.0), particle_radius);
                let height = radius_divisions_between_points(Vec2::new(-8.0, 0.0), Vec2::new(-8.0, particle_radius * 6.0), particle_radius);
                let mut suspension_bridge = ShapeBuilder::new();
                suspension_bridge.set_particle_template(Particle::default().set_radius(particle_radius).clone());
                suspension_bridge
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
                suspension_bridge.create_in_particle_container(&mut particle_container);
            }
*/
/* 
            // particle liquid + bucket
            {
                let liquid_particle_radius = particle_radius * 0.85;
                let liquid_particle_mass = g_to_kg(20.0);

                let funnel_height = 2.0;
                let funnel_particle_radius = liquid_particle_radius * 0.75;

                let bucket_height = particle_radius * 6.0;
                let bucket_width = 3.0;

                let origin = Vec2::new(8.0, 0.5);
                let width = 20;
                let height = 15;
                let mut liquid = ShapeBuilder::new();
                liquid
                    .set_particle_template(Particle::default().set_mass(liquid_particle_mass).set_radius(liquid_particle_radius).clone())
                    .add_grid((width - 1) as i32, (height - 1) as i32, liquid_particle_radius * 2.0, origin + Vec2::new(0.0 + liquid_particle_radius * 2.0, funnel_height + 1.0))
                    //.connect_with_stick_chain(2)
                    .create_in_particle_container(&mut particle_container);


                let mut funnel = ShapeBuilder::new();
                funnel
                    .set_particle_template(Particle::default().set_static(true).set_radius(funnel_particle_radius).clone())
                    .add_line(origin + Vec2::new(-3.0, funnel_height + 2.0), origin + Vec2::new(1.0, funnel_height), funnel_particle_radius)
                    .add_line(origin + Vec2::new(5.0, funnel_height + 2.0), origin + Vec2::new(1.0 + liquid_particle_radius * 8.0, funnel_height), funnel_particle_radius)
                    .create_in_particle_container(&mut particle_container);

                let mut bucket = ShapeBuilder::new();
                bucket
                    .set_particle_template(Particle::default().set_static(true).set_radius(particle_radius).clone())
                    .add_line(origin, origin + Vec2::new(bucket_height, -bucket_height), particle_radius)
                    .add_line(origin + Vec2::new(bucket_height, -bucket_height), origin + Vec2::new(bucket_width - bucket_height, -bucket_height), particle_radius)
                    .add_line(origin + Vec2::new(bucket_width - bucket_height, -bucket_height), origin + Vec2::new(bucket_width, 0.0), particle_radius)
                    .create_in_particle_container(&mut particle_container);
            }
*/
            {
                // ground line to the righ of the bucket
                ShapeBuilder::new()
                    .set_particle_template(Particle::default().set_static(true).set_radius(particle_radius * 2.0).clone())
                    .add_particles(&LineSegment::new(vec2(11.0, 0.3), vec2(20.0, 1.0))) //.add_line(Vec2::new(11.0, 0.3), Vec2::new(20.0, 1.0), particle_radius * 2.0)
                    .create_in_particle_container(&mut particle_container);
            }
        }

        Self {
            particle_sim,
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
            .add_systems(Update, update_particle_instances)
            //.add_systems(Update, update_camera);
            ;
    }
}

pub fn setup_car_scene(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    let car_scene = CarScene::new();
/* 
    let car = CarComponent::new(&mut car_scene.particle_sim);
*/
    let instance_data = {
        let particle_container_ref = car_scene.particle_sim.particle_container.as_ref().read().unwrap();
        let particle_container = &*particle_container_ref;

        // create the required particles
        let instance_data = InstanceMaterialData(particle_container.particles.iter().map(|(particle)| InstanceData {
            position: Vec3::new(particle.pos.x, particle.pos.y, 0.0),
            scale: particle.radius,
            //color: LinearRgba::from(Color::hsla(x * 360., y, 0.5, 1.0)).to_f32_array(),
            color: particle.color.to_linear().to_f32_array(), //LinearRgba::from(Color::srgba_u8(particle.color.r, particle.color.g, particle.color.b, particle.color.a)).to_f32_array(),
        })
        .collect());
        instance_data
    };

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
/* 
    // add car to bevy ecs
    commands.spawn((
        car,
    ));*/
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
            let particle = &car_scene.particle_sim.particle_container.as_ref().read().unwrap().particles[i];
            instance.position = Vec3::new(particle.pos.x, particle.pos.y, 0.0);
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
    //mut car_component_query: Query<(&mut CarComponent)>,
    mut instance_material_data_query: Query<(&mut InstanceMaterialData)>
) {
    let mut car_scene = query_car_scenes.single_mut();
    //let mut car_component = car_component_query.single_mut();
    let delta_seconds = time.delta_seconds();

    // handle pause - could go in a different update system
    if keys.just_pressed(KeyCode::KeyP) {
        car_scene.paused = !car_scene.paused;
    }

    if car_scene.paused {
        return;
    }

    car_scene.particle_sim.pre_update();

    car_scene.particle_sim.update(delta_seconds);
/* 
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
    */
}

/*
fn update_camera(
    time: Res<Time>, 
    mut commands: Commands,
    mut query_car_scenes: Query<(&mut CarScene)>,
    mut car_component_query: Query<(&mut CarComponent)>,
    mut camera_query: Query<(&mut Camera, &mut Transform)>,
) {
    /* 
    let mut car_scene = query_car_scenes.single_mut();
    let mut car_component = car_component_query.single_mut();
    let Ok((mut camera, mut camera_transform)) = camera_query.get_single_mut() else { return };

    let particle_accelerator = &mut car_scene.particle_accelerator;

    let camera_look_at_position = car_component.car.get_camera_look_at_position(particle_accelerator);

    camera_transform.translation = Vec3::new(camera_look_at_position.x, camera_look_at_position.y, 150.0);
    */
}

    */