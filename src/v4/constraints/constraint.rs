


pub trait Constraint {
    fn box_clone(&self) -> Box<dyn Constraint>;
}