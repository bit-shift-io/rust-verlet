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
use rand_pcg::Pcg64;

use crate::{level::level::{setup_level, update_level}, random::Random, v4::{particle_sim::ParticleSim, particle_solvers::spatial_hash_particle_solver::SpatialHashParticleSolver}, v5::{particle::Particle, particle_system::ParticleSystem, shape_builder::{circle, rectangle, shape_builder::ShapeBuilder}}};

use super::{car::{self, Car}, instance_material_data::{InstanceData, InstanceMaterialData}, performance_ui::performance_ui_build};

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
    //pub particle_sim: &'a mut ParticleSim,
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
pub struct CarScene {
    pub car: Option<Car>,

    pub particle_sim: ParticleSim, // todo: remove this
    
    pub particle_system: ParticleSystem,
    //pub rng: Pcg64,
    paused: bool,
}

impl CarScene {
    pub fn new() -> Self {
        let mut particle_system = ParticleSystem::default();

        // todo: remove this
        let particle_solver = Box::new(SpatialHashParticleSolver::new()); // SpatialHashParticleSolver::new()); // NaiveParticleSolver::new()); 
        //let particle_solver = Box::new(SpatialHashParticleSolver::new()); // SpatialHashParticleSolver::new()); // NaiveParticleSolver::new()); 
        let mut particle_sim = ParticleSim::new(particle_solver);

        {
            let particle_radius = cm_to_m(4.0);
            let particle_mass = 1.0; //g_to_kg(0.1);

            // line along the ground
            //let mask = 0x1;

            /* 
            ShapeBuilder::new()
                .set_particle_template(Particle::default().set_static(true).set_radius(particle_radius).clone()) 
                .apply_operation(LineSegment::new(vec2(-5.0, 0.0), vec2(5.0, 0.0)))
                .apply_operation(LineSegment::new(vec2(5.0, 0.0), vec2(8.0, 0.5)))
                .apply_operation(LineSegment::new(vec2(-5.0, 0.0), vec2(-8.0, 0.5)))
                .create_in_particle_sim(&mut particle_sim);
            */

            /* 
            // add a jellow cube to the scene
            ShapeBuilder::new()
                .set_particle_template(Particle::default().set_mass(particle_mass).set_radius(particle_radius).set_color(Color::from(LinearRgba::RED)).clone())
                //.set_constraint_template(StickConstraint::default().set_stiffness_factor(20.0).clone())// this ignores mass
                .apply_operation(RectangleStickGrid::from_rectangle(StickConstraint::default().set_stiffness_factor(20.0).clone(), 
                    Rectangle::from_center_size(vec2(3.0, 1.5), vec2(0.4, 0.8))))//                 //.add_stick_grid(2, 5, particle_radius * 2.2, Vec2::new(-3.0, cm_to_m(50.0)))
                .create_in_particle_sim(&mut particle_sim);
            */
 
 
          /* 
            ShapeBuilder::new()
                .set_particle_template(Particle::default().set_static(true).set_radius(particle_radius).clone()) 
                .apply_operation(LineSegment::new(vec2(-1.0, 0.0), vec2(1.0, 0.0)))
                .create_in_particle_sim(&mut particle_sim);

            // single particle for easier testing
            ShapeBuilder::new()
                .set_particle_template(Particle::default().set_mass(particle_mass).set_radius(particle_radius).clone())
                .set_constraint_template(StickConstraint::default().set_stiffness_factor(20.0).clone())// this ignores mass
                .apply_operation(LineSegment::new(vec2(0.0, 0.4 - particle_radius * 2.0), vec2(0.0, 0.4 + particle_radius * 2.0)))//                 //.add_stick_grid(2, 5, particle_radius * 2.2, Vec2::new(-3.0, cm_to_m(50.0)))
                .create_in_particle_sim(&mut particle_sim);
*/

             /* 
            // suspension bridge on the ground
            {
                let mut suspension_bridge = ShapeBuilder::new();
                suspension_bridge.set_particle_template(Particle::default().set_radius(particle_radius).clone());

                suspension_bridge.apply_operation(RectangleStickGrid::from_rectangle(StickConstraint::default().set_stiffness_factor(200.0).clone(), 
                    Rectangle::from_corners(vec2(-5.0, 0.0), vec2(-1.0, -particle_radius * 6.0))));
                
                // set left and right most particles and make them static
                // todo: make this an operation?
                let aabb = suspension_bridge.get_aabb();
                suspension_bridge.particles.iter_mut().for_each(|particle| {
                    if particle.pos.x == aabb.min.x {
                        particle.set_static(true);
                    }
                    if particle.pos.x == aabb.max.x {
                        particle.set_static(true);
                    }
                });

                suspension_bridge.create_in_particle_sim(&mut particle_sim);
            }*/

            // this is the bench
            {
                // the ideal is particle size around diamter 1, radius = 0.5, as the spatial has has a grid size of 1!
                let particle_radius = 0.5;

                // static
                let mut perimeter = ShapeBuilder::new();
                perimeter.set_particle_template(Particle::default().set_static(true).set_radius(particle_radius).clone())
                    .apply_operation(circle::Circle::new(vec2(0.0, 0.0), 3.0))
                    .create_in_particle_system(&mut particle_system);

                    /* 
                let mut perimeter2 = ShapeBuilder::new();
                perimeter2.set_particle_template(Particle::default().set_static(true).set_radius(particle_radius).clone())
                    .apply_operation(circle::Circle::new(vec2(0.0, 0.0), 10.0 + (particle_radius * 2.0)))
                    .create_in_particle_system(&mut particle_system);
                */
 
   
                // some dynamic particles on the inside
                /* 
                let mut liquid = ShapeBuilder::new();
                liquid
                    .set_particle_template(Particle::default().set_mass(20.0 * 0.001).set_radius(particle_radius).set_color(Color::from(LinearRgba::BLUE)).clone())
                    .apply_operation(rectangle::Rectangle::from_center_size(vec2(0.0, 0.0), vec2(120.0, 120.0)))
                    .create_in_particle_system(&mut particle_system);
*/

                // todo: some instability here, might need to increase radius by epsilon?
                let mut liquid3 = ShapeBuilder::new();
                liquid3
                    .set_particle_template(Particle::default().set_mass(20.0 * 0.01).set_radius(particle_radius).set_color(Color::from(LinearRgba::BLUE)).clone())
                    .apply_operation(rectangle::Rectangle::from_center_size(vec2(0.0, 0.0), vec2(2.0, 2.0)))
                    .create_in_particle_system(&mut particle_system);

/*
                // a single particle for testing
                let mut liquid2 = ShapeBuilder::new();
                liquid2
                    .set_particle_template(Particle::default().set_mass(20.0 * 0.001).set_radius(particle_radius).set_color(Color::from(LinearRgba::RED)).clone())
                    .add_particle_at_position(vec2(0.0, 0.0))
                    .add_particle_at_position(vec2(0.0, 3.0))
                    .create_in_particle_system(&mut particle_system);
                */
            }

            /* 
            // particle liquid + bucket
            {
                let liquid_particle_radius = particle_radius * 0.85;
                let liquid_particle_mass = g_to_kg(20.0);

                let funnel_height = 3.0;
                let funnel_width = 8.0;
                let funnel_particle_radius = liquid_particle_radius * 0.75;

                let bucket_height = particle_radius * 6.0;
                let bucket_width = 3.0;

                let origin = Vec2::new(0.0, 1.0);
                let liquid_width = liquid_particle_radius * 2.0 * 40.0;
                let liquid_height = liquid_particle_radius * 2.0 * 40.0;

                let mut liquid = ShapeBuilder::new();
                liquid
                    .set_particle_template(Particle::default().set_mass(liquid_particle_mass).set_radius(liquid_particle_radius).set_color(Color::from(LinearRgba::BLUE)).clone())
                    .apply_operation(Rectangle::from_center_size(origin + vec2(0.0, liquid_height * 0.5 + 1.0), vec2(liquid_width, liquid_height)))
                    .create_in_particle_sim(&mut particle_sim);
 
                let mut funnel = ShapeBuilder::new();
                funnel
                    .set_particle_template(Particle::default().set_static(true).set_radius(funnel_particle_radius).clone())
                    .apply_operation(LineSegment::new(origin + vec2(-funnel_width * 0.5, funnel_height), origin + vec2(- liquid_particle_radius * 4.0, 0.0))) //.add_line(origin + Vec2::new(-3.0, funnel_height + 2.0), origin + Vec2::new(1.0, funnel_height), funnel_particle_radius)
                    .apply_operation(LineSegment::new(origin + vec2(funnel_width * 0.5, funnel_height), origin + vec2(liquid_particle_radius * 4.0, 0.0))) //.add_line(origin + Vec2::new(5.0, funnel_height + 2.0), origin + Vec2::new(1.0 + liquid_particle_radius * 8.0, funnel_height), funnel_particle_radius)
                    .create_in_particle_sim(&mut particle_sim);
 
                / * 
                // bucket
                let mut bucket = ShapeBuilder::new();
                bucket
                    .set_particle_template(Particle::default().set_static(true).set_radius(particle_radius).clone())
                    .apply_operation(LineSegment::new(origin, origin + vec2(bucket_height, -bucket_height))) 
                    .apply_operation(LineSegment::new(origin + vec2(bucket_height, -bucket_height), origin + vec2(bucket_width - bucket_height, -bucket_height)))
                    .apply_operation(LineSegment::new(origin + vec2(bucket_width - bucket_height, -bucket_height), origin + vec2(bucket_width, 0.0)))
                    .create_in_particle_sim(&mut particle_sim);
                * /
            }*/

 
            /*
            {
                // ground line to the righ of the bucket
                ShapeBuilder::new()
                    .set_particle_template(Particle::default().set_static(true).set_radius(particle_radius * 2.0).clone())
                    .apply_operation(LineSegment::new(vec2(11.0, 0.3), vec2(20.0, 1.0))) //.add_line(Vec2::new(11.0, 0.3), Vec2::new(20.0, 1.0), particle_radius * 2.0)
                    .create_in_particle_sim(&mut particle_sim);
            }*/
        }

        // let particle system know all static particles have been built
        //particle_sim.notify_particle_container_changed();

        // SWITCH THE FOLLOWING TO LINES TO ENABLE THE CAR:
        //let car = Some(Car::new(&mut particle_sim, Vec2::new(0.0, 0.5)));
        let car = None;

        Self {
            car,
            particle_sim,
            particle_system,
            //rng: Random::seed_from_beginning_of_week(),
            paused: true,
        }
    }

