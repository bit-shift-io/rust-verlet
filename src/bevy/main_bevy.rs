// Code initially from shader_instancing.rs bevy/examples

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

use super::car_scene::CarScenePlugin;
use super::instancing::CustomMaterialPlugin;


pub fn main_bevy() {
    App::new()
        .add_plugins((DefaultPlugins, CustomMaterialPlugin, CarScenePlugin))
        .add_systems(Startup, setup_camera)
        .run();
}



fn setup_camera(mut commands: Commands) {

    /* 
    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 150.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });*/

    // camera
    commands.spawn(Camera3dBundle {
        projection: OrthographicProjection {
            // n world units (metres) per window height.
            scaling_mode: ScalingMode::FixedVertical(20.0), //200.0), // was 10
            ..default()
        }
        .into(),
        transform: Transform::from_xyz(0.0, 0.0, 1000.0).looking_at(Vec3::ZERO, Vec3::Y).with_translation(Vec3::new(0.0, 0.0, 0.0)),
        // transform: Transform::from_xyz(0.0, 0.0, 150.0).looking_at(Vec3::ZERO, Vec3::Y).with_translation(Vec3::new(0.0, 2.0, 0.0)),
        ..default()
    });
    

     /* 
    // https://bevy-cheatbook.github.io/2d/camera.html

    let mut my_2d_camera_bundle = Camera2dBundle::default();
    // For this example, let's make the screen/window height correspond to
    // 1600.0 world units. The width will depend on the aspect ratio.
    my_2d_camera_bundle.projection.scaling_mode = ScalingMode::FixedVertical(1600.0);
    my_2d_camera_bundle.transform = Transform::from_xyz(100.0, 200.0, 0.0);

    commands.spawn(my_2d_camera_bundle);
    */
}

