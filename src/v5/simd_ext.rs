use std::simd::f32x2;



pub trait f32x2Ext {

    fn length_squared(&self) -> f32;
}

impl f32x2Ext for f32x2 {
    fn length_squared(&self) -> f32 {
        panic!("TODO! f32x2Ext length_squared is not implemented");
        0.0 // todo:
    }
}