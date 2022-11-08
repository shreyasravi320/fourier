use crate::shapes::*;

extern crate graphics;
extern crate opengl_graphics;

use piston::RenderArgs;
use opengl_graphics::GlGraphics;
use graphics::types::Color;

// const PI: f64 = 3.14159265358979323846264338327950288419716939937510;

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
    n: f64,
    circle: Circle,
    line: Line,
    child: Child
}

#[allow(dead_code)]
impl Epicycle
{
    pub fn new(num: i32, n: f64, x: f64, y: f64, rad: f64, border_rad: f64, color: Color) -> Self
    {
        let circle = Circle::new(x, y, rad / n, border_rad, color);
        let line = Line::new(x, y, rad / n, 0.0, color);
        return Epicycle{ n: n, circle: circle, line: line, 
            child: if num <= 1 { Child::Empty } else { Child::More(Box::new(Self::new(num - 1, n + 2.0, x + rad / n, y, rad, border_rad, color))) } };
    }

    pub fn update(&mut self, time: f64)
    {
        self.line.set_theta(self.n * time);

        match self.child
        {
            Child::Empty => (),
            Child::More(ref mut child) =>
            {
                child.circle.set_x(self.line.get_x() + self.line.get_len() * f64::cos(self.line.get_theta()));
                child.circle.set_y(self.line.get_y() + self.line.get_len() * f64::sin(self.line.get_theta()));

                child.line.set_x(child.circle.get_x());
                child.line.set_y(child.circle.get_y());

                child.update(time);
            }
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