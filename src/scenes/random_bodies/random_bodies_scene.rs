use std::{cell::RefCell, rc::Rc};

use cgmath::{InnerSpace, Vector2};
use sdl2::{event::Event, gfx::primitives::DrawRenderer, pixels::Color};
use rand::Rng;

use crate::{application::{Context, Scene}, v2::{body::Body, particle::Particle, position::Position, solver::Solver, stick::Stick}};

use crate::v3::{types::Vec2, particle_accelerator::ParticleAccelerator, particle_accelerator::ParticleCollider, particle_accelerator::ParticleRenderer, shape_builder::ShapeBuilder};

pub struct RandomBodiesScene {
    // v2
    pub solver: Solver,

    // v3
    pub particle_accelerator: ParticleAccelerator,
}

impl RandomBodiesScene {
    pub fn new() -> Self {
        // v3
        let mut particle_accelerator = ParticleAccelerator::new();

        let mask = 0x1;
        ShapeBuilder::new()
            .set_static(true)
            .add_line(Vec2::new(100.0f32, 800.0f32), Vec2::new(600.0f32, 800.0f32), 8.0f32)
            .create_in_particle_accelerator(&mut particle_accelerator, mask);
        
        // v2
        let mut solver = Solver::new();
        let ground_plane = Rc::new(RefCell::new(Body::create_line(Vector2::new(100.0f32, 800.0f32), Vector2::new(600.0f32, 800.0f32), 8.0f32)));
        ground_plane.borrow_mut().set_static(true);
        solver.add_body(&ground_plane);
        
        Self { solver, particle_accelerator }
    }
}

impl Scene for RandomBodiesScene {
    fn update(&mut self, context: &mut Context) {
        // v2
        for body in self.solver.bodies.iter() {
            body.borrow_mut().zero_forces();
            body.borrow_mut().add_gravity();
        }

        self.solver.update(0.0167f32);

        // v3
        let mut collider = ParticleCollider::new();
        collider.reset_forces(&mut self.particle_accelerator);

        let dt = 0.0167f32;
        const SUB_STEPS: usize = 16;
        for sub_dt in collider.range_substeps(dt, SUB_STEPS).iter() {
            collider.solve_collisions(&mut self.particle_accelerator);
            collider.update_positions(&mut self.particle_accelerator, *sub_dt);
            collider.update_constraints(&mut self.particle_accelerator);
        }
    }

    fn draw(&mut self, context: &mut Context) {
        context.sdl.canvas.set_draw_color(Color::RGB(128, 255, 255));
        context.sdl.canvas.clear();

        // v2
        self.solver.draw(context.sdl);

        // v3
        let renderer = ParticleRenderer::new();
        renderer.draw(&mut context.sdl, &self.particle_accelerator);

        context.sdl.canvas.present();
    }

