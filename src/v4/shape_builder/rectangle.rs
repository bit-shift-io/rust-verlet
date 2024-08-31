use bevy::math::{vec2, Rect, Vec2};

use super::shape_builder::{radius_divisions_between_points, PositionProvider};


pub struct Rectangle {
    rect: Rect
}

impl Rectangle {
    pub fn from_center_size(centre: Vec2, size: Vec2) -> Self {
        Self { rect: Rect::from_center_size(centre, size) }
    }
}

impl PositionProvider for Rectangle {
    fn get_points_for_radius(&self, radius: f32) -> Vec::<Vec2> {
        let mut points = vec![];

        let x_divisions = radius_divisions_between_points(self.rect.min, self.rect.min + vec2(self.rect.width(), 0.0), radius);
        let y_divisions = radius_divisions_between_points(self.rect.min, self.rect.min + vec2(0.0, self.rect.height()), radius);
        /* 
        let delta = self.p2 - self.p1;

        for i in 0..divisions { 
            let percent = i as f32 / divisions as f32;
            let pos = self.p1 + (delta * percent);
            points.push(pos);
        }*/

        points
    }
}