
pub struct ConstraintContainer<T> {
    pub constraints: Vec<T>,
}

impl<T> ConstraintContainer<T> {
    pub fn new() -> Self {
        Self {
            constraints: vec![],
        }
    }

    pub fn add(&mut self, constraint: T) -> usize {
        let id = self.constraints.len();
        self.constraints.push(constraint);
        id
    }
}