use crate::constants::*;
use crate::shapes::*;
use crate::fourier::*;

extern crate graphics;
extern crate opengl_graphics;

use std::collections::VecDeque;
use piston::RenderArgs;
use opengl_graphics::GlGraphics;
use graphics::types::Color;

#[allow(dead_code)]
pub enum Child
{
    Empty,
    More(Box<Epicycle>)
}

pub struct Epicycle
{
    freq: f64,
    phase: f64,
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
        return Epicycle{ freq: n, phase: 0.0, circle: circle, line: line, 
            child: if num <= 1 { Child::Empty } else { Child::More(Box::new(Self::new(num - 1, n + 2.0, x + rad / (n * PI), y, rad, border_rad, color))) } };
    }

    pub fn from(i: usize, x: f64, y: f64, border_rad: f64, transform: Vec<[f64; 3]>, color: Color) -> Self
    {
        // [freq, amp, phase]
        let circle = Circle::new(x, y, transform[i][1], border_rad, color);
        let line = Line::new(x, y, transform[i][1], 0.0, color);
        return Epicycle{ freq: transform[i][0], phase: transform[i][2], circle: circle, line: line,
            child: if i < transform.len() - 1 { Child::More(Box::new(Self::from(i + 1, x, y, border_rad, transform, color))) } else { Child::Empty } };
    }

    pub fn update(&mut self, time: f64)
    {
        self.line.set_theta(self.freq * time + self.phase);

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

    pub fn render(&mut self, gl: &mut GlGraphics, arg: &RenderArgs, points: &mut VecDeque<[f64; 2]>)
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

                println!("[ {} {} {} {} ]", self.line.get_x(), self.line.get_y(), edge_x, edge_y);

                // points.push_back([edge_x, edge_y]);

                gl.draw(arg.viewport(), |c, gl| {
                    for i in 0..(points.len())
                    {
                        // line.draw_from_to([points[i][0], points[i][1]], [points[i + 1][0], points[i + 1][1]], &c.draw_state, c.transform, gl);
                        line.draw_from_to([points[i][0] + CENTER_X, points[i][1] + CENTER_Y], [points[(i + 1) % points.len()][0] + CENTER_X, points[(i + 1) % points.len()][1] + CENTER_Y], &c.draw_state, c.transform, gl);
                    }
                });
            },
            Child::More(ref mut child) => child.render(gl, arg, points)
        }
    }
}