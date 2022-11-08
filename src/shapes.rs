extern crate graphics;
extern crate opengl_graphics;

use piston::RenderArgs;
use opengl_graphics::GlGraphics;
use graphics::types::Color;

pub struct Line
{
    x: f64,
    y: f64,
    len: f64,
    theta: f64,
    color: Color
}

#[allow(dead_code)]
impl Line
{
    pub fn new(x: f64, y: f64, len: f64, theta: f64, color: Color) -> Self
    {
        return Line{ x: x, y: y, len: len, theta: theta, color: color };
    }

    pub fn set_x(&mut self, x: f64)
    {
        self.x = x;
    }

    pub fn get_x(&mut self) -> f64
    {
        return self.x;
    }

    pub fn set_y(&mut self, y: f64)
    {
        self.y = y;
    }

    pub fn get_y(&mut self) -> f64
    {
        return self.y;
    }

    pub fn set_len(&mut self, len: f64)
    {
        self.len = len;
    }

    pub fn get_len(&mut self) -> f64
    {
        return self.len;
    }

    pub fn set_theta(&mut self, theta: f64)
    {
        self.theta = theta;
    }

    pub fn get_theta(&mut self) -> f64
    {
        return self.theta;
    }

    pub fn render(&mut self, gl: &mut GlGraphics, arg: &RenderArgs)
    {
        use graphics::*;
        let line = Line::new(self.color, 0.5);

        gl.draw(arg.viewport(), |c, gl| {
            line.draw_from_to([self.x, self.y], [self.x + self.len * f64::cos(self.theta), self.y + self.len * f64::sin(self.theta)], &c.draw_state, c.transform, gl);
        });
    }
}

pub struct Circle
{
    x: f64,
    y: f64,
    rad: f64,
    border_rad: f64,
    color: Color
}

#[allow(dead_code)]
impl Circle
{
    pub fn new(x: f64, y: f64, rad: f64, border_rad: f64, color: Color) -> Self
    {
        return Circle{ x: x, y: y, rad: rad, border_rad: border_rad, color: color };
    }

    pub fn set_x(&mut self, x: f64)
    {
        self.x = x;
    }
    
    pub fn get_x(&mut self) -> f64
    {
        return self.x;
    }

    pub fn set_y(&mut self, y: f64)
    {
        self.y = y;
    }

    pub fn get_y(&mut self) -> f64
    {
        return self.y;
    }

    pub fn set_rad(&mut self, rad: f64)
    {
        self.rad = rad;
    }

    pub fn get_rad(&mut self) -> f64
    {
        return self.rad;
    }

    pub fn get_color(&mut self) -> Color
    {
        return self.color;
    }

    pub fn render(&mut self, gl: &mut GlGraphics, arg: &RenderArgs)
    {
        use graphics::*;

        let circle = ellipse::circle(self.x, self.y, self.rad);

        gl.draw(arg.viewport(), |c, gl| {
            Ellipse::new_border(self.color, self.border_rad).draw(circle, &c.draw_state, c.transform, gl);
        });
    }
}