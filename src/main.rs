mod shapes;
mod cycle;

extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use cycle::*;
use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::{ RenderArgs, RenderEvent };
use glutin_window::GlutinWindow;
use opengl_graphics::{ GlGraphics, OpenGL };

use graphics::types::Color;

const BACK: Color = [ 0.078, 0.098, 0.161, 1.0 ];
const WHITE: Color = [ 1.0, 1.0, 1.0, 1.0 ];

struct App
{
    gl: GlGraphics,
    bg_color: Color,
    cycle: Epicycle,
    points: [[f64; 2]; 300],
    point_count: usize
}

impl App
{
    fn update(&mut self, time: f64)
    {
        self.cycle.update(time);
    }

    fn render(&mut self, arg: &RenderArgs)
    {
        self.gl.draw(arg.viewport(), |_c, gl| {
            graphics::clear(self.bg_color, gl);
        });

        self.cycle.render(&mut self.gl, arg, &mut self.points, &mut self.point_count);
    }
}

fn main() {
    let opengl = OpenGL::V3_2;
    let mut time: f64 = 0.0;

    let mut window: GlutinWindow = WindowSettings::new(
        "Fourier", [ 800, 600 ]
    ).graphics_api(opengl).exit_on_esc(true).samples(8).build().unwrap();

    let mut app = App
    {
        gl: GlGraphics::new(opengl),
        bg_color: BACK,
        cycle: Epicycle::new(3, 1.0, 200.0, 300.0, 100.0, 0.5, WHITE),
        points: [[0.0, 0.0]; 300],
        point_count: 0
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window)
    {
        if let Some(r) = e.render_args()
        {
            app.update(time);
            app.render(&r);

            time += 0.03;
        }
    }
}