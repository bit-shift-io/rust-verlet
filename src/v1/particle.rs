use sdl2::pixels::Color;
use sdl2::gfx::primitives::DrawRenderer;
use cgmath::Vector2;

pub struct Particle {
    pub position_current: Vector2<f32>,
    pub position_old: Vector2<f32>,
    pub force: Vector2<f32>,
    pub radius: f32,
    pub mass: f32,
    pub color: Color,
}

impl Particle {
    pub fn new(position_current: Vector2<f32>, radius: f32, mass: f32, color: Color) -> Self {
        Self { position_current, position_old: position_current, radius, mass, color, force: Vector2::new(0f32, 0f32) }
    }

    pub fn draw(&self, canvas: &sdl2::render::Canvas<sdl2::video::Window>) {
        let pos_x = i16::try_from(self.position_current[0].round() as i32).unwrap();
        let pos_y = i16::try_from(self.position_current[1].round() as i32).unwrap();
        let r = i16::try_from(self.radius as i32).unwrap();
        canvas.filled_circle(pos_x, pos_y, r, self.color).unwrap();
    }

    pub fn update_position(&mut self, dt: f32) {
        let velocity: Vector2<f32> = self.position_current - self.position_old;
        let acceleration: Vector2<f32> = self.force / self.mass;
        self.position_old = self.position_current;
        self.position_current = self.position_current + velocity + acceleration * dt * dt;
    }

    pub fn accelerate(&mut self, acc: Vector2<f32>) {
        let force = acc * self.mass;
        self.force = force;
    }
}