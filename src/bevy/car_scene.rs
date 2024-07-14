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

use crate::v3::{particle_accelerator::{self, ParticleAccelerator}, shape_builder::ShapeBuilder, types::Vec2};

use super::instance_material_data::{InstanceData, InstanceMaterialData};




#[derive(Component)]
struct CarScene {
    particle_accelerator: ParticleAccelerator,
}

impl CarScene {
    pub fn new() -> Self {
        let mut particle_accelerator = ParticleAccelerator::new();

        let mask = 0x1;
        ShapeBuilder::new()
            .set_static(true)
            .add_line(Vec2::new(100.0f32, 800.0f32), Vec2::new(600.0f32, 800.0f32), 8.0f32)
            .add_line(Vec2::new(600.0f32, 800.0f32), Vec2::new(1000.0f32, 700.0f32), 8.0f32)
            .create_in_particle_accelerator(&mut particle_accelerator, mask);
        
        Self {
            particle_accelerator,
        }
    }
}


pub struct CarScenePlugin;

impl Plugin for CarScenePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup_car_scene)
            .add_systems(Update, update_car_scene);
    }
}

pub fn setup_car_scene(mut commands: Commands) {
    commands.spawn((
        CarScene::new(),
    ));
}

fn update_car_scene(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut query_car_scenes: Query<(&mut CarScene)>) {
    let car_scene = query_car_scenes.single_mut();
    

    // todo: create a particle from each particle in the particle_accelerator
    let circle = Circle { radius: 0.5 };
    
    commands.spawn((
        //meshes.add(Cuboid::new(0.5, 0.5, 0.5)),
        meshes.add(circle),
        SpatialBundle::INHERITED_IDENTITY,
        InstanceMaterialData(
            (1..=10)
                .flat_map(|x| (1..=10).map(move |y| (x as f32 / 10.0, y as f32 / 10.0)))
                .map(|(x, y)| InstanceData {
                    position: Vec3::new(x * 10.0 - 5.0, y * 10.0 - 5.0, 0.0),
                    scale: 1.0,
                    color: LinearRgba::from(Color::hsla(x * 360., y, 0.5, 1.0)).to_f32_array(),
                })
                .collect(),
        ),
        // NOTE: Frustum culling is done based on the Aabb of the Mesh and the GlobalTransform.
        // As the cube is at the origin, if its Aabb moves outside the view frustum, all the
        // instanced cubes will be culled.
        // The InstanceMaterialData contains the 'GlobalTransform' information for this custom
        // instancing, and that is not taken into account with the built-in frustum culling.
        // We must disable the built-in frustum culling by adding the `NoFrustumCulling` marker
        // component to avoid incorrect culling.
        NoFrustumCulling,
    ));
}
