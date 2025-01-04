use bevy::math::{bounding::Aabb2d, vec2, Vec2};


pub trait Aabb2dExt {
    fn from_position_and_radius(pos: Vec2, radius: f32) -> Self;
}

impl Aabb2dExt for Aabb2d {

    fn from_position_and_radius(pos: Vec2, radius: f32) -> Self {
        debug_assert!(!pos.x.is_nan());
        debug_assert!(!pos.y.is_nan());
        debug_assert!(!radius.is_nan());
        debug_assert!(radius > 0.0);
    
        let aabb = Aabb2d {
            min: pos - vec2(radius, radius),
            max: pos + vec2(radius, radius),
        };
        debug_assert!(aabb.min.x <= aabb.max.x && aabb.min.y <= aabb.max.y);
        aabb
    }
}


#[cfg(test)]
mod tests {
    use bevy::math::{bounding::Aabb2d, vec2};

    use super::*;

    #[test]
    fn from_position_and_radius() {
        let aabb = Aabb2d::from_position_and_radius(vec2(0.0, 0.0), 1.0);
        assert_eq!(aabb.min, vec2(-1.0, -1.0));
        assert_eq!(aabb.max, vec2(1.0, 1.0));
    }


}
