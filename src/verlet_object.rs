use sdl2::pixels::Color;
use sdl2::gfx::primitives::DrawRenderer;
use cgmath::Vector2;

pub struct VerletObject {
    pub position_current: Vector2<f32>,
    pub position_old: Vector2<f32>,
    pub acceleration: Vector2<f32>,
    pub radius: i16,
    pub color: Color,
}

impl VerletObject {
    pub fn draw(&self, canvas: &sdl2::render::Canvas<sdl2::video::Window>) {
        let pos_x = i16::try_from(self.position_current[0].round() as i32).unwrap();
        let pos_y = i16::try_from(self.position_current[1].round() as i32).unwrap();
        canvas.filled_circle(pos_x, pos_y, self.radius, self.color).unwrap();
    }

    pub fn update_position(&mut self, dt: f32) {
        let velocity: Vector2<f32> = self.position_current - self.position_old;
        self.position_old = self.position_current;
        self.position_current = self.position_current + velocity + self.acceleration * dt * dt;
        self.acceleration = Vector2::new(0f32, 0f32);
    }

    pub fn accelerate(&mut self, acc: Vector2<f32>) {
        self.acceleration += acc;
    }
}