    fn process_event(&mut self, context: &mut Context, event: Event) {
        match event {
            Event::MouseButtonDown { mouse_btn: sdl2::mouse::MouseButton::Left, x, y, .. } => {
                let xf = x as f32;
                let yf = y as f32;
                let mut rng = rand::thread_rng();

                let shape = 1;//rng.gen_range(0..=1);

                //let wheel_1 = Rc::new(RefCell::new(Body::create_stick_spoke_wheel(Vector2::new(xf, yf))));
                //self.solver.add_body(&wheel_1);

                // single particle
                if shape == 0 {
                    // v2
                    let mut body = Body::new();
                    let col = Color::RGB(rng.gen_range(0..=255), rng.gen_range(0..=255), rng.gen_range(0..=255));

                    let particle = Particle::new(Vector2::new(xf + 10f32, yf), 10f32, 1f32, col);
                    body.add_particle(&Rc::new(RefCell::new(particle)));

                    let particle_body = Rc::new(RefCell::new(body));
                    self.solver.add_body(&particle_body);


                    // v3
                    let mask = 0x1;
                    self.particle_accelerator.create_particle(Vec2::new(xf, yf), 10f32, 1f32, mask, col);
                }

                // chain of 2 particles
                if shape == 1 {
                    let radius = 10f32;

                    // v2
                    let mut body = Body::new();

                    let col = Color::RGB(rng.gen_range(0..=255), rng.gen_range(0..=255), rng.gen_range(0..=255));

                    //let particle_1 = 
                    let particle_1_handle = Rc::new(RefCell::new(Particle::new(Vector2::new(xf, yf), radius, 1f32, col)));
                    body.add_particle(&particle_1_handle);
                    let particle_1_dyn: Rc<RefCell<dyn Position>> = particle_1_handle;

                    //let particle_2: dyn Position = 
                    let particle_2_handle = Rc::new(RefCell::new(Particle::new(Vector2::new(xf + radius * 2f32, yf), radius, 1f32, col)));
                    body.add_particle(&particle_2_handle);
                    let particle_2_dyn: Rc<RefCell<dyn Position>> = particle_2_handle.clone();
                    
                    //let ph_test: Rc<RefCell<dyn Position>> = particle_1_handle;
                    let stick_handle = Rc::new(RefCell::new(Stick::new(&particle_1_dyn, &particle_2_dyn)));
                    body.add_stick(&stick_handle);

                    // now lets move particles to overlap
                    let p2clone = Rc::clone(&particle_2_handle);
                    p2clone.as_ref().borrow_mut().set_position(Vector2::new(xf + radius * 1f32, yf));
                    p2clone.as_ref().borrow_mut().pos_prev = Vector2::new(xf + radius * 1f32, yf);

                    let particle_body = Rc::new(RefCell::new(body));
                    self.solver.add_body(&particle_body);


                    // v3
                    let mask = 0x1;
                    ShapeBuilder::new()
                        .add_particle(Vec2::new(xf, yf), radius)
                        .add_particle(Vec2::new(xf + radius * 2.0, yf), radius)
                        .add_stick([-2, -1]) // -2 = second to last, -1 = last
                        .create_in_particle_accelerator(&mut self.particle_accelerator, mask);
        
                }
                /* 
                // chain of 3 circles
                if shape == 0 {
                    let radius = rng.gen_range(5..50) as f32;
                    let pos1 = Vector2::new(xf, yf);
                    let pos2 = Vector2::new(xf + radius, yf);
                    let pos3 = Vector2::new(xf - radius, yf);
                    let col = Color::RGB(rng.gen_range(0..=255), rng.gen_range(0..=255), rng.gen_range(0..=255));
                    let mass = radius;
                    let p1 = self.solver.add_particle(Particle::new(pos1, radius, mass, col));
                    let p2 = self.solver.add_particle(Particle::new(pos2, radius, mass, col));
                    let p3 = self.solver.add_particle(Particle::new(pos3, radius, mass, col));
                
                    let length = radius * 2f32;
                    self.solver.add_stick(Stick::new(length, p1, p2));
                    self.solver.add_stick(Stick::new(length, p1, p3));
                }

                // box
                if shape == 1 {
                    let radius = rng.gen_range(5..50) as f32;

                    let pos1 = Vector2::new(xf - radius, yf - radius);
                    let pos2 = Vector2::new(xf + radius, yf - radius);
                    let pos3 = Vector2::new(xf + radius, yf + radius);
                    let pos4 = Vector2::new(xf - radius, yf + radius);

                    let col = Color::RGB(rng.gen_range(0..=255), rng.gen_range(0..=255), rng.gen_range(0..=255));
                    let mass = radius;

                    let p1 = self.solver.add_particle(Particle::new(pos1, radius, mass, col));
                    let p2 = self.solver.add_particle(Particle::new(pos2, radius, mass, col));
                    let p3 = self.solver.add_particle(Particle::new(pos3, radius, mass, col));
                    let p4 = self.solver.add_particle(Particle::new(pos4, radius, mass, col));
                
                    //solver.add_stick(Stick::new((pos1 - pos2).magnitude(), p1, p2));
                    //solver.add_stick(Stick::new((pos2 - pos3).magnitude(), p2, p3));
                    //solver.add_stick(Stick::new((pos3 - pos4).magnitude(), p3, p4));
                    self.solver.add_stick(Stick::new((pos4 - pos1).magnitude(), p4, p1));


                    self.solver.add_stick(Stick::new((pos1 - pos3).magnitude(), p1, p3));
                    self.solver.add_stick(Stick::new((pos2 - pos4).magnitude(), p2, p4));
                }*/
            },
            _ => {}
        }
    }
}