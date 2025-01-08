use std::simd::{f32x2, num::SimdFloat};



pub trait f32x2Ext {

    fn length_squared(&self) -> f32;
}

impl f32x2Ext for f32x2 {
    fn length_squared(&self) -> f32 {
        // length_squared = a*a + b*b;
        let mul = self * self;
        //let add = mul[0] + mul[1];
        let add = mul.reduce_sum();
        add
    }
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn length_squared() {
        let a = f32x2::from_array([1.0, 1.0]);
        let len = a.length_squared();
        assert_eq!(len, 2.0);
    }
}