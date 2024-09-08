use super::{constraints::constraint::Constraint, particle_handle::ConstraintHandle};


pub struct ConstraintContainer {
    pub constraints: Vec<Box<dyn Constraint + Send + Sync>>,
}

impl ConstraintContainer {
    pub fn new() -> Self {
        Self {
            constraints: vec![],
        }
    }

    pub fn add(&mut self, constraint: Box<dyn Constraint + Send + Sync>) -> ConstraintHandle {
        let id = self.constraints.len();
        self.constraints.push(constraint);
        ConstraintHandle::new(id) 
    }

    /* 
    pub fn filter_constraints_by_type<T>(&self) -> Vec::<Box<T>> {
        let v = Vec::<Box<T>>::new();
        for constraint in self.constraints {
            //constraint.as_any().downcast_ref::<T>().unwrap()

            if let Some(downcast_constraint) = constraint.as_any().downcast_ref::<T>() {
                v.push(downcast_constraint);
            }
        }

        //self.constraints.into_iter().map(|constraint| constraint.as_any().downcast_ref::<T>().unwrap())
        v
    }*/
}