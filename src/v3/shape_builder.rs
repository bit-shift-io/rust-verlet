use sdl2::pixels::Color;
use rand::Rng;

use super::{particle_accelerator::{ParticleAccelerator, ParticleHandle, SpringHandle, StickHandle}, types::Vec2};


pub struct ParticlePrim {
    pub pos: Vec2,
    pub radius: f32,
    pub mass: f32,
    pub is_static: bool,
    pub color: Color,
}

impl ParticlePrim {
    pub fn new(pos: Vec2, radius: f32, mass: f32, is_static: bool, color: Color) -> Self {
        Self { pos, radius, mass, is_static, color }
    }
}

struct SpringPrim {
    particle_indicies: [usize; 2],
    length: f32,
    spring_constant: f32,
    damping: f32,
    elastic_limit: f32,
}

impl SpringPrim {
    pub fn new(particle_indicies: [usize; 2], particle_positions: [Vec2; 2], spring_constant: f32, damping: f32, elastic_limit: f32) -> Self {
        let length = (particle_positions[1] - particle_positions[0]).magnitude();
        Self { particle_indicies, length, spring_constant, damping, elastic_limit }
    }
}

struct StickPrim {
    particle_indicies: [usize; 2],
    length: f32,
    stiffness_factor: f32,
}

impl StickPrim {
    pub fn new(particle_indicies: [usize; 2], particle_positions: [Vec2; 2], stiffness_factor: f32) -> Self {
        let length = (particle_positions[1] - particle_positions[0]).magnitude();
        Self { particle_indicies, length, stiffness_factor }
    }
}

pub struct ShapeBuilder {
    pub particles: Vec<ParticlePrim>,
    sticks: Vec<StickPrim>,
    springs: Vec<SpringPrim>,

    // particle properties
    is_static: bool,
    mass: f32,
    color: Color,
    radius: f32,
    stiffness_factor: f32,

    // spring properties
    spring_constant: f32,
    elastic_limit: f32,
    damping: f32,

    pub particle_handles: Vec<ParticleHandle>,
    pub stick_handles: Vec<StickHandle>,
    pub spring_handles: Vec<SpringHandle>,
}


fn convert_to_real_index(idx: i64, len: usize) -> usize {
    if idx >= 0 { idx as usize } else { (len as i64 + idx) as usize }
}

impl ShapeBuilder {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let color = Color::RGB(rng.gen_range(0..=255), rng.gen_range(0..=255), rng.gen_range(0..=255));

