use cgmath::Vector2;
use sdl2::{event::Event, mouse::MouseWheelDirection};


pub struct Mouse {
    pub position_current: Vector2<f32>,
    pub position_old: Vector2<f32>,

    pub cursor_size: f32,
    pub max_cursor_size: f32,
    pub min_cursor_size: f32,

    pub left_button_down: bool,
    pub right_button_down: bool,
}

impl Mouse {
    pub fn new() -> Self {
        let pos = Vector2::new(0f32, 0f32);
        Self { position_current: pos, position_old: pos, cursor_size: 20f32, max_cursor_size: 100f32, min_cursor_size: 20f32, left_button_down: false, right_button_down: false }
    }

    pub fn increase_cursor_size(&mut self, increment: f32) {
        if self.cursor_size + increment > self.max_cursor_size || self.cursor_size + increment < self.min_cursor_size {
            return;
        }
        self.cursor_size += increment;
    }

    pub fn update_position(&mut self, x: i32, y: i32) {
        self.position_old = self.position_current;
        self.position_current.x = x as f32;
        self.position_current.y = y as f32;
    }

    pub fn process_event(&mut self, event: Event) {
        match event {
            Event::MouseButtonDown { mouse_btn, x, y, .. } => {
                self.update_position(x, y);

                if !self.left_button_down && mouse_btn == sdl2::mouse::MouseButton::Left {
                    self.left_button_down = true;
                }

                if !self.right_button_down && mouse_btn == sdl2::mouse::MouseButton::Right {
                    self.right_button_down = true;
                }
            },
            Event::MouseButtonUp { timestamp, window_id, which, mouse_btn, clicks, x, y } => {
                if self.left_button_down && mouse_btn == sdl2::mouse::MouseButton::Left {
                    self.left_button_down = false;
                }
                if self.right_button_down && mouse_btn == sdl2::mouse::MouseButton::Right {
                    self.right_button_down = false;
                }
            },
            Event::MouseMotion { timestamp, window_id, which, mousestate, x, y, xrel, yrel } => {
                self.update_position(x, y);
            },
            Event::MouseWheel { timestamp, window_id, which, x, y, direction, precise_x, precise_y } => {
                if direction == MouseWheelDirection::Normal {
                    self.increase_cursor_size(10f32);
                }
                if direction == MouseWheelDirection::Flipped {
                    self.increase_cursor_size(-10f32);
                }
            },
            _ => {}
        }
    }

    pub fn update(&mut self) {
    }
}
