use crate::constants::*;
use crate::shapes::*;

extern crate graphics;
extern crate opengl_graphics;

use std::collections::VecDeque;
use piston::RenderArgs;
use opengl_graphics::GlGraphics;
use graphics::types::Color;

fn add_point(points: &mut VecDeque<f64>, point: f64)
{
    if points.len() == 0
    {
        points.push_front(point);
    }
    else if points.len() == POINTS
    {
        points.pop_back();
        points.push_front(point);
    }
    else
    {
        points.push_front(point);
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
        let circle = Circle::new(x, y, rad / (n * PI), border_rad, color);
        let line = Line::new(x, y, rad / (n * PI), 0.0, color);
        return Epicycle{ n: n, circle: circle, line: line, 
            child: if num <= 1 { Child::Empty } else { Child::More(Box::new(Self::new(num - 1, n + 2.0, x + rad / (n * PI), y, rad, border_rad, color))) } };
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

    pub fn render(&mut self, gl: &mut GlGraphics, arg: &RenderArgs, points: &mut VecDeque<f64>)
    {
        self.circle.render(gl, arg);
        self.line.render(gl, arg);
        
        match self.child
        {
            Child::Empty =>
            {
                use graphics::*;

                let line = Line::new(self.circle.get_color(), LINE_THICKNESS);
                let edge_x: f64 = self.line.get_x() + self.line.get_len() * f64::cos(self.line.get_theta());
                let edge_y: f64 = self.line.get_y() + self.line.get_len() * f64::sin(self.line.get_theta());

                add_point(points, edge_y);

                gl.draw(arg.viewport(), |c, gl| {
                    line.draw_from_to([edge_x, edge_y], [IMAGE_X, edge_y], &c.draw_state, c.transform, gl);
                    for i in 0..(points.len() - 1)
                    {
                        line.draw_from_to([IMAGE_X + i as f64, points[i]], [IMAGE_X + i as f64 + 1.0, points[i + 1]], &c.draw_state, c.transform, gl);
                    }
                });
            },
            Child::More(ref mut child) => child.render(gl, arg, points)
        }
    }
}