    pub fn update(&mut self, time: Res<Time>, keys: Res<ButtonInput<KeyCode>>) {
        let delta_seconds = time.delta_seconds();

        // handle pause - could go in a different update system
        if keys.just_pressed(KeyCode::KeyP) {
            self.paused = !self.paused;
        }

        if self.paused {
            return;
        }

        self.particle_system.pre_update();
    
    /* 
        // do other physics here...
        // now update the car which will setup its forces on the particles
        match self.car {
            Some(ref mut car) => {
                car.update(&mut self.particle_system, keys);
            }
            None => (),
        };
*/

        self.particle_system.update(delta_seconds);
    }
    
}


pub struct CarScenePlugin;

impl Plugin for CarScenePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, (
                setup_origin_and_axis_indicators, 
                setup_car_scene, 
                
                // ENABLE THIS TO GET THE RANDOM LEVEL GENERATOR WORKING:
                //setup_level.after(setup_car_scene) // https://github.com/bevyengine/bevy/blob/main/examples/ecs/one_shot_systems.rs
            ))
            .add_systems(Update, update_car_scene)
            .add_systems(Update, update_level)
            .add_systems(Update, update_particle_instances)
            .add_systems(Update, update_camera)
            ;

        performance_ui_build(app);
    }
}

pub fn setup_origin_and_axis_indicators(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {

    // spawn a rect at the origin so we know where the origin is!
    let color = Color::from(LinearRgba::new(1.0, 0.0, 0.0, 1.0));

    // create a particle from each particle in the particle_accelerator
    let rectangle = bevy::prelude::Rectangle::new(1.0, 1.0); // Add random height to base height
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(rectangle),
            material: materials.add(color),
            transform: Transform::from_xyz(
                0.0,
                0.0,
                1.0,
            ),
            ..default()
        }
    ));
}

