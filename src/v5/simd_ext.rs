use std::simd::{f32x1, f32x2, f32x4, num::SimdFloat};



pub trait f32x2Ext {

    fn length_squared(&self) -> f32;
    fn length_squared_1(&self) -> f32x1;

    fn length_squared_2_into_2(&self) -> f32x2;
}

impl f32x2Ext for f32x2 {
    fn length_squared(&self) -> f32 {
        // length_squared = a*a + b*b;
        let mul = self * self;
        //let add = mul[0] + mul[1];
        let add = mul.reduce_sum();
        add
    }

    fn length_squared_1(&self) -> f32x1 {
        // length_squared = a*a + b*b;
        let mul = self * self;
        //let add = mul[0] + mul[1];
        let add = f32x1::from_array([mul.reduce_sum()]);
        add
    }

    fn length_squared_2_into_2(&self) -> f32x2 {
        // length_squared = a*a + b*b;
        let mul = self * self;
        //let add = mul[0] + mul[1];
        let add = f32x2::splat(mul.reduce_sum());
        add
    }
}


pub trait f32x4Ext {

    fn length_squared_2_into_2(&self) -> f32x2;
    fn length_squared_2_into_4(&self) -> f32x4;
}

impl f32x4Ext for f32x4 {

    fn length_squared_2_into_2(&self) -> f32x2 {
        // length_squared = a*a + b*b;
        let mul = self * self;
        let sum_1 = mul[0] + mul[1];
        let sum_2 = mul[2] + mul[3];
        let add = f32x2::from_array([sum_1, sum_2]);
        add
    }

    fn length_squared_2_into_4(&self) -> f32x4 {
        // length_squared = a*a + b*b;
        let mul = self * self;
        let sum_1 = mul[0] + mul[1];
        let sum_2 = mul[2] + mul[3];
        let add = f32x4::from_array([sum_1, sum_1, sum_2, sum_2]);
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