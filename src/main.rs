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
    gl: GlGraphics,
    circle: Circle
}

impl App
{
    fn render(&mut self, arg: &RenderArgs) {
        const BACK: [f32; 4] = [ 0.078, 0.098, 0.161, 1.0 ];

        self.gl.draw(arg.viewport(), |_c, gl| {
            graphics::clear(BACK, gl);
        });

        self.circle.render(&mut self.gl, arg);
    }
}

struct Circle
{
    x: i32,
    y: i32,
    rad: i32
}

impl Circle
{
    fn render(&mut self, gl: &mut GlGraphics, arg: &RenderArgs)
    {
        use graphics::*;

        const WHITE: [f32; 4] = [ 1.0, 1.0, 1.0, 0.7 ];
        let circle = ellipse::circle(self.x as f64, self.y as f64, self.rad as f64);

        gl.draw(arg.viewport(), |c, gl| {
            let transform = c.transform;

            Ellipse::new_border(WHITE, 0.5).draw(circle, &c.draw_state, transform, gl);
        });
    }
}

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: GlutinWindow = WindowSettings::new(
        "Fourier", [ 800, 600 ]
    ).graphics_api(opengl).exit_on_esc(true).samples(8).build().unwrap();

    let mut app = App
    {
        gl: GlGraphics::new(opengl),
        circle: Circle { x: 400, y: 300, rad: 100 }
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