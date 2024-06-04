pub mod orbital;

extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;
use piston::MouseCursorEvent;

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    rotation: f64, // rotation for the square.
    scale: f64, // size of the square.
    mouse: [f64; 2], // Location of mouse.
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let square = rectangle::square(0.0, 0.0, 50.0);
        let rotation = self.rotation;
        let scale = self.scale.cos().abs();
        let (x, y) = (args.window_size[0] / 2.0, args.window_size[1] / 2.0);

        self.gl.draw(args.viewport(), |c, gl| {
            // clear the screen
            clear(GREEN, gl);
            let st1 = c
                .transform
                .trans(x, y)
                .rot_rad(rotation)
                .scale(scale, scale)
                .trans(-25.0, -25.0);
            let st2 = c
                .transform
                .trans(x, y)
                .rot_rad(2.0*rotation)
                .scale(scale, scale)
                .trans(-25.0, -25.0);

            // draw bock rotating around the middle of the screen.
            rectangle(RED, square, st1, gl);
            rectangle(color::YELLOW, square, st2, gl);

            let circle_tf = c
                .transform
                .trans(self.mouse[0], self.mouse[1])
                .trans(-5.0, -5.0);
            circle_arc(color::BLUE, 10.0, 0.0, 7.0, rectangle::square(0.0, 0.0, 10.0), circle_tf, gl);
            
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        // Rotate 2 radians per second.
        self.rotation += 2.0 * args.dt;
        self.scale += args.dt;
    }
    
    fn mouse_move(&mut self, args: &[f64; 2]) {
        self.mouse = [args[0], args[1]];
    }
}

fn main() {
    // change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // create Glutin window.
    let mut window: Window = WindowSettings::new("spinning square", [1280, 720])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        rotation: 0.0,
        scale: 1.0,
        mouse: [0.0, 0.0]
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }
        if let Some(args) = e.update_args() {
            app.update(&args);
        }
        if let Some(args) = e.mouse_cursor_args() {
            app.mouse_move(&args);
        }
    }
}