pub fn setup_car_scene(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    let car_scene = CarScene::new();

    //let car = CarComponent::new(&mut car_scene.particle_sim);

    /* todo: remove
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
    };*/

    // this is the v5 version. should we split it into static and dynamic parts? maybe not, I only update the dynamic particles
    let instance_data = {
        let particle_data = &car_scene.particle_system.particle_data;
        let dynamic_particles = &particle_data.dynamic_particles;
        let static_particles = &particle_data.static_particles;

        let mut instance_data: Vec<InstanceData> = vec![]; 
        
        for i in 0..dynamic_particles.len() {
            instance_data.push(InstanceData {
                position: Vec3::new(dynamic_particles.pos[i][0], dynamic_particles.pos[i][1], 0.0),
                scale: dynamic_particles.radius[i][0],
                color: dynamic_particles.color[i].to_linear().to_f32_array(),
            });
        }

        for i in 0..static_particles.len() {
            instance_data.push(InstanceData {
                position: Vec3::new(static_particles.pos[i][0], static_particles.pos[i][1], 0.0),
                scale: static_particles.radius[i][0],
                color: static_particles.color[i].to_linear().to_f32_array(),
            });
        }

        // create the required particles
        let instance_data = InstanceMaterialData(instance_data);
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
    ));
    */
}

