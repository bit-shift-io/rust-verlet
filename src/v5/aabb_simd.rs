use std::simd::f32x4;
use std::simd::f32x2;

pub struct AabbSimd { 
    pub data: f32x4
}

impl AabbSimd {

    pub fn from_position_and_radius(pos: f32x2, radius: f32) -> Self {
        debug_assert!(!pos[0].is_nan());
        debug_assert!(!pos[1].is_nan());
        debug_assert!(!radius.is_nan());
        debug_assert!(radius > 0.0);
    
        let radius_f32x2 = f32x2::splat(radius);
        let min = pos - radius_f32x2;
        let max = pos + radius_f32x2;
        let aabb = Self {
            data: f32x4::from_array([min[0], min[1], max[0], max[1]])
        };
        
        debug_assert!(aabb.data[0] <= aabb.data[2] && aabb.data[1] <= aabb.data[3]);
        aabb
    }
}


#[cfg(test)]
mod tests {
    use std::simd::f32x2;

    use super::*;

    #[test]
    fn from_position_and_radius() {
        let aabb = AabbSimd::from_position_and_radius(f32x2::from_array([0.0, 0.0]), 1.0);

        assert_eq!(aabb.data[0], -1.0);
        assert_eq!(aabb.data[1], -1.0);

        assert_eq!(aabb.data[2], 1.0);
        assert_eq!(aabb.data[3], 1.0);
    }


}
