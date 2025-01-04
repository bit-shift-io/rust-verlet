
#[cfg(test)]
mod tests {
    use bevy::math::Vec2;

    use crate::v5::{particle::Particle, shape_builder::{circle::Circle, line_segment::LineSegment, shape_builder::ShapeBuilder}};

    use super::*;

    #[test]
    fn set_particle_properties() {
        let mut b = ShapeBuilder::new();
        b.set_particle_template(Particle::default().set_radius(3.0).clone());
        assert_eq!(b.particle_template.radius, 3.0);
    }

    #[test]
    fn add_particle() {
        let mut b = ShapeBuilder::new();
        b.add_particle(Particle::default().set_position(Vec2::new(1.0, 1.0)).clone());
        assert_eq!(b.particles.len(), 1);
    }

    #[test]
    fn line() {
        let mut b = ShapeBuilder::new();
        b.apply_operation(LineSegment::new(Vec2::new(0.0, 0.0), Vec2::new(10.0, 0.0)));
        assert_eq!(b.particles.len(), 10);
    }

    #[test]
    fn create_in_particle_container() {
        let mut b = ShapeBuilder::new();
        b.set_particle_template(Particle::default().set_static(true).clone());
        b.apply_operation(Circle::new(Vec2::new(0.0, 0.0), 10.0));

        let mut pc = ParticleContainer::new();
        b.create_in_particle_container(&mut pc);

        assert_eq!(pc.particles.len(), b.particle_handles.len());
        assert_eq!(pc.particles.len(),  b.particles.len());
    }
}

