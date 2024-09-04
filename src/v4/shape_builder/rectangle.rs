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

        let min = self.rect.min;
        let x_max = self.rect.min + vec2(self.rect.width(), 0.0);
        let y_max = self.rect.min + vec2(0.0, self.rect.height());

        let x_divisions = radius_divisions_between_points(min, x_max, radius);
        let y_divisions = radius_divisions_between_points(min, y_max, radius);

        let x_delta = x_max - min;
        let y_delta = y_max - min;

        for yi in 0..y_divisions { 
            let y_percent = yi as f32 / y_divisions as f32;
            let y_offset = y_delta * y_percent;

            for xi in 0..x_divisions { 
                let x_percent = xi as f32 / x_divisions as f32;
                let x_offset = x_delta * x_percent;

                let pos = min + y_offset + x_offset;
                points.push(pos);
            }
        }

        points
    }
}