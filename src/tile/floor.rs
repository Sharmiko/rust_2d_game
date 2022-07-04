use ggez::Context;
use ggez::graphics::{self, Image, Drawable, Rect, Canvas};

use glam::Vec2;

use crate::base::{LocationType, ObjectLocation};


pub struct Floor {
    left_corner: Image,
    middle: Image,
    right_corner: Image
}


impl ObjectLocation for Floor {
    fn get_location(&self, _ctx: &mut Context) -> LocationType {
        LocationType::Multiple(self.generate_location(_ctx))
    } 
}


impl Floor {
    pub fn new(ctx: &mut Context) -> Self {
        Self {
            left_corner: Image::from_path(ctx, "/tiles/park/tiles/Tile_01.png", true).unwrap(),
            middle: Image::from_path(ctx, "/tiles/park/tiles/Tile_02.png", true).unwrap(),
            right_corner: Image::from_path(ctx, "/tiles/park/tiles/Tile_02.png", true).unwrap()
        }
    }

    pub fn generate_location(&self, ctx: &mut Context) -> Vec<Rect> {
        let mut res = Vec::new();
        let (w, h) = ctx.gfx.size();
        res.push(Rect {
            x: 0.,
            y: h - self.left_corner.height() as f32 * 2.,
            w: self.left_corner.width() as f32,
            h: self.left_corner.height() as f32
        });

        let left_width = self.left_corner.width();
        let middle_height = self.middle.height();
        for i in 1..(w as u32 / self.left_corner.width() as u32 - 1) {
            res.push(Rect {
                x: (left_width * i) as f32,
                y: h - middle_height as f32 * 2.,
                w: self.middle.width() as f32,
                h: self.middle.height() as f32
            });
        }

        res.push(Rect {
            x: w - self.right_corner.width() as f32,
            y: h - self.right_corner.height() as f32 * 2.,
            w: self.right_corner.width() as f32,
            h: self.right_corner.height() as f32
        });

        return res;
    }

    pub fn draw(&self, ctx: &mut Context, canvas: &mut Canvas)  {
        let locations = self.generate_location(ctx);

        let mut params = graphics::DrawParam::default()
            .dest(Vec2::new(locations.first().unwrap().x, locations.first().unwrap().y));
        canvas.draw(&self.left_corner, params);

        for i in 1..(locations.len() - 1) {
            let params = graphics::DrawParam::default()
                .dest(Vec2::new(locations[i].x, locations[i].y));
            canvas.draw(&self.middle, params);
        }

        params = graphics::DrawParam::default()
            .dest(Vec2::new(locations.last().unwrap().x, locations.last().unwrap().y));
        canvas.draw(&self.right_corner, params);
    }   
}