use crate::shapes::*;

extern crate graphics;
extern crate opengl_graphics;

use piston::RenderArgs;
use opengl_graphics::GlGraphics;

fn add_point(points: &mut [[f64; 2]; 300], point: [f64; 2], point_count: usize)
{
    if point_count == 0
    {
        points[0] = point;
    }
    else if point_count == 300
    {
        for i in (0..(point_count - 1)).rev()
        {
            points[i + 1] = points[i];
            points[i + 1][0] += 1.0;
        }
        points[0] = point;
    }
    else
    {
        for i in (0..point_count).rev()
        {
            points[i + 1] = points[i];
            points[i + 1][0] += 1.0;
        }
        points[0] = point;
    }
}

#[allow(dead_code)]
pub enum Child
{
    Empty,
    More(Box<Epicycle>)
}

pub struct Epicycle
{
    circle: Circle,
    line: Line,
    child: Child
}

#[allow(dead_code)]
impl Epicycle
{
    pub fn new(mut circle: Circle, child: Child) -> Self
    {
        let line = Line::new(circle.get_x(), circle.get_y(), circle.get_rad(), 0.0, circle.get_color());
        return Epicycle{ circle: circle, line: line, child: child };
    }

    pub fn update(&mut self, time: f64)
    {
        self.line.set_theta(time);

        match self.child
        {
            Child::Empty => (),
            Child::More(ref mut child) => child.update(time)
        }
    }

    pub fn render(&mut self, gl: &mut GlGraphics, arg: &RenderArgs, points: &mut [[f64; 2]; 300], point_count: &mut usize)
    {
        self.circle.render(gl, arg);
        self.line.render(gl, arg);
        
        match self.child
        {
            Child::Empty =>
            {
                use graphics::*;

                let line = Line::new(self.circle.get_color(), 0.5);
                let edge_x: f64 = self.line.get_x() + self.line.get_len() * f64::cos(self.line.get_theta());
                let edge_y: f64 = self.line.get_y() + self.line.get_len() * f64::sin(self.line.get_theta());

                add_point(points, [500.0, edge_y], *point_count);
                if *point_count < 300
                {
                    *point_count += 1;
                }

                gl.draw(arg.viewport(), |c, gl| {
                    line.draw_from_to([edge_x, edge_y], [500.0, edge_y], &c.draw_state, c.transform, gl);
                    for i in 0..(*point_count - 1)
                    {
                        line.draw_from_to(points[i], points[i + 1], &c.draw_state, c.transform, gl);
                    }
                });
            },
            Child::More(ref mut child) => child.render(gl, arg, points, point_count)
        }
    }
}