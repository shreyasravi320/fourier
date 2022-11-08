extern crate graphics;
use graphics::types::Color;

// Constants
pub const BACK: Color = [ 0.078, 0.098, 0.161, 1.0 ];
pub const WHITE: Color = [ 1.0, 1.0, 1.0, 1.0 ];

pub const WIDTH: u32 = 800;
pub const HEIGHT: u32 = 600;
pub const CENTER_X: f64 = 200.0;
pub const CENTER_Y: f64 = 300.0;
pub const IMAGE_X: f64 = 500.0;

// POINTS = WIDTH - IMAGE_X
pub const POINTS: usize = WIDTH as usize - IMAGE_X as usize;
pub const RADIUS: f64 = 400.0;
pub const CIRCLE_THICKNESS: f64 = 0.15;
pub const LINE_THICKNESS: f64 = 0.5;

pub const TIME_DELTA: f64 = 0.03;
pub const PI: f64 = 3.14159265358979323846264338327950288419716939937510;
