/* 
use cgmath::Vector2;
use sdl2::rect::Point;

use crate::v2::types::Vec2;


pub trait PointExtension {
    fn from(v: &Vec2) -> Self;
}

impl PointExtension for Point {
    fn from(v: &Vec2) -> Self {
        let x = v[0].round() as i32;
        let y = v[1].round() as i32;
        Self::new(x, y)
    }
}*/


// External struct
use sdl2::rect::Point;

use crate::v2::types::Vec2;

// https://stackoverflow.com/questions/25413201/how-do-i-implement-a-trait-i-dont-own-for-a-type-i-dont-own
// Create a new type.
//pub struct PointWrapper(Point);


pub fn vec2_to_point(vec: Vec2) -> Point {
    let x = vec[0].round() as i32;
    let y = vec[1].round() as i32;
    return Point::new(x, y)
}
/* 
// Provide your own implementations
impl PointWrapper {
    pub fn new(x: i32, y: i32) -> Point {
        PointWrapper {
            raw: sys::SDL_Point {
                x: clamp_position(x),
                y: clamp_position(y),
            },
        }
    }

    pub fn new(vec: &Vec2) -> Self {
        let x = vec[0].round() as i32;
        let y = vec[1].round() as i32;
        return Self::new(x, y)
    }
}


impl From<&Vec2> for PointWrapper {
    fn from(vec: &Vec2) -> PointWrapper {
        PointWrapper::new(vec)
    }
}
*/