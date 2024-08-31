use bevy::math::Vec2;

use super::shape_builder::PositionProvider;

pub struct Circle {
    centre: Vec2,
    radius: f32,
}

impl Circle {
    pub fn new(centre: Vec2, radius: f32) -> Self {
        Self { centre, radius }
    }
}

impl PositionProvider for Circle {
    fn get_points_for_radius(&self, radius: f32) -> Vec::<Vec2> {
        let mut points = vec![];

        // putting a smaller circle on the bigger circle, creates 2x isosceles triangles where they intersect
        // so solve to find the half angle
        // https://www.quora.com/How-do-you-find-the-angles-of-an-isosceles-triangle-given-three-sides
        let top = radius * radius; // particle radius ^ 2
        let bottom = 2.0 * self.radius * self.radius; // circle_radius ^ 2
        let c_angle = f32::acos(1.0 - (top / bottom)); // this is the half angle made by the isosceles trangle from the 2 intersecting circles
        let theta = c_angle * 2.0;
        
        let divisions = (2.0 * std::f32::consts::PI) / theta;
        let integer_divisions = divisions as usize;
        for i in 0..integer_divisions {
            let radians = i as f32 * theta;
            let x = f32::sin(radians);
            let y = f32::cos(radians);
            let pos = self.centre + Vec2::new(x * self.radius, y * self.radius);
            points.push(pos);
        }

        points
    }
}
