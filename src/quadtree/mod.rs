use ggez::Context;
use ggez::graphics::Rect;
use ggez::graphics::{self, *};

use crate::consts::CHAR_WIDTH;


pub struct QuadTree {
    pub boundary: Rect,
    pub points: Vec<Rect>,
    pub top_left: Option<Box<QuadTree>>,
    pub top_right: Option<Box<QuadTree>>,
    pub bottom_left: Option<Box<QuadTree>>,
    pub bottom_right: Option<Box<QuadTree>>
}

impl QuadTree {

    const LIMIT: f32 = CHAR_WIDTH * 2.5;

    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            boundary: Rect {
                x: x,
                y: y,
                w: width,
                h: height
            },
            points: Vec::new(),
            top_left: None,
            top_right: None,
            bottom_left: None,
            bottom_right: None
        }
    }

    pub fn insert(&mut self, x: f32, y: f32, data: Rect) {
        if !self._in_boundary(x, y) {
            return;
        }

        if self._reach_limit() {
            self.points.push(data);
            return;
        }

        if (self.boundary.x + self.boundary.w) / 2. >= x {

            // top left tree
            if (self.boundary.y + self.boundary.h) / 2. >= y {
                if self.top_left.is_none() {
                    self.top_left = Some(Box::new(QuadTree::new(
                        self.boundary.x,
                        self.boundary.y,
                        (self.boundary.x + self.boundary.w) / 2.,
                        (self.boundary.y + self.boundary.h) / 2.
                    )));
                }
                
                self.top_left.as_mut().unwrap().insert(x, y, data);
            } 
            // bottom left tree
            else {
                if self.bottom_left.is_none() {
                    self.bottom_left = Some(Box::new(QuadTree::new(
                        self.boundary.x,
                        (self.boundary.y + self.boundary.h) / 2.,
                        (self.boundary.x + self.boundary.w) / 2.,
                        self.boundary.h
                    )));
                }
                
                self.bottom_left.as_mut().unwrap().insert(x, y, data);
            }
        } else {
            // top right tree
            if (self.boundary.y + self.boundary.h) / 2. >= y {

                if self.top_right.is_none() {
                    self.top_right = Some(Box::new(QuadTree::new(
                        (self.boundary.x + self.boundary.w ) / 2.,
                        self.boundary.y,
                        self.boundary.w,
                        (self.boundary.y + self.boundary.h) / 2.
                    )));
                }

                self.top_right.as_mut().unwrap().insert(x, y, data);
            }
            // bottom right tree
            else {

                if self.bottom_right.is_none() {
                    self.bottom_right = Some(Box::new(QuadTree::new(
                        (self.boundary.x + self.boundary.w) / 2.,
                        (self.boundary.y + self.boundary.h) / 2.,
                        self.boundary.w,
                        self.boundary.h
                    )));
                }

                self.bottom_right.as_mut().unwrap().insert(x, y, data);
            }
        }
    }

    pub fn draw_boundries(&self, ctx: &mut Context, canvas: &mut Canvas, color: graphics::Color) {
        let rect = graphics::Rect{
            x: self.boundary.x,
            y: self.boundary.y,
            w: self.boundary.w,
            h: self.boundary.h
        };

        let mesh = graphics::Mesh::new_rectangle(
            &ctx.gfx, 
            graphics::DrawMode::stroke(3.), 
            graphics::Rect {
                x: 0.,
                y: 0.,
                w: rect.w,
                h: rect.h
            },
            color
        ).unwrap();

        let draw_params = graphics::DrawParam::new()
            .dest(glam::Vec2::new(self.boundary.x, self.boundary.y));

        canvas.draw(&mesh, draw_params);

        if !self.top_left.is_none() {
            self.top_left.as_ref().unwrap().draw_boundries(ctx, canvas, graphics::Color::YELLOW);
        }

        if !self.top_right.is_none() {
            self.top_right.as_ref().unwrap().draw_boundries(ctx, canvas, graphics::Color::RED);
        }

        if !self.bottom_left.is_none() {
            self.bottom_left.as_ref().unwrap().draw_boundries(ctx, canvas, graphics::Color::GREEN);
        }

        if !self.bottom_right.is_none() {
            self.bottom_right.as_ref().unwrap().draw_boundries(ctx, canvas, graphics::Color::MAGENTA);
        }
    }

    pub fn search(&self, x: f32, y: f32) -> Option<&Vec<Rect>> {
        if !self._in_boundary(x, y) {
            return None;
        }

        if self._reach_limit() {
            return Some(&self.points);
        }

        if (self.boundary.x + self.boundary.w) / 2. >= x {

            // top left
            if (self.boundary.y + self.boundary.h) / 2. >= y {
                if self.top_left.is_none() {
                    return None;
                }

                return self.top_left.as_ref().unwrap().search(x, y);
            } 
            // bottom left
            else {
                if self.bottom_left.is_none() {
                    return None;
                }

                return self.bottom_left.as_ref().unwrap().search(x, y);
            }
        } else {

            // top right tree
            if (self.boundary.y + self.boundary.h) / 2. >= y {
                if self.top_right.is_none() {
                    return None;
                }

                return self.top_right.as_ref().unwrap().search(x, y);
            } 
            // bottom right tree
            else {
                if self.bottom_right.is_none() {
                    return None;
                }

                return self.bottom_right.as_ref().unwrap().search(x, y);
            }
        }

    }

    fn _reach_limit(&self) -> bool {
        (self.boundary.x - self.boundary.w).abs() <= QuadTree::LIMIT * 1.5 &&
        (self.boundary.y - self.boundary.y).abs() <= QuadTree::LIMIT * 1.5
    }

    fn _in_boundary(&self, x: f32, y: f32) -> bool {
        x >= self.boundary.x &&
        x <= self.boundary.w &&
        y >= self.boundary.y && 
        y <= self.boundary.h
    }
}