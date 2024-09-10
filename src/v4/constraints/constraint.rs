use std::any::Any;

use crate::v4::particle_handle::ParticleHandle;




pub trait Constraint {
    /// https://users.rust-lang.org/t/solved-is-it-possible-to-clone-a-boxed-trait-object/1714/7
    /// todo: look at the iproved version
    fn box_clone(&self) -> Box<dyn Constraint + Send + Sync>;

    fn as_any(&self) -> &dyn Any;
}