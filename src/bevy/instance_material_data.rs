use bevy::{ecs::query::QueryItem, math::Vec3, prelude::{Component, Deref, DerefMut}, render::extract_component::ExtractComponent};
use bytemuck::{Pod, Zeroable};

#[derive(Component, Deref, DerefMut)]
pub struct InstanceMaterialData(pub Vec<InstanceData>);

impl ExtractComponent for InstanceMaterialData {
    type QueryData = &'static InstanceMaterialData;
    type QueryFilter = ();
    type Out = Self;

    fn extract_component(item: QueryItem<'_, Self::QueryData>) -> Option<Self> {
        Some(InstanceMaterialData(item.0.clone()))
    }
}


#[derive(Clone, Copy, Pod, Zeroable)]
#[repr(C)]
pub struct InstanceData {
    pub position: Vec3,
    pub scale: f32,
    pub color: [f32; 4],
}