mod constants;
mod shapes;
mod cycle;
mod fourier;

extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use constants::*;
use cycle::*;
use fourier::*;
use std::collections::VecDeque;
use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::{ RenderArgs, RenderEvent };
use glutin_window::GlutinWindow;
use opengl_graphics::{ GlGraphics, OpenGL };

use graphics::types::Color;


struct App
{
    gl: GlGraphics,
    bg_color: Color,
    cycle: Epicycle,
    points: VecDeque<Complex>
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

        self.cycle.render(&mut self.gl, arg, &mut self.points);
    }
}

fn main() {
    let opengl = OpenGL::V3_2;
    let mut time: f64 = 0.0;

    let mut window: GlutinWindow = WindowSettings::new(
        "Fourier", [ WIDTH, HEIGHT ]
    ).graphics_api(opengl).exit_on_esc(true).samples(8).build().unwrap();

    const NUM_POINTS: usize = 100;
    let mut points: Vec<Complex> = Vec::new();
    for i in 0..NUM_POINTS
    {
        points.push(Complex::new(100.0 * f64::cos(i as f64), 100.0 * f64::sin(i as f64)));
    }
    let transform: Vec<[f64; 3]> = dft(&mut points);

    let mut app = App
    {
        gl: GlGraphics::new(opengl),
        bg_color: BACK,
        cycle: Epicycle::from(0, CENTER_X, CENTER_Y, CIRCLE_THICKNESS, transform, WHITE),
        points: VecDeque::new()
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window)
    {
        if let Some(r) = e.render_args()
        {
            app.update(time);
            app.render(&r);

            time += 2.0 * PI / NUM_POINTS as f64;
        }
    }
}