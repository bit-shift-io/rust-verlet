use crate::v4::{constraints::{constraint::Constraint, stick_constraint::StickConstraint}, particle_handle::ParticleHandle};

use super::{rectangle::Rectangle, shape_builder::{ShapeBuilder, ShapeBuilderOperation}};

/// Takes a Rectangle and created stick constraints in a grid layout between them
pub struct RectangleStickGrid {
    rectangle: Rectangle,
    constraint_template: StickConstraint
}

impl RectangleStickGrid {
    pub fn from_rectangle(constraint_template: StickConstraint, rectangle: Rectangle) -> Self {
        Self {
            constraint_template,
            rectangle
        }
    }
}

impl ShapeBuilderOperation for RectangleStickGrid {
    fn apply_to_shape_builder(&self, shape_builder: &mut ShapeBuilder) {
        let radius = shape_builder.particle_radius();
        self.rectangle.apply_to_shape_builder(shape_builder);

        let (x_divisions, y_divisions, _x_delta, _y_delta) = self.rectangle.get_divisions_and_deltas_for_radius(radius);

        for yi in 0..=y_divisions {
            for xi in 0..=x_divisions {
                let current_index = yi * x_divisions + xi;
                if xi != 0 {
                    let particle_handles = [
                        ParticleHandle::new(current_index - 1),
                        ParticleHandle::new(current_index)
                    ];
                    let constraint = self.constraint_template.clone().set_particle_handles(particle_handles).box_clone();
                    shape_builder.add_constraint(constraint);
                }

                if yi != 0 {
                    let up_point = xi + (yi - 1) * (x_divisions + 1);
                    let particle_handles = [
                        ParticleHandle::new(up_point),
                        ParticleHandle::new(current_index)
                    ];
                    let constraint = self.constraint_template.clone().set_particle_handles(particle_handles).box_clone();
                    shape_builder.add_constraint(constraint);
                }
            }
        }
    }
}