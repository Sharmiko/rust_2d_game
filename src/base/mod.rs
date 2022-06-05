use ggez::Context;
use ggez::graphics::{self, *};


pub enum LocationType {
    Single(Rect),
    Multiple(Vec<Rect>)
}

pub trait ObjectLocation {
    fn get_location(&self, _ctx: &mut Context) -> LocationType;
}


pub struct CustomRect {
    pub fields: Rect
}


impl CustomRect {
    pub fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
        Self {
            fields: Rect{
                x: x,
                y: y,
                w: w,
                h: h
            }
        }
    }

    pub fn from_rect(rect: Rect) -> Self {
        Self {
            fields: rect
        }
    }

    pub fn draw(&mut self, ctx: &mut Context) {
        let mesh = graphics::MeshBuilder::new()
        .rectangle(
            graphics::DrawMode::stroke(3.), 
            graphics::Rect {
                x: 0.,
                y: 0.,
                w: self.fields.w,
                h: self.fields.h
            },
            graphics::Color::BLUE
        ).unwrap().build(ctx).unwrap();

        let draw_params = graphics::DrawParam::new()
            .dest(glam::Vec2::new(self.fields.x, self.fields.y));
        graphics::draw(ctx, &mesh, draw_params).unwrap();
    }
}