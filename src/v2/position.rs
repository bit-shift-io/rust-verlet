use super::types::Vec2;

pub trait Position {
    fn get_position(&self) -> Vec2;
    fn set_position(&mut self, pos: Vec2);
}