fn update_particle_instances(
    time: Res<Time>, 
    mut commands: Commands, 
    mut query_car_scenes: Query<(&mut CarScene)>,
    mut instance_material_data_query: Query<(&mut InstanceMaterialData)>,
    mut meshes: ResMut<Assets<Mesh>>
) {
    // todo: can we somehow make this simd optimised?

    let car_scene = query_car_scenes.single_mut();

    let particle_data = &car_scene.particle_system.particle_data;
    let dynamic_particles = &particle_data.dynamic_particles;
    let static_particles = &particle_data.static_particles;

    let combined_length = dynamic_particles.len() + static_particles.len();

    let mut instance_data = instance_material_data_query.single_mut();

    if instance_data.len() != combined_length {
 
        instance_data.resize(combined_length, InstanceData {
            scale: 1.0,
            position: Vec3::default(),
            color: Color::WHITE.to_linear().to_f32_array(),
        });
        
        for i in 0..dynamic_particles.len() {
            instance_data[i] = InstanceData {
                position: Vec3::new(dynamic_particles.pos[i][0], dynamic_particles.pos[i][1], 0.0),
                scale: dynamic_particles.radius[i][0],
                color: dynamic_particles.color[i].to_linear().to_f32_array(),
            };
        }

        for i in 0..static_particles.len() {
            let idx = i + dynamic_particles.len();
            instance_data[idx] = InstanceData {
                position: Vec3::new(static_particles.pos[i][0], static_particles.pos[i][1], 0.0),
                scale: static_particles.radius[i][0],
                color: static_particles.color[i].to_linear().to_f32_array(),
            };
        }

    } else {
        // todo: simd optimise this. Vec3 is simd friendly.
        for i in 0..dynamic_particles.len() {
            instance_data[i].position = Vec3::new(dynamic_particles.pos[i][0], dynamic_particles.pos[i][1], 0.0);
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
    let mut car_scene_mut = query_car_scenes.single_mut();
    car_scene_mut.update(time, keys);
}

fn update_camera(
    time: Res<Time>, 
    mut commands: Commands,
    mut query_car_scenes: Query<(&mut CarScene)>,
    //mut car_component_query: Query<(&mut CarComponent)>,
    mut camera_query: Query<(&mut Camera, &mut Transform)>,
) {
    let car_scene = query_car_scenes.single_mut();
    if car_scene.car.is_none() {
        return;
    }

    //let Ok(car_component) = car_component_query.get_single() else { return };
    let Ok((mut _camera, mut camera_transform)) = camera_query.get_single_mut() else { return };
    let particle_sim = &car_scene.particle_sim;
    let camera_look_at_position = car_scene.car.as_ref().unwrap().get_camera_look_at_position(particle_sim); // car_component
    camera_transform.translation = Vec3::new(camera_look_at_position.x, camera_look_at_position.y, 250.0);
}
