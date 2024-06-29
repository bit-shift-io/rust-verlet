use sdl2::{event::Event, mouse::MouseWheelDirection};

use crate::v3::types::Vec2;


pub struct Mouse {
    pub position_current_old: Vec2,
    pub position_old_old: Vec2,

    pub pos: Vec2,
    pub pos_prev: Vec2,

    pub cursor_size: f32,
    pub max_cursor_size: f32,
    pub min_cursor_size: f32,

    pub left_button_down: bool,
    pub right_button_down: bool,
}

impl Mouse {
    pub fn new() -> Self {
        let pos = Vec2::new(0f32, 0f32);
        Self { 
            position_current_old: pos, 
            position_old_old: pos, 

            pos: Vec2::zeros(),
            pos_prev: Vec2::zeros(),

            cursor_size: 20f32, 
            max_cursor_size: 100f32, 
            min_cursor_size: 20f32, 
            left_button_down: false, 
            right_button_down: false 
        }
    }

    pub fn increase_cursor_size(&mut self, increment: f32) {
        if self.cursor_size + increment > self.max_cursor_size || self.cursor_size + increment < self.min_cursor_size {
            return;
        }
        self.cursor_size += increment;
    }

    pub fn update_position(&mut self, x: i32, y: i32) {
        // legacy
        self.position_old_old = self.position_current_old;
        self.position_current_old.x = x as f32;
        self.position_current_old.y = y as f32;

        // new math lib
        self.pos_prev = self.pos;
        self.pos.x = x as f32;
        self.pos.y = y as f32;
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
            Event::MouseButtonUp { mouse_btn, .. } => {
                if self.left_button_down && mouse_btn == sdl2::mouse::MouseButton::Left {
                    self.left_button_down = false;
                }
                if self.right_button_down && mouse_btn == sdl2::mouse::MouseButton::Right {
                    self.right_button_down = false;
                }
            },
            Event::MouseMotion { x, y, .. } => {
                self.update_position(x, y);
            },
            Event::MouseWheel { direction, .. } => {
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
