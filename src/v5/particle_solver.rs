/// Compute which particle should move by how much if a and or b is static
#[inline(always)]
pub fn compute_movement_weight(a_is_static: bool, b_is_static: bool) -> (f32, f32) {
    // movement weight is used to stop static objects being moved
    let a_movement_weight = if a_is_static { 0.0f32 } else if b_is_static { 1.0f32 } else { 0.5f32 };
    let b_movement_weight = 1.0f32 - a_movement_weight;
    (a_movement_weight, b_movement_weight)
}