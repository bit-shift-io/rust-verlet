// https://pikuma.com/blog/verlet-integration-2d-cloth-physics-simulation

use cgmath::{InnerSpace, Vector2};
use sdl2::pixels::Color;
use sdl2::rect::Point;
use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;
use std::ptr::{self, null};
use std::rc::{Rc, Weak};

use crate::particle::Particle;
use crate::sdl_system::SdlSystem;
use crate::stick::Stick;

pub struct CMouse {
    /* 
    Vec2 pos;
    Vec2 prevPos;

    float cursorSize = 20;
    float maxCursorSize = 100;
    float minCursorSize = 20;

    bool leftButtonDown = false;
    bool rightButtonDown = false;
    */
}

pub struct CPoint {
    pub sticks: [Weak<RefCell<CStick>>; 2],
    pub position_current: Vector2<f32>,
    pub position_old: Vector2<f32>,
    pub position_init: Vector2<f32>,
    pub is_pinned: bool,
    pub is_selected: bool,
}

impl CPoint {
    pub fn new(position_current: Vector2<f32>) -> Self {
        Self { position_current, position_old: position_current, position_init: position_current, is_pinned: false, is_selected: false, sticks: [Weak::<RefCell<CStick>>::new(), Weak::<RefCell<CStick>>::new()] }
    }

    pub fn add_stick(&mut self, stick: Weak<RefCell<CStick>>, idx: usize) {
        self.sticks[idx] = stick;
    }

    pub fn set_pinned(&mut self, is_pinned: bool) {
        self.is_pinned  = is_pinned;
    }

    pub fn update(&mut self, dt: f32, cloth: &Cloth, mouse: &CMouse, window_width: i32, window_height: i32) {
        let drag = cloth.drag;
        let acceleration = cloth.gravity;
        let elasticity = cloth.elasticity;

        /* 
        Vec2 mouseDir = pos - mouse->GetPosition();
        float mouseDist = sqrtf(mouseDir.x * mouseDir.x + mouseDir.y * mouseDir.y);
        isSelected = mouseDist < mouse->GetCursorSize();
      
        for (Stick* stick : sticks) {
          if (stick != nullptr) {
            stick->SetIsSelected(isSelected);
          }
        }
      
        if (mouse->GetLeftButtonDown() && isSelected) {
          Vec2 difference = mouse->GetPosition() - mouse->GetPreviousPosition();
          if (difference.x > elasticity) difference.x = elasticity;
          if (difference.y > elasticity) difference.y = elasticity;
          if (difference.x < -elasticity) difference.x = -elasticity;
          if (difference.y < -elasticity) difference.y = -elasticity;
          prevPos = pos - difference;
        }
      
        if (mouse->GetRightMouseButton() && isSelected) {
          for (Stick* stick : sticks) {
            if (stick != nullptr) {
              stick->Break();
            }
          }
        }
      */
        if self.is_pinned {
          self.position_current = self.position_init;
          return;
        }
      
        let velocity = self.position_current - self.position_old;
        let new_pos = self.position_current + velocity * (1.0f32 - drag) + acceleration * (1.0f32 - drag) * dt * dt;
        self.position_old = self.position_current;
        self.position_current = new_pos;
      
        self.keep_inside_view(window_width, window_height);
    }

    fn keep_inside_view(&mut self, window_width: i32, window_height: i32) {
        if self.position_current.y >= (window_height as f32) {
            self.position_current.y = window_height as f32;
        }
        if self.position_current.x >= (window_width as f32) {
            self.position_current.x = window_width as f32;
        }
        if self.position_current.y < 0f32 {
            self.position_current.y = 0f32;
        }
        if self.position_current.x < 0f32 {
            self.position_current.x = 0f32;
        }
      }

}

pub struct CStick {
    pub points: [Weak<RefCell<CPoint>>; 2],
    pub length: f32,
    pub is_active: bool,
    pub is_selected: bool,
}

impl CStick {
    pub fn new(p0: Weak<RefCell<CPoint>>, p1: Weak<RefCell<CPoint>>, length: f32) -> Self {
        Self { length, is_active: true, is_selected: false, points: [p0, p1] }
    }