        Self { 
            particles: vec![], 
            sticks: vec![], 
            springs: vec![],
            is_static: false, 
            radius: 4.0,
            mass: 1.0,
            stiffness_factor: 0.0,
            particle_handles: vec![],
            stick_handles: vec![],
            spring_handles: vec![],
            color,
            spring_constant: 1.0,
            damping: 1.0,
            elastic_limit: -1.0,
        }    
    }

    pub fn set_color(&mut self, color: Color) -> &mut Self {
        self.color = color;
        self
    }

    pub fn set_radius(&mut self, radius: f32) -> &mut Self {
        self.radius = radius;
        self
    }

    pub fn set_random_color(&mut self) -> &mut Self {
        let mut rng = rand::thread_rng();
        let color = Color::RGB(rng.gen_range(0..=255), rng.gen_range(0..=255), rng.gen_range(0..=255));
        self.set_color(color)
    }

    pub fn set_static(&mut self, is_static: bool) -> &mut Self {
        self.is_static = is_static;
        self
    }

    pub fn set_mass(&mut self, mass: f32) -> &mut Self {
        self.mass = mass;
        self
    }

    pub fn set_stiffness_factor(&mut self, stiffness_factor: f32) -> &mut Self {
        self.stiffness_factor = stiffness_factor;
        self
    }

    pub fn set_spring_constant(&mut self, spring_constant: f32) -> &mut Self {
        self.spring_constant = spring_constant;
        self
    }

    pub fn set_damping(&mut self, damping: f32) -> &mut Self {
        self.damping = damping;
        self
    }

    pub fn set_elastic_limit(&mut self, elastic_limit: f32) -> &mut Self {
        self.elastic_limit = elastic_limit;
        self
    }

    pub fn create_in_particle_accelerator(&mut self, particle_accelerator: &mut ParticleAccelerator, mask: u32) -> &mut Self {
        let mut particle_handles = vec![];
        for particle in self.particles.iter() {
            let particle_handle = particle_accelerator.create_particle(particle.pos, particle.radius, particle.mass, mask, particle.color);
            particle_accelerator.set_particle_static(&particle_handle, particle.is_static);
            particle_handles.push(particle_handle);
        }

        let mut stick_handles = vec![];
        for stick in self.sticks.iter() {
            let stick_handle = particle_accelerator.create_stick([&particle_handles[stick.particle_indicies[0]], &particle_handles[stick.particle_indicies[1]]], stick.length, stick.stiffness_factor);
            stick_handles.push(stick_handle);
        }

        let mut spring_handles = vec![];
        for spring in self.springs.iter() {
            let spring_handle = particle_accelerator.create_spring([&particle_handles[spring.particle_indicies[0]], &particle_handles[spring.particle_indicies[1]]], spring.length, spring.spring_constant, spring.damping, spring.elastic_limit);
            spring_handles.push(spring_handle);
        }

        self.particle_handles = particle_handles;
        self.stick_handles = stick_handles;
        self.spring_handles = spring_handles;

        self
    }

    pub fn add_particle(&mut self, pos: Vec2, radius: f32) -> &mut Self {
        self.particles.push(ParticlePrim::new(pos, radius, self.mass, self.is_static, self.color));
        self
    }
    
    pub fn add_stick(&mut self, particle_indicies: [i64; 2]) -> &mut Self {
        let real_particle_indicies: [usize; 2] = [
            convert_to_real_index(particle_indicies[0], self.particles.len()),
            convert_to_real_index(particle_indicies[1], self.particles.len()),
        ];
        let particle_positions = [self.particles[real_particle_indicies[0]].pos, self.particles[real_particle_indicies[1]].pos];
        self.sticks.push(StickPrim::new(real_particle_indicies, particle_positions, self.stiffness_factor));

        //let combined_radius = self.particles[real_particle_indicies[0]].radius + self.particles[real_particle_indicies[1]].radius;
        //let last_stick = self.sticks.last().unwrap();
        //assert!(last_stick.length >= combined_radius, "Overlapping particles with sticks detected! System is unstable!");

        self
    }

    pub fn add_spring(&mut self, particle_indicies: [i64; 2]) -> &mut Self {
        let real_particle_indicies: [usize; 2] = [
            convert_to_real_index(particle_indicies[0], self.particles.len()),
            convert_to_real_index(particle_indicies[1], self.particles.len()),
        ];
        let particle_positions = [self.particles[real_particle_indicies[0]].pos, self.particles[real_particle_indicies[1]].pos];
        self.springs.push(SpringPrim::new(real_particle_indicies, particle_positions, self.spring_constant, self.damping, self.elastic_limit));

        let combined_radius = self.particles[real_particle_indicies[0]].radius + self.particles[real_particle_indicies[1]].radius;
        let last_spring = self.springs.last().unwrap();
        assert!(last_spring.length >= combined_radius, "Overlapping particles with springs detected! System is unstable!");

        self
    }

    pub fn remove_first_particle(&mut self) -> &mut Self {
        self.particles.remove(0);
        self
    }

    pub fn add_line(&mut self, p1: Vec2, p2: Vec2, radius: f32) -> &mut Self {
        let particle_mass = 1.0f32;

        let dist = (p2 - p1).magnitude();
        let divisions = (dist / ((radius * 2.0) + f32::EPSILON)) as usize;
        let delta = (p2 - p1);

        //let col = Color::RGB(rng.gen_range(0..=255), rng.gen_range(0..=255), rng.gen_range(0..=255));
        for i in 0..divisions { 
            let percent = i as f32 / divisions as f32;
            let pos = p1 + (delta * percent);
            self.particles.push(ParticlePrim::new(pos, radius, particle_mass, self.is_static, self.color));
        }

        self
    }

    /*
    // create a parallelogram with two sides defined by:
    // 1: p2 - p1
    // 2: p3 - p2
    pub fn add_parallelogram(&mut self, p1: Vec2, p2: Vec2, p3: Vec2) -> &mut Self {
        let divisions1 = radius_divisions_between_points(p1, p2, self.radius);
        let delta1 = p2 - p1;

        let divisions2 = radius_divisions_between_points(p2, p3, self.radius);
        let delta2 = p3 - p2;

        for i1 in 0..divisions1 { 
            let percent1 = i1 as f32 / divisions1 as f32;
            let pos1 = p1 + (delta1 * percent1);

            for i2 in 0..divisions2 { 
                let percent2 = i2 as f32 / divisions2 as f32;
                let pos2 = pos1 + (delta2 * percent2);

                self.particles.push(ParticlePrim::new(pos2, self.radius, self.mass, self.is_static, self.color));
            }
        }

        self
    }

    // Connect a grid of particles with sticks in a cross + grid pattern
    pub fn connect_with_cross_grid_of_sticks(&mut self, stride: usize) -> &mut Self {
        let particle_count = self.particles.len();
        let grid_height = stride;
        let grid_width = particle_count / stride;

        for x in 0..grid_width {
            for y in 0..grid_height {
                let cur_particle_idx = (y * grid_height + x) as i64;

                // add adjacent particle to the right
                {
                    let particle_indicies = [
                        cur_particle_idx as i64,
                        (cur_particle_idx + 1) as i64,
                    ];
                    self.add_stick(particle_indicies);
                }

                // add adjacent particle below
                {
                    let particle_indicies = [
                        cur_particle_idx as i64,
                        cur_particle_idx + stride as i64,
                    ];
                    self.add_stick(particle_indicies);
                }

                // todo: add cross
            } 
        }

        self
    }*/

    // think of spokes on a circle, to help gives wheels rigidity
    pub fn connect_adjacent_sticks(&mut self, particle_stride: usize, stick_stride: i64) -> &mut Self {
        let particle_count = self.particles.len() as i64;
        
        for i in (0..particle_count).step_by(particle_stride) {
            let k = if (i + stick_stride) >= particle_count { i + stick_stride - particle_count } else { i + stick_stride };

            //println!("i: {} -> {}", i, k);
            
            let particle_indicies = [
                i,
                k
            ];
            self.add_stick(particle_indicies);    
            
        }
        
        self
    }

    pub fn add_circle(&mut self, circle_origin: Vec2, circle_radius: f32, particle_radius: f32, divisions: i64) -> &mut Self {
        for i in 0..divisions {  
            let percent = i as f32 / divisions as f32;
            let radians = percent * 2f32 * std::f32::consts::PI;
            let x = f32::sin(radians);
            let y = f32::cos(radians);
            let pos = circle_origin + Vec2::new(x * circle_radius, y * circle_radius);
    
            self.particles.push(ParticlePrim::new(pos, particle_radius, self.mass, self.is_static, self.color));     
        }

        self
    }

    pub fn add_adjacent_stick_circle(&mut self, circle_origin: Vec2, circle_radius: f32, particle_radius: f32, divisions: i64) -> &mut Self {
        self.add_circle(circle_origin, circle_radius, particle_radius, divisions);

        // add adjacent sticks
        for i in 0..divisions {
            let particle_indicies = [
                i,
                if (i + 1) == divisions { 0 } else { i + 1 }
            ];
            self.add_stick(particle_indicies);    
        }

        /* 
        // try opposite sticks?
        let half_divisions = divisions / 2;
        for i in 0..half_divisions {
            let particle_indicies = [
                i,
                if (i + 1) == half_divisions { 0 } else { i + half_divisions }
            ];
            self.add_stick(particle_indicies);    
        }*/

        self

    }

    pub fn add_cloth_grid(&mut self, width: i32, height: i32, spacing: f32, origin: Vec2) -> &mut Self {
        let particle_radius = 4.0;

        for y in 0..=height {
            for x in 0..=width {
                let is_static = if y == 0 && x % 2 == 0 { true } else { false };
                let pos = Vec2::new((origin[0] + x as f32 * spacing) as f32, (origin[1] + y as f32 * spacing) as f32);
              
                self.particles.push(ParticlePrim::new(pos, particle_radius, self.mass, is_static, self.color));

                if x != 0 {
                    let particle_indicies: [i64; 2] = [
                        -2,
                        -1
                    ];
                    self.add_stick(particle_indicies); 
                }
              
                if y != 0 {
                    let up_point = (x + (y - 1) * (width + 1)) as i64;
                    let particle_indicies: [i64; 2] = [
                        up_point,
                        -1
                    ];
                    self.add_stick(particle_indicies); 
                }
            }
          }

          self
    }


    pub fn add_grid(&mut self, width: i32, height: i32, spacing: f32, origin: Vec2) -> &mut Self {
        for y in 0..=height {
            for x in 0..=width {
                let is_static = false; //if y == 0 && x % 2 == 0 { true } else { false };
                let pos = Vec2::new((origin[0] + x as f32 * spacing) as f32, (origin[1] + y as f32 * spacing) as f32);
              
                self.particles.push(ParticlePrim::new(pos, self.radius, self.mass, is_static, self.color));
            }
        }

        self
    }

    // connect every 'particles_per_line' into a line to make more of a 
    // 'thick' liquid
    pub fn connect_with_stick_chain(&mut self, particles_per_line: usize) -> &mut Self {
        let particle_count = self.particles.len() as i64;

        for i in (0..particle_count).step_by(particles_per_line) {
            for k in 0..(particles_per_line - 1) {
                let particle_indicies = [
                    i + k as i64,
                    i + k as i64 + 1
                ];
                self.add_stick(particle_indicies);
            }
        }

        self
    }

    pub fn add_stick_grid(&mut self, width: i32, height: i32, spacing: f32, origin: Vec2) -> &mut Self {
        for y in 0..=height {
            for x in 0..=width {
                let is_static = false; //if y == 0 && x % 2 == 0 { true } else { false };
                let pos = Vec2::new((origin[0] + x as f32 * spacing) as f32, (origin[1] + y as f32 * spacing) as f32);
              
                self.particles.push(ParticlePrim::new(pos, self.radius, self.mass, is_static, self.color));

                if x != 0 {
                    // horizonal spring
                    let particle_indicies: [i64; 2] = [
                        -2,
                        -1
                    ];
                    self.add_stick(particle_indicies);
                }
              
                if y != 0 {
                    // vertical spring
                    let up_point = (x + (y - 1) * (width + 1)) as i64;
                    let particle_indicies: [i64; 2] = [
                        up_point,
                        -1
                    ];
                    self.add_stick(particle_indicies); 

                    
                    // cross spring (bottom left to top right)
                    if x < width {
                        let particle_indicies: [i64; 2] = [
                            up_point + 1,
                            -1
                        ];
                        self.add_stick(particle_indicies); 
                    }

                    
                    // cross spring (bottom right to top left)
                    if x > 0 {
                        let particle_indicies: [i64; 2] = [
                            up_point - 1,
                            -1
                        ];
                        self.add_stick(particle_indicies); 
                    }
                }
            }
        }

        self
    }

    pub fn add_hanging_particle(&mut self, origin: Vec2, hanging_origin: Vec2) -> &mut Self {
        // add a static particle at origin
        self.particles.push(ParticlePrim::new(origin, self.radius, self.mass, true, self.color));

        // add a handing particle underneath
        self.particles.push(ParticlePrim::new(hanging_origin, self.radius, self.mass, false, self.color));

        // connecting spring
        let particle_indicies: [i64; 2] = [
            -2,
            -1
        ];
        self.add_spring(particle_indicies);

        self
    }

}


// Utility function that takes 2 points (a line segment) and a radius
// and calculates how many circles can fit touching each other between the 2 points.
pub fn radius_divisions_between_points(p1: Vec2, p2: Vec2, radius: f32) -> usize {
    let dist = (p2 - p1).magnitude();
    let divisions = (dist / (radius * 2.0)) as usize;
    return divisions;
}