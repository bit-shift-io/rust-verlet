use std::any::Any;

pub trait Constraint {
    /// https://users.rust-lang.org/t/solved-is-it-possible-to-clone-a-boxed-trait-object/1714/7
    /// todo: look at the improved version
    fn box_clone(&self) -> Box<dyn Constraint + Send + Sync>;

    fn as_any(&self) -> &dyn Any;

    // ShapeBuilder keeps particle handles in its own local index/space.
    // then when we move these constraints into the particle sim, we need to adjust these offsets
    // so the handles continue to point to the correct particle which have new indices.
    // when this adjustment occurs the shape builder will call this function with an offset.
    // Constraints just need to forward this to their particle handles.
    fn offset_particle_handles(&mut self, offset: u64);
}