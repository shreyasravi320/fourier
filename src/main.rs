extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::{ RenderArgs, RenderEvent };
use glutin_window::GlutinWindow;
use opengl_graphics::{ GlGraphics, OpenGL };

struct App
{
    gl: GlGraphics
}

impl App
{
    fn render(&mut self, arg: &RenderArgs) {
        const BACK: [f32; 4] = [ 0.078, 0.098, 0.161, 1.0 ];

        self.gl.draw(arg.viewport(), |_c, gl| {
            graphics::clear(BACK, gl);
        });
    }
}

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: GlutinWindow = WindowSettings::new(
        "Fourier", [ 800, 600 ]
    ).graphics_api(opengl).exit_on_esc(true).build().unwrap();

    let mut app = App
    {
        gl: GlGraphics::new(opengl)
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window)
    {
        if let Some(r) = e.render_args()
        {
            app.render(&r);
        }
    }
}
