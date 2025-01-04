use bevy::math::{bounding::Aabb2d, vec2, Vec2};


pub trait Aabb2dExt {
    fn from_position_and_radius(pos: Vec2, radius: f32) -> Self;

    fn fabian_test(self: &Self) -> bool;
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

    fn fabian_test(self: &Self) -> bool {
        true
    }
}


pub fn aabb2d_from_position_and_radius(pos: Vec2, radius: f32) -> Aabb2d {
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