    pub fn update(&mut self, dt: f32) {
        if !self.is_active {
            return;
        }

        let p00 = self.points[0].upgrade().unwrap();
        let p11 = self.points[1].upgrade().unwrap();

        let mut p0 = p00.as_ref().borrow_mut();
        let mut p1 = p11.as_ref().borrow_mut();

        let difference = p0.position_current - p1.position_current;
        let diff_length = difference.magnitude();
        let diff_factor = (self.length - diff_length) / diff_length * 0.5;
        let offset = difference * diff_factor;

        p0.position_current += offset;
        p1.position_current -= offset;
    }

    pub fn draw(&self, sdl: &mut SdlSystem) {
        if !self.is_active {
            return;
        }

        let p00 = self.points[0].upgrade().unwrap();
        let p11 = self.points[1].upgrade().unwrap();

        let p0 = p00.as_ref().borrow();
        let p1 = p11.as_ref().borrow();

        let p0_pos = p0.position_current;
        let p1_pos = p1.position_current;

        sdl.canvas.set_draw_color(Color::RGB(255, 0, 0));
        let _ = sdl.canvas.draw_line(Point::new(p0_pos.x as i32, p0_pos.y as i32), Point::new(p1_pos.x as i32, p1_pos.y as i32));

        //sdl.canvas.DrawLine(p0Pos.x, p0Pos.y, p1Pos.x, p1Pos.y, isSelected ? colorWhenSelected : color);

    }
}

pub struct Cloth {
    pub gravity: Vector2<f32>,
    pub drag: f32,
    pub elasticity: f32,
    pub points: Vec<Rc<RefCell<CPoint>>>,
    pub sticks: Vec<Rc<RefCell<CStick>>>
}

impl Cloth {
    pub fn new(width: i32, height: i32, spacing: i32, start_x: i32, start_y: i32) -> Self {
        let mut s = Self { gravity: Vector2::new(0f32, 1000f32), drag: 0.01f32, elasticity: 10.0f32, points: vec![], sticks: vec![] };
        s.construct(width, height, spacing, start_x, start_y);
        s
    }

    fn construct(&mut self, width: i32, height: i32, spacing: i32, start_x: i32, start_y: i32) {
        for y in 0..=height {
            for x in 0..=width {
                let pos = Vector2::new((start_x + x * spacing) as f32, (start_y + y * spacing) as f32);
                let point = Rc::new(RefCell::new(CPoint::new(pos)));
              
                if x != 0 {
                    let left_point = &self.points[self.points.len() - 1];
                    let s = Rc::new(RefCell::new(CStick::new(Rc::downgrade(&point), Rc::downgrade(&left_point), spacing as f32)));
                    left_point.as_ref().borrow_mut().add_stick(Rc::downgrade(&s), 0);
                    point.as_ref().borrow_mut().add_stick(Rc::downgrade(&s), 0);
                    self.sticks.push(s);
                }
              
                if y != 0 {
                    let up_point = &self.points[(x + (y - 1) * (width + 1)) as usize];
                    let s = Rc::new(RefCell::new(CStick::new(Rc::downgrade(&point), Rc::downgrade(&up_point), spacing as f32)));
                    up_point.as_ref().borrow_mut().add_stick(Rc::downgrade(&s), 1);
                    point.as_ref().borrow_mut().add_stick(Rc::downgrade(&s), 1);
                    self.sticks.push(s);
                }
                
                if y == 0 && x % 2 == 0 {
                    point.as_ref().borrow_mut().set_pinned(true);
                }
                
                self.points.push(point);
            }
          }
    }


    pub fn update(&mut self, dt: f32) {

        let mouse = CMouse {};

        const SUB_STEPS: u32 = 16;
        //let sub_dt: f32 = dt / SUB_STEPS as f32;
        let sub_dt = dt;

        //for _ in 0..SUB_STEPS {
            for point in self.points.iter() {
                // todo: fix hard coded window width and height
                point.as_ref().borrow_mut().update(sub_dt, self, &mouse, 1200, 800);
            }

            for stick in self.sticks.iter() {
                stick.as_ref().borrow_mut().update(sub_dt);
            }
        //}
    }

    pub fn draw(&self, sdl: &mut SdlSystem) {
        for stick in self.sticks.iter() {
            stick.as_ref().borrow_mut().draw(sdl);
        }
    }
}