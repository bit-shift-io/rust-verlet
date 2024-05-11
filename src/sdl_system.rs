use sdl2::gfx::primitives::DrawRenderer;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::Sdl;
use sdl2::video::WindowPos;
use sdl2::render::WindowCanvas;
//use sdl2::render::Texture;
use sdl2::render::TextureCreator;
//use sdl2::image::LoadTexture;
use sdl2::video::WindowContext;

pub struct SdlSystem {
    pub sdl_context: Sdl,
    //window: Window,
    pub canvas: WindowCanvas,
    pub texture_creator: TextureCreator<WindowContext>
}

impl SdlSystem {
    pub fn new(title: &str, width: u32, height: u32) -> SdlSystem {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let mut window = video_subsystem.window(title, width, height)
            //.position_centered()
            .build()
            .expect("could not initialize video subsystem");
        window.set_position(WindowPos::Positioned(0), WindowPos::Positioned(0)); // easy to debug

        let canvas = window.into_canvas().build()
            .expect("could not make a canvas");

        let texture_creator = canvas.texture_creator();
        //let texture = texture_creator.load_texture("assets/gradient_linear.png").unwrap();

        SdlSystem {
            sdl_context,
            //window,
            canvas,
            texture_creator
        }
    }

    /* 
    pub fn load_texture(&self, path: &str) -> Texture {
        //let texture_creator = self.canvas.texture_creator();
        //let texture = self.texture_creator.load_texture(path).unwrap();
        return self.texture_creator.load_texture(path).unwrap();
    }*/

    pub fn draw_line(&mut self, start: Point, end: Point, color: Color) {
        self.canvas.set_draw_color(color);
        self.canvas.draw_line(start, end).unwrap();
    }

    pub fn draw_filled_circle(&self, point: Point, radius: i32, color: Color) {
        // I'm getting a panic here, I think the system is exploding and
        // drawing stuf way off screen more than i16 can handle.
        // I should ignore if drawn off screen
        let (u_width, u_height) = self.canvas.output_size().unwrap(); 
        // ideally we use an AABB here, expand to include the circle radius
        //let aabb = Aabb2::<Point>::zero();

        let width = u_width as i32;
        let height = u_height as i32;

        // bounds checking. Discard circle if out of the window
        if point.x < -radius {
            return
        }
        if point.y < -radius {
            return
        }
        if (point.x - radius) > width {
            return
        }
        if (point.y - radius) > height {
            return
        }

        let pos_x = i16::try_from(point.x).unwrap();
        let pos_y = i16::try_from(point.y).unwrap();
        let r = i16::try_from(radius as i32).unwrap();
        self.canvas.filled_circle(pos_x, pos_y, r, color).unwrap();
    }

/*
    pub fn run_event_loop<F: Fn(f32)>(&mut self, update: F) {
        let mut event_pump = self.sdl_context.event_pump().unwrap();
/*

        let mut event_pump = sdl_context.event_pump()?;
        'running: loop {
            // Handle events
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit {..} |
                    Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        break 'running;
                    },
                    _ => {}
                }
            }

            // Update
            fluid_sim.update(0.001);

            // Render
            fluid_sim_renderer.draw();

            // Time management!
            //Duration::from_millis(1000)
            ::std::thread::sleep(Duration::from_millis(SLEEP_PER_FRAME_MS));
            //::std::thread::sleep(Duration::from::new(0, 1_000_000_000u32 / 60));
        }*/
    } */
}